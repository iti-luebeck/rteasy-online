import React from "react";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../model";
import { State, StateEdit } from "../state";

export function model(
  rtEasy: RtEasy,
  state: StateEdit,
  setState: React.Dispatch<React.SetStateAction<State>>
): GlobalModelEdit {
  return {
    tag: "Edit",
    sourceCode: state.sourceCode,
    base: state.base,
    setBase: (base) => setState({ ...state, base }),
    log: state.log,
    setSourceCode: (sourceCode) => {
      let log: string;
      try {
        rtEasy.check(sourceCode);
        log = "--- ok ---";
      } catch (e) {
        log = e;
      }

      localStorage.setItem("source-code", sourceCode);
      setState({ ...state, sourceCode, log });
    },
    build: () => {
      try {
        const simulator = rtEasy.build(state.sourceCode);
        setState({
          tag: "Run",
          sourceCode: state.sourceCode,
          base: state.base,
          simulator,
          currSpan: null,
          timerId: null,
        });
      } catch (e) {
        alert(e);
      }
    },
  };
}
