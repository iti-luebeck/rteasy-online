import React from "react";
import { Classes, Dialog } from "@blueprintjs/core";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

const AboutDialog: React.FC<Props> = ({ isOpen, onClose }) => {
  return (
    <Dialog title="About" onClose={onClose} isOpen={isOpen}>
      <div className={Classes.DIALOG_BODY}>
        <p>
          RTeasy-Online is a development environment for the register transfer
          language RTeasy. With RTeasy-Online it is possible to design and
          simulate register transfer programs.
        </p>

        <br />

        <p>
          Institute of Computer Engineering (Institut für Technische
          Informatik), University of Lübeck
        </p>

        <br />

        <p>
          <strong>Author:</strong> Jannik Obermann
        </p>
        <p>
          <strong>GitHub:</strong>{" "}
          <a
            href="https://github.com/iti-luebeck/rteasy-online"
            target="_blank"
            rel="noreferrer"
          >
            https://github.com/iti-luebeck/rteasy-online
          </a>
        </p>
      </div>
    </Dialog>
  );
};

export default AboutDialog;
