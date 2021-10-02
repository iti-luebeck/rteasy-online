import React, { useState, useContext, useMemo } from "react";
import { HTMLTable, Text, Button } from "@blueprintjs/core";

import { InputValue, Focused, BaseInheritSelect } from "../components";
import { useFilePicker } from "../hooks/useFilePicker";
import { downloadFile } from "../util/downloadFile";
import { GlobalContext, BaseInherit } from "../global/context";

interface Props {
  memory: string;
}

const MemoryStateView: React.FC<Props> = ({ memory }) => {
  // Context and state
  const globalModel = useContext(GlobalContext);
  const [pageNr, setPageNr] = useState("1");
  const [focused, setFocused] = useState<Focused | null>(null);
  const [baseInherit, setBaseInherit] = useState<BaseInherit>(() => {
    if (globalModel.tag === "Run") {
      return globalModel.inheritBasesStorage.current.get(memory) ?? "Inherit";
    }
    return "Inherit";
  });
  const base = baseInherit === "Inherit" ? globalModel.base : baseInherit;

  // Page count
  const pageCount = useMemo(() => {
    if (globalModel.tag === "Edit") return "";
    return globalModel.memoryPageCount(memory);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [globalModel.tag, memory]);

  // File picker
  const openLoadFromSaveFilePicker = useFilePicker({
    accept: [".rtmem"],
    onChange: (_name, content) => {
      if (globalModel.tag === "Edit") return;
      globalModel.memoryLoadFromSave(memory, content);
    },
  });

  if (globalModel.tag === "Edit") {
    return <div>Err</div>;
  }

  return (
    <div style={{ height: "100%", padding: "0 8px" /*, overflow: "hidden"*/ }}>
      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          alignItems: "center",
        }}
      >
        <BaseInheritSelect
          value={baseInherit}
          onChange={(baseInherit) => {
            if (globalModel.tag === "Run") {
              globalModel.inheritBasesStorage.current.set(memory, baseInherit);
            }
            setBaseInherit(baseInherit);
          }}
        />
        <div
          style={{
            display: "flex",
            alignItems: "center",
            marginLeft: "auto",
          }}
        >
          <Button onClick={() => openLoadFromSaveFilePicker()} small>
            Load
          </Button>
          <div style={{ width: 8 }} />
          <Button
            onClick={() =>
              downloadFile(
                `memory-${memory}.rtmem`,
                globalModel.memorySave(memory)
              )
            }
            small
          >
            Save
          </Button>
        </div>
      </div>

      <div style={{ height: 16 }} />

      <div
        style={{
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Button
          icon="arrow-left"
          onClick={() => {
            const pageNrPrev = globalModel.memoryPagePrev(memory, pageNr);
            if (pageNrPrev !== null) setPageNr(pageNrPrev);
          }}
          minimal
          small
        />
        <Text>
          {pageNr} / {pageCount}
        </Text>
        <Button
          icon="arrow-right"
          onClick={() => {
            const pageNrNext = globalModel.memoryPageNext(memory, pageNr);
            if (pageNrNext !== null) setPageNr(pageNrNext);
          }}
          minimal
          small
        />
      </div>

      <div style={{ height: 16 }} />

      <HTMLTable width="100%" bordered condensed>
        <thead>
          <tr>
            <th>Address</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          {globalModel.memoryPage(memory, pageNr, base).map((row) => (
            <tr key={row.address}>
              <td>{row.address}</td>
              <td>
                <InputValue
                  focused={focused}
                  setFocused={setFocused}
                  inputKey={row.address}
                  value={() => row.value}
                  valueNext={null}
                  onChanged={(value: string) =>
                    globalModel.writeMemory(memory, row.address, value, base)
                  }
                />
              </td>
            </tr>
          ))}
        </tbody>
      </HTMLTable>
    </div>
  );
};

export default MemoryStateView;
