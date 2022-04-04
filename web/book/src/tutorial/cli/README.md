# CLI User Guide

The CLI is a alternative frontend to the graphical user interface. It currently supports the help, check, gen-vhdl and test commands. The help command displays a general help page or help for a subcommand. The check command checks if the given RTeasy program is syntactically and semantically valid. The gen-vhdl command can be used to export an RTeasy program to VHDL. The test command allows automatic testing of an RTeasy program against a test file.

The CLI can be downloaded from [GitHub](https://github.com/iti-luebeck/rteasy-online/releases).

## Help

Run `rt-easy-cli --help` to display the message below, or run, for example, `rt-easy-cli test --help` to get more detailed information about a specific subcommand.

```bash
rt-easy-cli 0.1.0
rt easy cli

USAGE:
    rt-easy-cli [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
        --no-ansi    Disable ansi colors
    -V, --version    Prints version information

SUBCOMMANDS:
    check       Check the rt file
    gen-vhdl    Generate VHDL code
    help        Prints this message or the help of the given subcommand(s)
    test        Test the rt file
```

## Check

The check command checks if the given RTeasy program is syntactically and semantically valid. The program to be checked is passed as a file path.

```shell
$ rt-easy-cli check path/to/my/code.rt
```

## Generate VHDL Code

The gen-vhdl command can be used to export an RTeasy program to VHDL. The first argument is the path to the RTeasy program and the second argument is the path where the generated VHDL code is to be saved.

```shell
$ rt-easy-cli gen-vhdl path/to/my/code.rt path/to/generated.vhdl
```

The name of the generated VHDL module can be specified via `--module-name <module-name>`, otherwise the name of the VHDL file is used. In addition, memories can be pre-initialized via `--memories MEM_A path/to/MEM_A.rtmem MEM_B path/to/MEM_B.rtmem` (see [Memory File Format](../advanced/memory-file-format.md)).

## Test

The test command allows automatic testing of an RTeasy program against a test file. The first argument is the path to the RTeasy program and the second argument is the path to the test file.

```shell
$ rt-easy-cli test path/to/my/code.rt path/to/my/test_file.rtt
```

The test only has access to the interface of the program, meaning only to the inputs and ouputs. The syntax and semantic of a test file is explained in the following example:

```rteasy,ignore
# Test files support line comments prefixed by a `#`.
# All statements are newline terminated and are simply executed sequentially.

# Step: Execute one or more steps on the simulator.
# If no amount is given, one step is performed. Examples:
step
step 5
step 1
step 2

# Micro step: Execute one or more micro steps on the simulator.
# If no amount is given, one micro step is performed. Examples:
microStep
microStep 5
microStep 1
microStep 2

# Run: Run the simulator to the end or until a breakpoint is reached.
run

# Reset: Reset the simulator and all breakpoints.
reset

# Set/Remove breakpoint: Set or remove a breakpoint at the given label. Examples:
set breakpoint MY_LABEL
set breakpoint LABEL_B
remove breakpoint MY_LABEL

# Set input: Writing to inputs is similar to assigning in RTeasy programs.
# The expression can use all inputs and outputs of the program. Examples:
MY_INPUT <- 42 + -1
MY_INPUT <- MY_OUTPUT(3:0)."1"

# Assert: Assert statements work the same way as in RTeasy with the restriction
# that only inputs and ouputs can be accessed. Examples:
assert MY_OUTPUT = 28
assert 1
assert MY_INPUT + 4 = MY_OUTPUT
```

### Example Usage

**Task**: Find the sum of two numbers in the two's complement. For the calculation 8 bit wide registers are to be used. The two summands are to be read in via an input named `IN` in the first and second clock cycle respectively. After termination the result is to be available in the output `OUT`.

**Test file**:

```rteasy,ignore
# Test various additions. Always reset after each test case.

# Test 8 + 3 = 11
IN <- 8
step
IN <- 3
run
assert OUT = 11
reset

# Test 255 + 1 = 0
IN <- 255
step
IN <- 1
run
assert OUT = 0
reset
```

**Possible valid solutions:**

```rteasy
declare input IN(7:0)
declare output OUT(7:0)

OUT <- IN;
OUT <- OUT + IN;
```

```rteasy
declare input IN(7:0)
declare output OUT(7:0)
declare register A(7:0), B(7:0)

A <- IN;
B <- IN;
OUT <- A + B;
```
