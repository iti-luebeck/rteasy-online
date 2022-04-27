# Introduction

RTeasy-Online is a development environment for the register transfer language RTeasy. With RTeasy it is possible to design and simulate register transfer programs. Moreover, execution unit and control unit can be extracted from the algorithms. Among other things, this makes it possible to export an RTeasy program to VHDL.

## Tutorial

The tutorial is the main part of this book. It starts with an [explanation of the language](tutorial/lang). After that there is a user guide for the [IDE](tutorial/ide) and for the [CLI](tutorial/cli). This is followed by a few practical application [examples](tutorial/examples) and additional [advanced/in-depth material](tutorial/advanced).

If you are already familiar with RTeasy or RT code in general, you may want to jump straight to the [examples](tutorial/examples) to see what can be done with RTeasy.

## Compiler Error Index

The [compiler error index](compiler-error-index/errors.md) serves as an overview of all possible errors that can occur during compilation.

## Code Blocks

You will find code blocks in many places in this book. They may contain several different icons for interacting with them:

| Icon                       | Description                                                                                                                         |
| -------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| <i class="fa fa-copy"></i> | Copies the code block into your local clipboard.                                                                                    |
| <i class="fa fa-eye"></i>  | Toggle visibility of "hidden" lines. Some examples will hide lines that are not particularly relevant to what is being illustrated. |

Here is an example:

```rteasy
~declare register X(7:0) # <-- hidden by default
~
# Increment X by 1
X <- X + 1;
```
