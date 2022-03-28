import * as wasm from "./pkg";
import { Range } from "monaco-editor";
import { showErrorToast } from "../toaster";
import Anser from "anser";

type RtEasyWasm = typeof import("./pkg");

export async function load(): Promise<RtEasy> {
  const rtEasyWasm = await import("./pkg");
  rtEasyWasm.setPanicHook();
  return new RtEasy(rtEasyWasm);
}

export class RtEasy {
  private rtEasyWasm: RtEasyWasm;

  constructor(rtEasyWasm: RtEasyWasm) {
    this.rtEasyWasm = rtEasyWasm;
  }

  check(code: string): CompilerResult<string> {
    try {
      this.rtEasyWasm.check(code);
      return {
        tag: "Ok",
        value: errorToHtml("\u001b[32mCode is syntactically valid.\u001b[0m"),
      };
    } catch (e) {
      return { tag: "Error", error_html: errorToHtml(e as string) };
    }
  }

  build(code: string, onChange?: () => void): CompilerResult<Simulator> {
    try {
      const simulatorWasm = this.rtEasyWasm.build(code);
      return { tag: "Ok", value: new Simulator(simulatorWasm, code, onChange) };
    } catch (e) {
      return { tag: "Error", error_html: errorToHtml(e as string) };
    }
  }

  buildVhdl(code: string): CompilerResult<Vhdl> {
    try {
      const vhdlWasm = this.rtEasyWasm.build_vhdl(code);
      return { tag: "Ok", value: new Vhdl(vhdlWasm) };
    } catch (e) {
      return { tag: "Error", error_html: errorToHtml(e as string) };
    }
  }

  buildSignals(code: string): CompilerResult<Signals> {
    try {
      const signalsWasm = this.rtEasyWasm.build_signals(code);
      const signals: Signals = {
        conditionSignals: signalsWasm.condition_signals(),
        controlSignals: signalsWasm.control_signals(),
      };
      signalsWasm.free();
      return { tag: "Ok", value: signals };
    } catch (e) {
      return { tag: "Error", error_html: errorToHtml(e as string) };
    }
  }
}

export type CompilerResult<T> =
  | { tag: "Ok"; value: T }
  | { tag: "Error"; error_html: string };

export class Simulator {
  private simulatorWasm: wasm.Simulator;
  private sourceCode: string;
  private simState: SimState | null;
  private runTimer: NodeJS.Timeout | null;

  private onChange: () => void;

  constructor(
    simulatorWasm: wasm.Simulator,
    sourceCode: string,
    onChange?: () => void
  ) {
    this.simulatorWasm = simulatorWasm;
    this.sourceCode = sourceCode;
    this.simState = null;
    this.runTimer = null;

    this.onChange = onChange ?? (() => {});
  }

  free = (): void => {
    if (this.runTimer !== null) clearInterval(this.runTimer);
    this.simulatorWasm.free();
  };

  reset = (): void => {
    this.stop();
    this.simulatorWasm.reset();
    this.simState = null;
    this.onChange();
  };
  cycleCount = (): number => this.simulatorWasm.cycle_count();
  isFinished = (): boolean =>
    (this.simulatorWasm.is_finished() && this.simState === null) ||
    (this.simState?.isAtAssertError ?? false);
  getSimState = (): SimState | null => this.simState;

  statementRange = (statement: number): Range | null => {
    const span = this.simulatorWasm.statement_span(statement);
    if (span === undefined) return null;
    const range = calcRange(this.sourceCode, span);
    span.free();
    return range;
  };
  addBreakpoint = (statement: number): void => {
    this.simulatorWasm.add_breakpoint(statement);
    this.onChange();
  };
  removeBreakpoint = (statement: number): void => {
    this.simulatorWasm.remove_breakpoint(statement);
    this.onChange();
  };
  breakpoints = (): number[] => Array.from(this.simulatorWasm.breakpoints());

  microStep = (): void => {
    this.simState = simulatorStep({
      simulator: this.simulatorWasm,
      currSimState: this.simState,
      micro: true,
      stopOnBreakpoint: false,
      sourceCode: this.sourceCode,
    });
    this.onChange();
  };
  step = (): void => {
    this.simState = simulatorStep({
      simulator: this.simulatorWasm,
      currSimState: this.simState,
      micro: false,
      stopOnBreakpoint: false,
      sourceCode: this.sourceCode,
    });
    this.onChange();
  };

  run = (intervalMsOrMax: number | "Max"): void => {
    if (this.isRunning() || this.isFinished()) return;
    this.runTimer = setInterval(
      () => {
        // Run
        if (intervalMsOrMax === "Max") {
          // Run for MS ms
          const MS = 5;
          let start = performance.now();
          while (true) {
            this.simState = simulatorStep({
              simulator: this.simulatorWasm,
              currSimState: this.simState,
              micro: false,
              stopOnBreakpoint: true,
              sourceCode: this.sourceCode,
            });

            if (this.simState?.isAtBreakpoint) break;
            if (performance.now() - start > MS) break;
          }
        } else {
          // Run one step
          this.simState = simulatorStep({
            simulator: this.simulatorWasm,
            currSimState: this.simState,
            micro: false,
            stopOnBreakpoint: true,
            sourceCode: this.sourceCode,
          });
        }

        // Check finished or breakpoint
        if (this.isFinished() || this.simState?.isAtBreakpoint) {
          this.stop();
          return;
        }

        // Call on changed
        this.onChange();
      },
      intervalMsOrMax === "Max" ? 10 : intervalMsOrMax
    );
    this.onChange();
  };
  stop = (): void => {
    if (this.runTimer === null) return;
    clearInterval(this.runTimer);
    this.runTimer = null;
    this.onChange();
  };
  toggleRun = (intervalMsOrMax: number | "Max"): void => {
    this.isRunning() ? this.stop() : this.run(intervalMsOrMax);
  };
  isRunning = (): boolean => this.runTimer !== null;

  registers = (kind: "Intern" | "Output"): string[] =>
    this.simulatorWasm.registers(kind);
  registerValue = (name: string, base: Base): string =>
    this.simulatorWasm.register_value(name, base);
  registerValueNext = (name: string, base: Base): string | null =>
    this.simulatorWasm.register_value_next(name, base) ?? null;
  writeRegister = (name: string, value: string, base: Base): void => {
    try {
      this.simulatorWasm.write_register(name, value, base);
      this.onChange();
    } catch (e) {
      showErrorToast({ message: e as string });
    }
  };

  buses = (kind: "Intern" | "Input"): string[] =>
    this.simulatorWasm.buses(kind);
  busValue = (name: string, base: Base): string =>
    this.simulatorWasm.bus_value(name, base);
  writeBus = (name: string, value: string, base: Base): void => {
    try {
      this.simulatorWasm.write_bus(name, value, base);
      this.onChange();
    } catch (e) {
      showErrorToast({ message: e as string });
    }
  };

  registerArrays = (): string[] => this.simulatorWasm.register_arrays();
  registerArrayValueNext = (
    name: string,
    base: Base
  ): { idx: number; value: string } | null => {
    const raw = this.simulatorWasm.register_array_value_next(name, base);
    if (raw === undefined) return null;
    return { idx: raw[0], value: raw[1] };
  };
  registerArrayPageCount = (name: string): number =>
    this.simulatorWasm.register_array_page_count(name);
  registerArrayPage = (
    name: string,
    pageNr: number,
    base: Base
  ): { idx: number; value: string }[] => {
    // Page returned from wasm is in the form:
    // [idx, value, idx, value, ...]
    const pageRaw = this.simulatorWasm.register_array_page(name, pageNr, base);

    // Map to [{idx, value}, ...] form
    let page: { idx: number; value: string }[] = [];
    for (let i = 0; i < pageRaw.length; i += 2) {
      page.push({ idx: pageRaw[i], value: pageRaw[i + 1] });
    }

    return page;
  };
  writeRegisterArray = (
    name: string,
    idx: number,
    value: string,
    base: Base
  ): void => {
    try {
      this.simulatorWasm.write_register_array(name, idx, value, base);
      this.onChange();
    } catch (e) {
      showErrorToast({ message: e as string });
    }
  };

  memories = (): string[] => this.simulatorWasm.memories();
  memoryValueNext = (
    name: string,
    base: Base
  ): { address: string; value: string } | null => {
    const raw = this.simulatorWasm.memory_value_next(name, base);
    if (raw === undefined) return null;
    return { address: raw[0], value: raw[1] };
  };
  memoryPageCount = (name: string): string =>
    this.simulatorWasm.memory_page_count(name);
  memoryPagePrev = (name: string, pageNr: string): string | null =>
    this.simulatorWasm.memory_page_prev(name, pageNr) ?? null;
  memoryPageNext = (name: string, pageNr: string): string | null =>
    this.simulatorWasm.memory_page_next(name, pageNr) ?? null;
  memoryPageNrOfAddress = (name: string, address: string): string | null =>
    this.simulatorWasm.memory_page_nr_of_address(name, address) ?? null;
  memoryPage = (
    name: string,
    pageNr: string,
    base: Base
  ): { address: string; value: string }[] => {
    // Page returned from wasm is in the form:
    // [addr, value, addr, value, ...]
    const pageRaw = this.simulatorWasm.memory_page(name, pageNr, base);

    // Map to [{addr, value}, ...] form
    let page: { address: string; value: string }[] = [];
    for (let i = 0; i < pageRaw.length; i += 2) {
      page.push({ address: pageRaw[i], value: pageRaw[i + 1] });
    }

    return page;
  };
  writeMemory = (
    name: string,
    address: string,
    value: string,
    base: Base
  ): void => {
    try {
      this.simulatorWasm.write_memory(name, address, value, base);
      this.onChange();
    } catch (e) {
      showErrorToast({ message: e as string });
    }
  };
  memorySave = (name: string): string => this.simulatorWasm.save_memory(name);
  memoryLoadFromSave = (name: string, save: string): void => {
    try {
      this.simulatorWasm.load_memory_from_save(name, save);
      this.onChange();
    } catch (e) {
      showErrorToast({ message: e as string });
    }
  };
}

export type RenderResult<T> =
  | { tag: "Ok"; value: T }
  | { tag: "Error"; error: string };

export class Vhdl {
  private vhdlWasm: wasm.Vhdl;

  constructor(vhdlWasm: wasm.Vhdl) {
    this.vhdlWasm = vhdlWasm;
  }

  free = (): void => {
    this.vhdlWasm.free();
  };

  memories = (): string[] => this.vhdlWasm.memories();
  render = (
    moduleName: string,
    memories: { name: string; file: string }[]
  ): RenderResult<string> => {
    try {
      let memoriesArg: string[] = [];
      for (const mem of memories) {
        memoriesArg.push(mem.name);
        memoriesArg.push(mem.file);
      }
      return {
        tag: "Ok",
        value: this.vhdlWasm.render(moduleName, memoriesArg),
      };
    } catch (e) {
      return { tag: "Error", error: e as string };
    }
  };
}

export const baseValues = ["BIN", "DEC", "HEX"] as const;
export type Base = typeof baseValues[number];
export const isBase = (x: any): x is Base => baseValues.includes(x);

export interface Signals {
  conditionSignals: String[];
  controlSignals: String[];
}

export interface SimState {
  statement: number;
  span: Range;

  markerCurrent: SimStateMarker | null;
  marker: SimStateMarker[];

  isAtBreakpoint: boolean;
  isAtAssertError: boolean;
  isStatementEnd: boolean;

  changed: Changed | null;
}

export interface SimStateMarker {
  kind: "True" | "False" | "Breakpoint" | "AssertError";
  span: Range;
}

export interface Changed {
  registers: Set<string>; // Name
  registerArrays: Map<string, number>; // Name -> idx
  memories: Map<string, string>; // Name -> address
}

// ------------ HELPER ------------

function errorToHtml(error: string): string {
  // Ansi to html
  error = Anser.ansiToHtml(Anser.escapeForHtml(error));

  // Error links
  error = error.replaceAll(
    /\[E(\d+)\]/g,
    '<a href="/rteasy-online/book/compiler-error-index/errors.html#e$1" target="_blank" style="text-decoration: underline;">$&</a>'
  );

  return error;
}

function calcRange(sourceCode: string, span: wasm.Span): Range {
  let startLineNumber = 1;
  let startColumn = 1;
  let endLineNumber = 1;
  let endColumn = 1;

  for (let i = 0; i < sourceCode.length && i < span.end; i++) {
    if (sourceCode.charAt(i) === "\n") {
      if (i < span.start) {
        startLineNumber++;
        startColumn = 1;
        endColumn = 1;
      } else {
        endColumn = 1;
      }
      endLineNumber++;
    } else {
      if (i < span.start) startColumn++;
      endColumn++;
    }
  }

  return new Range(startLineNumber, startColumn, endLineNumber, endColumn);
}

interface SimulatorStepParams {
  simulator: wasm.Simulator;
  currSimState: SimState | null;
  micro: boolean;
  stopOnBreakpoint: boolean;
  sourceCode: string;
}

function simulatorStep({
  simulator,
  currSimState,
  micro,
  stopOnBreakpoint,
  sourceCode,
}: SimulatorStepParams): SimState | null {
  // Return current sim state if at assert error
  if (currSimState?.isAtAssertError) return currSimState;

  // Step
  const stepResultWasm = micro
    ? simulator.micro_step(stopOnBreakpoint) ?? null
    : simulator.step(stopOnBreakpoint) ?? null;
  if (stepResultWasm === null) return null;

  // Map step result
  let stepResult = calcStepResult(stepResultWasm, sourceCode);
  stepResultWasm.free();

  // Marker
  let markerCurrent: SimStateMarker | null;
  switch (stepResult.kind.tag) {
    case "Condition":
      markerCurrent = {
        kind: stepResult.kind.result ? "True" : "False",
        span: stepResult.kind.span,
      };
      break;
    case "Breakpoint":
      markerCurrent = {
        kind: "Breakpoint",
        span: stepResult.span,
      };
      break;
    case "AssertError":
      markerCurrent = {
        kind: "AssertError",
        span: stepResult.span,
      };
      break;
    case "Void":
    case "Pipe":
    case "StatementEnd":
      markerCurrent = null;
      break;
  }

  // Next sim state
  const nextSimState: SimState = {
    statement: stepResult.statement,
    span: stepResult.span,

    markerCurrent,
    marker: currSimState?.isStatementEnd
      ? []
      : currSimState?.markerCurrent
      ? [...currSimState.marker, currSimState.markerCurrent]
      : currSimState?.marker ?? [],

    isAtBreakpoint: stepResult.kind.tag === "Breakpoint",
    isAtAssertError: stepResult.kind.tag === "AssertError",
    isStatementEnd: stepResult.kind.tag === "StatementEnd",

    changed:
      stepResult.kind.tag === "Pipe" || stepResult.kind.tag === "StatementEnd"
        ? stepResult.kind.changed
        : null,
  };

  return nextSimState;
}

interface StepResult {
  statement: number;
  span: Range;
  kind: StepResultKind;
}

type StepResultKind =
  | { tag: "Void" }
  | { tag: "Condition"; result: boolean; span: Range }
  | { tag: "Pipe"; changed: Changed }
  | { tag: "StatementEnd"; changed: Changed }
  | { tag: "Breakpoint" }
  | { tag: "AssertError" };

function calcStepResult(
  stepResultWasm: wasm.StepResult,
  sourceCode: string
): StepResult {
  let kind: StepResultKind;

  if (stepResultWasm.is_void()) {
    kind = { tag: "Void" };
  } else if (stepResultWasm.is_condition()) {
    const cond = stepResultWasm.as_condition()!;
    kind = {
      tag: "Condition",
      result: cond.result,
      span: calcRange(sourceCode, cond.span),
    };
    cond.free();
  } else if (stepResultWasm.is_pipe()) {
    kind = { tag: "Pipe", changed: calcChanged(stepResultWasm) };
  } else if (stepResultWasm.is_statement_end()) {
    kind = { tag: "StatementEnd", changed: calcChanged(stepResultWasm) };
  } else if (stepResultWasm.is_breakpoint()) {
    kind = { tag: "Breakpoint" };
  } else if (stepResultWasm.is_assert_error()) {
    kind = { tag: "AssertError" };
  } else {
    throw new Error("unexpected step result");
  }

  return {
    statement: stepResultWasm.statement,
    span: calcRange(sourceCode, stepResultWasm.span),
    kind,
  };
}

function calcChanged(stepResultWasm: wasm.StepResult): Changed {
  const registersWasm = stepResultWasm.changed_registers();
  const registers = new Set<string>();
  registersWasm.forEach((e) => registers.add(e));

  const registerArraysWasm = stepResultWasm.changed_register_arrays();
  const registerArrays = new Map<string, number>();
  for (let i = 0; i < registerArraysWasm.length; i += 2) {
    registerArrays.set(registerArraysWasm[i], registerArraysWasm[i + 1]);
  }

  const memoriesWasm = stepResultWasm.changed_memories();
  const memories = new Map<string, string>();
  for (let i = 0; i < memoriesWasm.length; i += 2) {
    memories.set(memoriesWasm[i], memoriesWasm[i + 1]);
  }

  return { registers, registerArrays, memories };
}
