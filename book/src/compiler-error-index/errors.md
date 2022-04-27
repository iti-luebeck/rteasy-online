# Errors

An overview of all possible errors that can occur during compilation.

## E001

This error indicates that a symbol is declared more than once.

### Examples

```rteasy,compile_fail(E001)
declare register X(3:0)
declare register X # error: duplicate symbol "X"
```

```rteasy,compile_fail(E001)
declare register X(3:0)
declare bus X # error: duplicate symbol "X"
```

## E002

This error indicates that the length of a register array is not a power of two. The length must always be a power of two.

### Examples

```rteasy,compile_fail(E002)
# error: length of register array "ARR" must be a power of two
declare register array ARR(7:0)[3]
```

```rteasy,compile_fail(E002)
# error: length of register array "ARR" must be a power of two
declare register array ARR(7:0)[0]
```

## E003

This error occurs when a register array is used without an index expression.

### Examples

```rteasy,compile_fail(E003)
~declare register X(7:0)
~declare register array ARR(7:0)[4]
~
X <- ARR[0] + 1; # ok
X <- ARR + 1;    # error: register array "ARR" is missing index [...]
```

```rteasy,compile_fail(E003)
~declare register array ARR(7:0)[4]
~
ARR[3] <- 1 + 1; # ok
ARR <- 1 + 1;    # error: register array "ARR" is missing index [...]
```

## E004

This error indicates that a label is declared more than once. Labels are used as goto marks and must therefore be unique.

### Examples

```rteasy,compile_fail(E004)
~declare register X(3:0), Y(3:0)
~
MY_LABEL: X <- Y;
MY_LABEL: X <- X + 1; # error: duplicate label "MY_LABEL"
```

## E005

This error occurs when a symbol can not be found.

### Examples

```rteasy,compile_fail(E005)
declare register AR(3:0)
declare memory MEM(AR, DR) # error: no register named "DR" found
```

```rteasy,compile_fail(E005)
X <- 42 + 2; # error: no register or bus named "X" found
```

```rteasy,compile_fail(E005)
declare register X(3:0)
X <- ARR[0]; # error: no register array named "ARR" found
```

```rteasy,compile_fail(E005)
read MEM; # error: no memory named "MEM" found
```

## E006

This error occurs when a label can not be found.

### Examples

```rteasy,compile_fail(E006)
LABEL_A: goto LABEL_B; # error: no label named "LABEL_B" found
```

## E007

This error occurs when an expression without a fixed size is used in a switch operation. This requirement is necessary to have a well defined size in which to evaluate. Fixed size expression are: comparisons, concatenations, registers, buses, register arrays and bit strings.

### Examples

```rteasy,compile_fail(E007)
~declare register X(3:0), Y(3:0)
~
switch X + Y { # error: expected fixed size expression
    case 1: nop
    default: nop
};
```

```rteasy,compile_fail(E007)
switch 12 { # error: expected fixed size expression
    case 1: nop
    default: nop
};
```

```rteasy
~declare register X(3:0), Y(3:0)
~
switch X = Y { # ok
    case 1: nop
    default: nop
};
```

```rteasy
switch "1100" { # ok
    case 1: nop
    default: nop
};
```

## E008

This error occurs when a non-constant expression used in a case clause. Constant expression are: literals, concatenations only containing constants and terms only containing constants.

### Examples

```rteasy,compile_fail(E008)
~declare register X(3:0), Y(3:0)
~
switch "0101" {
    case X + Y: nop # error: expected constant expression
    default: nop
};
```

```rteasy
switch "0101" {
    case 7: nop # ok
    default: nop
};
```

```rteasy
switch "0101" {
    case 3 + 4: nop # ok
    default: nop
};
```

## E009

This error indicates a switch operation with zero or more than one default clause. Switch operations must always have exactly one default clause.

### Examples

```rteasy,compile_fail(E009)
# error: expected exactly one default clause
switch "0101" {
    case 1: nop
};
```

```rteasy,compile_fail(E009)
# error: expected exactly one default clause
switch "0101" {
    case 1: nop
    default: nop
    default: nop
};
```

## E010

This error occurs when a literal other than the bit string is used in a concatenation. Concatenations may only contain elements of fixed size, thus only registers, buses, register arrays and bit strings.

### Examples

```rteasy,compile_fail(E010)
~declare register X(7:0), Y(3:0)
~
X <- Y."101".Y(0); # ok
X <- Y.5.Y(0);     # error: concat must not contain numbers other than bit strings
```

## E011

This error occurs when the right-hand side of an assignment is wider than the target. Consider restricting the right-hand side to a smaller bit range or making the target wider.

### Examples

```rteasy,compile_fail(E011)
declare register X(7:0), Y(3:0)

X <- Y;      # ok
Y <- X(4:1); # ok
Y <- X;      # error: right-hand side is too wide: 8 > 4
```

## E012

This error occurs when the index expression of an register array is too wide. The size of the index must be less than or equal to `log2(arr_length)`, so that no invalid index can be used.

### Examples

```rteasy,compile_fail(E012)
~declare register X(7:0)
declare register array ARR(7:0)[4]

X <- ARR[0b01];  # ok
X <- ARR[0b11];  # ok
X <- ARR[0b100]; # error: index expression is too wide: 3 > 2
```

## E013

This error occurs when the condition of an assert or if operation is not exactly one bit wide. A condition must always be either `0` or `1`.

### Examples

```rteasy,compile_fail(E013)
declare register X(7:0), Y(3:0)

# error: condition expression must be exactly one bit wide, but is: 8
assert X + Y;

# error: condition expression must be exactly one bit wide, but is: 4
if Y then nop fi;
```

```rteasy
~declare register X(7:0), Y(3:0)
~
assert X = Y;         # ok
if X = Y then nop fi; # ok
```

## E014

This error indicates that a bit range exceeds the maximum size. The maximum is `65536 = 2^16` by default. Address registers of memories are further limited to a size of `64 = 2^6`.

### Examples

```rteasy,compile_fail(E014)
# error: bit range size exceeds max size: 65538 > 65536
declare bus B(0:65537)
```

```rteasy,compile_fail(E014)
declare register AR(128:32), DR(3:0)
declare memory MEM(AR, DR) # error: bit range size exceeds max size: 97 > 64
```

## E015

This error occurs when a case value is wider than the expression being compared to. Consider making the expression wider or shrinking the case value.

### Examples

```rteasy,compile_fail(E015)
declare register X(3:0)

switch X {
    case 9: nop       # ok
    case "1111": nop  # ok
    case "10001": nop # error: case value is too wide: 5 > 4
    default: nop
};
```

## E016

This error indicates a duplicate case value. Each case arm must have a unique value.

### Examples

```rteasy,compile_fail(E016)
~declare register X(3:0)
~
switch X {
    case 3: nop     # ok
    case 2 + 1: nop # error: duplicate case value

    case 4: nop # ok
    case 4: nop # error: duplicate case value

    default: nop
};
```

## E017

This error indicates that the left-hand side of an assignment contains clocked and unclocked variables. Since they take on their new values at different times, it is not allowed to combine them on the left-hand side.

### Examples

```rteasy,compile_fail(E017)
declare register REG1(3:0), REG2(3:0)
declare bus BUS1(3:0), BUS2(3:0)

REG1.REG2 <- 42; # ok
BUS1.BUS2 <- 42; # ok

# error: the left-hand side of the assignment may contain either clocked or unclocked variables only
REG1.BUS1 <- 42;
```

## E018

This error indicates that the left-hand side of an assignment contains non-variable items, such as bit strings.

### Examples

```rteasy,compile_fail(E018)
~declare register X(3:0), Y(3:0)
~
# error: the left-hand side of the assignment must be a variable
X."010".Y <- X + Y;
```

## E019

This error indicates that the left-hand side of an assignment contains an input. Inputs are read-only and therefore cannot be assigned from within the program.

### Examples

```rteasy,compile_fail(E019)
declare input IN(7:0)
declare bus BUS(3:0)

IN <- 12;    # error: cannot assign to input (inputs are read-only)
BUS.IN <- 3; # error: cannot assign to input (inputs are read-only)
```

## E020

This error occurs when a used bit range exceeds the declaration. Bit ranges must specify a subrange of the declaration. Furthermore, they must also have the same "direction", e.g. `(2:0)` is contained in `(3:0)`, but **not** in `(0:3)`.

### Examples

```rteasy,compile_fail(E020)
declare register X(7:0), Y(0:3)

X <- X(6:2); # ok
X <- X(15);  # error: bit range (15) exceeds declaration (7:0)

Y <- Y(1:1); # ok
Y <- Y(2:0); # error: bit range (2:0) exceeds declaration (0:3)
```

## E021

This error occurs when a goto operation is used before the pipe. If a statement uses the pipe operator and thus the conditional branch, all gotos must be placed after the pipe.

### Examples

```rteasy,compile_fail(E021)
~declare register X(7:0)
~
# ok, since this statement has no conditional branch
goto L1;

# error: no goto operations are allowed before pipe ("|")
goto L1 | if X = 0 then goto L2 fi;
~
~L1: nop;
~L2: nop;
```

## E022

This error occurs when mutating operations are used after the pipe. After the pipe only conditional (if/switch), nop and assert operations are allowed.

### Examples

```rteasy,compile_fail(E022)
~declare register X(7:0)
~
# error: no mutating operations allowed after pipe ("|")
nop | X <- 12;
```

```rteasy
~declare register X(7:0)
~
~START:
X <- 2 + 2 | assert X = 4;         # ok
nop | if X = 5 then goto START fi; # ok
```

## E023

This error occurs when a sign extend operator is used on a term. The sign extend operator may only be applied on simple expressions, like registers, buses or concatenations.

### Examples

```rteasy,compile_fail(E023)
~declare register X(7:0), Y(3:0)
~
X <- sxt Y;       # ok
X <- sxt (Y + 1); # error: sxt operator is not supported for terms
```

## E024

This error occurs when a symbol can be found but is of the wrong type. For example, a memory is always expected in a read operation.

### Examples

```rteasy,compile_fail(E024)
declare register X(7:0), Y(3:0)
declare memory MEM(X, Y)

read X;   # error: expected memory, found: register
MEM <- 2; # error: expected register or bus, found: memory
```

## E025

This error indicates that a register, bus, register array or memory is assigned more than once in a cycle. Only one assignment to an item may be executed per execution path and cycle.

### Examples

```rteasy,compile_fail(E025)
~declare register X(3:0)
~
X <- 2, X <- 1; # error: register "X" is assigned more than once
```

```rteasy,compile_fail(E025)
~declare register AR(3:0), DR(3:0)
~declare memory MEM(AR, DR)
~
read MEM, read MEM; # error: register "DR" is assigned more than once
write MEM, write MEM; # error: memory "MEM" is assigned more than once
```

```rteasy
~declare register X(3:0), COND
~
# ok, because always only one of the two assignments is executed in one cycle
if COND then X <- 2 else X <- 1 fi;
```

## E026

This error indicates that a statement contains multiple gotos on at least one possible execution path. Only one goto may be executed per execution path and cycle.

### Examples

```rteasy,compile_fail(E026)
# error: statement contains multiple gotos on at least one possible execution path
goto L1, goto L2;
~
~L1: nop;
~L2: nop;
```

```rteasy,compile_fail(E026)
~declare register COND
~
# error: statement contains multiple gotos on at least one possible execution path
goto L1, if COND then goto L2 fi;
~
~L1: nop;
~L2: nop;
```

```rteasy
~declare register COND
~
# ok, because always only one of the gotos is executed in one cycle
if COND then goto L1 else goto L2 fi;
~
~L1: nop;
~L2: nop;
```

## E027

This error indicates that a register array is read more than 2 times on at least one possible execution path. Only 2 read ports are available per register array and cycle.

### Examples

```rteasy,compile_fail(E027)
~declare register X(7:0)
~declare register array ARR(3:0)[4]
~
# error: register array "ARR" is read more than 2 times
X <- ARR[0] + ARR[1] + ARR[2];

# error: register array "ARR" is read more than 2 times
X <- ARR[1] + ARR[1] + ARR[1];
```

```rteasy
~declare register X(7:0), COND
~declare register array ARR(3:0)[4]
~
# ok, since the register array is read at most twice
if COND then X <- ARR[0] else X <- ARR[1] + ARR[2] fi;
```

## E028

This error indicates that a statement has a feeback loop. This can happen, for example, when the value of a bus A depends on a bus B, and B in turn depends on A.

### Examples

```rteasy,compile_fail(E028)
declare bus A, B

# error: statement has a feedback loop
# (A depends on B and B depends on A)
A <- B, B <- A;
```

```rteasy,compile_fail(E028)
declare bus A, B

# error: statement has a feedback loop
# (A depends on itself)
if A then A <- B fi;
```

## E029

This error occurs when a register array is used with a bit range on the left-hand side of an assignment. With register arrays it is only possible to access individual bits when reading.

The access to a register in an array is always exhaustive. When writing all bits must be written and when reading all bits must be read. Since single bits can be simply ignored when reading, it is possible to specify a bit range when reading.

### Examples

```rteasy,compile_fail(E029)
~declare register X(3:0)
~declare register array ARR(7:0)[4]
~
X <- ARR[2](3:0); # ok

ARR[2](3:0) <- X; # error: bit range is not allowed in this position
```

## E200

_(VHDL Export only)_

This error indicates that a goto operation before the pipe operator is executed conditionally depending on a bus. The gotos are transformed to a state machine during the VHDL export. A dependency of the next state on an unclocked element is not possible with this.

This can be solved by moving the goto operation(s) after the pipe, but beware: this might change the semantics of the program! For example in the code below if we move `if BUS = 2 then goto END fi` after the pipe, this will change the semantics. Before the pipe `BUS` gets the old value of `X` (3), and after the pipe `BUS` gets the new value of `X` (2).

### Examples

```rteasy,vhdl_fail(E200)
declare register X(7:0)
declare bus BUS(7:0)

X <- 3;
BUS <- X, X <- 2, if BUS = 2 then goto END fi; # error: next state depends on an unclocked item
X <- X + 42;
END:
```

## E201

_(VHDL Export only)_

This error indicates that the first state contains a conditional goto operation before the pipe operator. The gotos are transformed to a state machine during the VHDL export. For this transformation it is not possible that the first state contains a goto before the pipe operator.

If it is not possible to get rid of the conditional goto, there are two possible solutions: First, by simply inserting an empty state before the first state (`nop;`). And second, by moving the goto operation(s) after the pipe, but beware: this might changes the semantics of the program if the registers have different values after the pipe!

### Examples

```rteasy,vhdl_fail(E201)
declare register X(3:0)

if X(0) then goto SKIP fi; # error: conditional goto in first state
X <- 4;
SKIP: X <- X + 1;
```
