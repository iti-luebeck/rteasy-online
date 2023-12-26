import React, { useContext, useState, useEffect } from "react";
import {
  Classes,
  Dialog,
  Button,
  InputGroup,
  HTMLTable,
  Switch,
  FormGroup,
} from "@blueprintjs/core";

import { useFilePicker } from "../hooks/useFilePicker";
import { downloadFile } from "../util/downloadFile";
import { CompilerResult, Vhdl } from "../wasm";
import { showSuccessToast, showErrorToast } from "../toaster";
import { RtEasyContext } from "../wasm/context";
import { GlobalContext } from "../global/context";

const DEFAULT_NAME: string = "my_module";

interface Memory {
  name: string;
  file?: { name: string; content: string } | null;
}

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const VhdlExportDialog: React.FC<Props> = ({ isOpen, onClose }) => {
  const rtEasy = useContext(RtEasyContext);
  const globalModel = useContext(GlobalContext);
  const [vhdl, setVhdl] = useState<CompilerResult<Vhdl>>({
    tag: "Error",
    error_html: "",
  });
  const [memories, setMemories] = useState<Memory[]>([]);
  const [renderError, setRenderError] = useState("");
  const [moduleName, setModuleName] = useState("");
  const [isDebug, setIsDebug] = useState(false);

  // Set memory file by name
  const setMemoryFileByName = (
    name: string,
    file: { name: string; content: string } | null
  ) => {
    setMemories(
      memories.map((mem) =>
        mem.name === name ? { name: mem.name, file } : mem
      )
    );
  };

  // Memory file picker
  const openLoadMemory = useFilePicker({
    accept: [".rtmem"],
    onChange: (fileName, fileContent, memoryName: string) => {
      setMemoryFileByName(memoryName, { name: fileName, content: fileContent });
    },
  });

  // Reset if `isOpen` changes
  useEffect(() => {
    if (isOpen) {
      const vhdl = rtEasy.buildVhdl(globalModel.editorModel.getValue());
      setVhdl(vhdl);

      switch (vhdl.tag) {
        case "Ok":
          setMemories(
            vhdl.value.memories().map((name) => ({
              name,
              file: memories.find((mem) => mem.name === name)?.file,
            }))
          );
          break;
        case "Error":
          setMemories([]);
          break;
      }
    }
    setRenderError("");

    // Only listen to `isOpen`
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpen]);

  // Show error if any
  if (vhdl.tag === "Error") {
    return (
      <Dialog title="VHDL Export" onClose={onClose} isOpen={isOpen}>
        <div className={Classes.DIALOG_BODY}>
          <div style={{ marginBottom: 16 }}>
            <strong>Errors</strong>
          </div>
          <pre
            dangerouslySetInnerHTML={{
              __html: vhdl.error_html,
            }}
            style={{ margin: 0, overflow: "auto" }}
          ></pre>
        </div>
      </Dialog>
    );
  }

  // Export
  const doExport = (onSuccess: (name: string, content: string) => void) => {
    let name = moduleName.trim();
    if (name === "") name = DEFAULT_NAME;
    const res = vhdl.value.render(
      name,
      isDebug,
      memories
        .filter((mem) => mem.file)
        .map((mem) => ({ name: mem.name, file: mem.file!.content }))
    );

    switch (res.tag) {
      case "Ok": {
        setRenderError("");
        onSuccess(name, res.value);
        break;
      }
      case "Error": {
        setRenderError(res.error);
        break;
      }
    }
  };

  return (
    <Dialog title="VHDL Export" onClose={onClose} isOpen={isOpen}>
      <div className={Classes.DIALOG_BODY}>
        <div style={{ maxWidth: 300,  }}>
          <FormGroup
            label="Module Name"
            labelFor="module-name"
            
          >
            <InputGroup
              placeholder={DEFAULT_NAME}
              id="module-name"
              value={moduleName}
              onChange={(e) => setModuleName(e.target.value)}
            />
          </FormGroup>
          
          <FormGroup label="Flags">
            <Switch
              checked={isDebug}
              onChange={(e) => setIsDebug((e.target as HTMLInputElement).checked)}
              label="Enable Debug Interface"
            />
          </FormGroup>

        </div>
        {memories.length !== 0 && (
          <>
            <FormGroup label="Memories">
              <HTMLTable width="100%" bordered condensed>
                <thead>
                  <tr>
                    <th style={{ width: "20%" }}>Name</th>
                    <th>Content</th>
                  </tr>
                </thead>
                <tbody>
                  {memories.map((mem) => (
                    <tr key={mem.name}>
                      <td>{mem.name}</td>
                      <td>
                        {mem.file ? (
                          <>
                            <span>{mem.file.name}</span>
                            <Button
                              icon="delete"
                              onClick={() => setMemoryFileByName(mem.name, null)}
                              style={{ marginLeft: "16px" }}
                              minimal
                              small
                            />
                          </>
                        ) : (
                          <Button onClick={() => openLoadMemory(mem.name)} small>
                            Load
                          </Button>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </HTMLTable>
            </FormGroup>
          </>
        )}
        {renderError !== "" && (
          <>
            <div style={{ margin: "16px 0" }}>
              <strong>Error</strong>
            </div>
            <pre style={{ color: "red", overflow: "auto" }}>{renderError}</pre>
          </>
        )}
      </div>
      <div className={Classes.DIALOG_FOOTER}>
        <div className={Classes.DIALOG_FOOTER_ACTIONS}>
          <Button
            onClick={() =>
              doExport((_name, content) =>
                navigator.clipboard
                  .writeText(content)
                  .then(() => showSuccessToast({ message: "Copied" }))
                  .catch(() =>
                    showErrorToast({ message: "Failed to copy to clipboard" })
                  )
              )
            }
          >
            Copy to clipboard
          </Button>
          <Button
            onClick={() =>
              doExport((name, content) => downloadFile(`${name}.vhdl`, content))
            }
          >
            Download
          </Button>
        </div>
      </div>
    </Dialog>
  );
};

export default VhdlExportDialog;
