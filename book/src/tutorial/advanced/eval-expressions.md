# Evaluation of Expressions

This chapter describes in detail how expressions are evaluated. Starting with the local concepts of signedness and overflow behavior. Next, the precedence and associativity of operators is defined. The last section defines in which size expressions are evaluated in and how this is derived.

## Signedness

Signedness specifies whether a numeric data type can represent negative and positive numbers or only non-negative numbers. For example C has `unsigned int` and `signed int`.

RTeasy has no concept of signed variables or numbers. All operations always treat the operands as unsigned numbers.

Although there are no signed numbers, expressions like `-1` are still valid. This is possible because this is parsed as `neg 1` where `neg` is just the arithmetic negation operator.
This also fits well with how the size of expressions is calculated, as shown in [Expression Size and Context Size](#expression-size-and-context-size).

For operations like add, subtract and negate there is nothing to consider, because they behave the same for signed and unsigned numbers in the 2's complement anyway. Care must be taken with comparisons. Since every number is always treated as unsigned, unexpected things can happen as seen in the following example:

```rteasy
# The following assert will hold,
# because it is equivalent to 0b1111 > 0b1100
assert -1 > 12;
```

## Overflow Behavior

Arithmetic overflow can occur if the result of an operation is outside of the range of the value produced. Since registers and buses are sized in RTeasy this behavior can occur.

Overflow behavior affects only two operators in RTeasy: Addition and subtraction. These two arithmetic operations will always overflow and wrap, for example:

```rteasy
declare register X(2:0) # 0b000 to 0b111, or 0 to 7
X <- 0b100 + 0b101; # 4 + 5
assert X = 0b001; # X = 9 mod 8 = 1
```

## Operator Precedence and Associativity

Operator precedence and associativity forms the basis for how an expression is parsed. The following table gives an overview of all operators. Operators with higher precedence are executed before operators with lower precedence. Precedence comes before associativity.

{{#include ../../includes/operators.md}}

## Expression Size and Context Size

At first glance it may seem that there is only one type in RTeasy: unsigned integer.
But in fact RTeasy considers the exact size of each expression. Thus, each possible size of an expression results in a different type. This raises the question what happens when two expressions with different sizes are added, for example.
In RTeasy there are no explicit casts, instead there are rules about which expression yields which size.

To determine the size there are actually two algorithms. One for calculating the (minimum) size of an expression and one for providing a size an expression will actually evaluate in.

### Size

The first algorithm runs at compile time. It annotates each expression with the calculated size. The algorithm operates bottom-up. The resulting size is used for two purposes: First, to check if an expression can fit into a target, e.g. a register. So an expression of size 7 can fit into an 8 bit wide register, but not into a 4 bit wide register. Second, the calculated size is needed for the second algorithm. The rules for calculating the size are specified in the following table:

| Expression type                                        | Size                                      |
| ------------------------------------------------------ | ----------------------------------------- |
| Register/Bus/Register Array                            | Size of bit range                         |
| Bit string literal                                     | Amount of digits                          |
| Other literals                                         | Size of shortest binary representation    |
| Concatenation                                          | Sum of the sizes of all subexpressions    |
| Unary operators (-, neg, sxt, not)                     | Size of inner expression                  |
| Comparisons (<, <=, >, >=, =, <>)                      | 1                                         |
| Other binary operators (+, -, nand, and, nor, or, xor) | Maximum size of left and right expression |

### Context Size

The second algorithm runs at runtime in the case of the simulator. When generating hardware, it is part of the compiler instead. The algorithm operates top-down. The size calculated by the algorithm will be called "context size" in the following. An expression that is evaluated with the context size _n_ always returns a result that is _n_ bit wide.

For leaf expressions, like literals or registers, this is achieved by just zero extending the value to the context size. For unary/binary operations, like a comparison, this is achieved by zero extending the result of the operation, except for `sxt` where the result of the inner expression is sign extended.

The algorithm is used when executing assignments, if conditions and switch-case values. Depending on where it is used, the initial context size will vary. For assignments, the initial context size is equal to the size of the target. For if conditions the initial size is always one and for switch-case values it is equal to the size of the expression which it is matched against. The rules for passing down the context size are specified in the following table (Inherit means that the context size received is simply passed on):

| Expression type                                       | Passed down context size                  |
| ----------------------------------------------------- | ----------------------------------------- |
| Register/Bus/Register Array                           | n/a                                       |
| Literals                                              | n/a                                       |
| Concatenation                                         | Each subexpression gets its own size      |
| Sign Extend (sxt)                                     | Size of inner expression                  |
| Other unary operators (-, neg, not)                   | Inherit                                   |
| Comparisons (<, <=, >, >=, =, <>)                     | Maximum size of left and right expression |
| Other binary operators (+, -, nand, and, nor, or, xor | Inherit                                   |
