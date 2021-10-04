import React from "react";
import { editor } from "monaco-editor";

import { RtEasy } from "../../wasm";
import { GlobalModelEdit } from "../context";
import { model as modelCommon } from "./common";
import { State, StateEdit } from "../state";

export function model(
  rtEasy: RtEasy,
  state: StateEdit,
  setState: React.Dispatch<React.SetStateAction<State>>,
  editorModel: editor.IModel
): GlobalModelEdit {
  const build = () => {
    try {
      const onChange = () =>
        setState((prev) => {
          return { ...prev };
        });
      const simulator = rtEasy.build(editorModel.getValue(), onChange);
      setState({
        tag: "Run",
        editor: state.editor,
        base: state.base,
        clockRate: state.clockRate,
        simulator,
      });
    } catch (_e) {
      alert("Code has errors");
    }
  };

  return {
    ...modelCommon(state, setState, editorModel, build),
    tag: "Edit",
    build,
  };
}
