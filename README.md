# RTeasy-Online &emsp; [![Website iti-luebeck.github.io/rteasy-online](https://img.shields.io/website-up-down-green-red/https/iti-luebeck.github.io/rteasy-online.svg)](https://iti-luebeck.github.io/rteasy-online/) [![CI](https://github.com/iti-luebeck/rteasy-online/workflows/ci/badge.svg)](https://github.com/iti-luebeck/rteasy-online/actions) [![Lines Of Code](https://tokei.rs/b1/github/iti-luebeck/rteasy-online?category=code)](https://github.com/iti-luebeck/rteasy-online) [![codecov](https://codecov.io/gh/iti-luebeck/rteasy-online/branch/main/graph/badge.svg?token=lAURO5lxWL)](https://codecov.io/gh/iti-luebeck/rteasy-online)

(Note: This README is intended for users rather than contributors. If you want to contribute, you should check [here](./DEVELOPMENT.md).)

## About

RTeasy-Online is a development environment for the register transfer language RTeasy. With RTeasy it is possible to design and simulate register transfer programs. Moreover, execution unit and control unit can be extracted from the algorithms. Among other things, this makes it possible to export an RTeasy program to VHDL.

RTeasy-Online is the successor to [RTeasy2](https://github.com/iti-luebeck/RTeasy2).

## Getting Started

The best way to get started is to read [the book](https://iti-luebeck.github.io/rteasy-online/book/) first.

The IDE is available at https://iti-luebeck.github.io/rteasy-online/.

The CLI can be downloaded from the [releases](https://github.com/iti-luebeck/rteasy-online/releases).

## Example

A simple multiplier for two integer, positive fixed point numbers. The two factors are read in via one input. The result is output via one output.

```bash
# Declare the registers. FACTOR and A are the two factors that are read from the input. RES is the result register.
declare register A(7:0), FACTOR(7:0), RES(15:0)

# Declare input and output.
declare input IN(7:0)
declare output OUT(15:0)

# Here in the first cycle the value for A and then in the second cycle the value for FACTOR is read in via the input IN.
BEGIN:
A <- IN, RES <- 0;
FACTOR <- IN;

# A loop follows which checks whether the value in FACTOR is zero. If yes, the result is written to OUT, if not, FACTOR is decremented and the value from A is added to RES.
LOOP:
if FACTOR = 0 then
    OUT <- RES
else
    RES <- RES + A,
    FACTOR <- FACTOR - 1,
    goto LOOP
fi;

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
