# Development

## Tools

If you want to build, test and run RTeasy-Online locally, you need to install a few things.

- **Rust**: Rust is basically required for everything and can be installed from [here](https://www.rust-lang.org/).

- **Node and npm**: Node and npm are required to build the IDE and can be installed from [here](https://nodejs.org/en/).

- **mdbook**: mdbook is required to build the book. ~~It can be installed by executing the command `cargo install mdbook`.~~ It currently needs to be installed via the following command because it uses a feature that has not yet been released:

  ```
  cargo install mdbook --git https://github.com/rust-lang/mdBook --rev refs/pull/1761/head
  ```

- **ghdl**: ghdl is required to run the vhdl tests and can be installed from [here](https://github.com/ghdl/ghdl).

## Project Structure

This repository contains the complete source code for RTeasy-Online and is divided into the following components:

- **[Backend](./backend/README.md)**: The backend contains the core logic for RTeasy. Here you can find among others the parser, compiler and simulator. All code in here is frontend agnostic and can be used from a graphical as well as from a console-based interface. The [API Docs](https://iti-luebeck.github.io/rteasy-online/dev/docs/backend/rt_easy) for the backend are built automatically.

- **[IDE](./app/README.md)**: The IDE is a graphical frontend built on top of the backend.

- **[CLI](./cli/README.md)**: The CLI is a console-based frontend built on top of the backend.

- **[Book](./book/README.md)**: The book is the best way to get started with RTeasy-Online. It contains a tutorial and detailed explanations of all possible errors that can occur when compiling RTeasy programs.

- **[VHDL Tests](./vhdl-tests/README.md)**: Here you can find tests for the VHDL export of RTeasy-Online. Various examples are compiled to VHDL and then tested with [ghdl](https://github.com/ghdl/ghdl).

- **[Wasm](./wasm/README.md)**: Small wrapper around the backend to make it accessible from Wasm. It makes use of [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen). This is mainly used for the IDE.
