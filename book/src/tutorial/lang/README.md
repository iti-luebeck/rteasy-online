# The Register Transfer Language RTeasy

A register transfer language (RT language) is used to describe the hardware using register operations. The following basic pattern is used for the operations:

```
Z <- f(X_1, X_2, ..., X_m)
```

where `Z`, `X_1`, `X_2`, ... , `X_m` represent identifiers for registers and the function `f` describes the operation. The result is then stored in `Z`.

An RTeasy program consists of a set of declarations followed by the actual algorithm. All declarations must be placed in the header of the program. Besides registers, buses, register arrays and memories can be declared. These are all explained in more detail in the next section.

An important concept in RTeasy is the difference between the clocked and the unclocked items. While registers, register arrays and memories are clocked, buses are unclocked. This means that values assigned to a register are only available in the next clock cycle. In contrast, values assigned to a bus are yet available in the same and only in this clock cycle.
