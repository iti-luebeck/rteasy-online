import { useCallback } from "react";

export type Accept = "*" | string[];
export interface Options<Args extends unknown[]> {
  accept?: "*" | string[];
  onChange: (name: string, content: string, ...args: Args) => void;
}
export type FilePicker<Args extends unknown[]> = (...args: Args) => void;

export function useFilePicker<Args extends unknown[]>({
  accept,
  onChange,
}: Options<Args>): FilePicker<Args> {
  const openFilePicker = useCallback(
    (...args: Args) => {
      const inputElement = document.createElement("input");

      inputElement.type = "file";
      inputElement.multiple = false;
      inputElement.accept =
        accept === undefined || accept === "*" ? "*" : accept.join(",");

      inputElement.addEventListener("change", () => {
        const files = inputElement.files ? Array.from(inputElement.files) : [];
        if (files.length === 1) {
          const file = files[0];
          const reader = new FileReader();
          reader.onload = () => {
            const content = reader.result as string;
            onChange(file.name, content, ...args);
          };
          reader.readAsText(file);
        }
      });

      inputElement.click();
    },
    [accept, onChange]
  );

  return openFilePicker;
}
