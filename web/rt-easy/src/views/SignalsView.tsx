import React, { useContext, useMemo } from "react";
import { HTMLTable } from "@blueprintjs/core";

import { CompilerResult, Signals } from "../wasm";
import { RtEasyContext } from "../wasm/context";
import { GlobalContext } from "../global/context";

interface Props {}

const StateView: React.FC<Props> = () => {
  const rtEasy = useContext(RtEasyContext);
  const globalModel = useContext(GlobalContext);
  const sourceCode = globalModel.editorModel.getValue();
  const signals = useMemo<CompilerResult<Signals>>(() => {
    return globalModel.tag === "Run"
      ? rtEasy.buildSignals(sourceCode)
      : {
          tag: "Ok",
          value: { conditionSignals: [], controlSignals: [] },
        };
  }, [rtEasy, globalModel.tag, sourceCode]);

  return (
    <div style={{ padding: "16px 8px" /*, overflow: "hidden"*/ }}>
      {signals.tag === "Ok" ? (
        signalsTable(signals.value)
      ) : (
        <pre
          dangerouslySetInnerHTML={{
            __html: signals.error_html,
          }}
          style={{ margin: 0 }}
        ></pre>
      )}
    </div>
  );
};

export default StateView;

function signalsTable(signals: Signals): React.ReactNode {
  const headerRow = (title: string) => (
    <tr style={{ backgroundColor: "#f2f2f2" }}>
      <td colSpan={2}>{title}:</td>
    </tr>
  );
  const dividerRow = (
    <tr key="dividerRow">
      <td colSpan={2}>&nbsp;</td>
    </tr>
  );

  return (
    <HTMLTable width="100%" bordered condensed>
      <thead></thead>
      <tbody>
        {headerRow("Condition Signals")}
        {signals.conditionSignals.map((value, idx) => (
          <tr key={idx}>
            <td>{"k" + idx}</td>
            <td style={{ fontFamily: "monospace" }}>{value}</td>
          </tr>
        ))}
        {dividerRow}
        {headerRow("Control Signals")}
        {signals.controlSignals.map((value, idx) => (
          <tr key={idx}>
            <td>{"c" + idx}</td>
            <td style={{ fontFamily: "monospace" }}>{value}</td>
          </tr>
        ))}
      </tbody>
    </HTMLTable>
  );
}
