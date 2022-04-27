| Precedence | Operator              | Associativity | Individual operators |
| ---------- | --------------------- | ------------- | -------------------- |
| 10         | Unary Sign            | right-to-left | `- ... `             |
| 10         | Unary Negation        | right-to-left | `neg ... `           |
| 9          | Sign Extend           | right-to-left | `sxt ... `           |
| 8          | Addition              | left-to-right | `... + ...`          |
| 8          | Subtraction           | left-to-right | `... - ...`          |
| 7          | Less Than             | left-to-right | `... < ...`          |
| 7          | Less Than Or Equal    | left-to-right | `... <= ...`         |
| 7          | Greater Than          | left-to-right | `... > ...`          |
| 7          | Greater Than Or Equal | left-to-right | `... >= ...`         |
| 6          | Equality              | left-to-right | `... = ...`          |
| 6          | Inequality            | left-to-right | `... <> ...`         |
| 5          | Bitwise NOT           | right-to-left | `not ...`            |
| 4          | Bitwise NAND          | left-to-right | `... nand ...`       |
| 3          | Bitwise AND           | left-to-right | `... and ...`        |
| 2          | Bitwise NOR           | left-to-right | `... nor ...`        |
| 1          | Bitwise OR            | left-to-right | `... or ...`         |
| 0          | Bitwise XOR           | left-to-right | `... xor ...`        |
