# FluxSharp Operators

All operators and expressions.

## Arithmetic Operators

### Addition (+)

```flux
int a = 5 + 3;          // 8
int b = 10 + 20;        // 30
string c = "Hello" + " World";  // "Hello World"
```

### Subtraction (-)

```flux
int a = 10 - 3;         // 7
int b = 5 - 10;         // -5
```

### Multiplication (*)

```flux
int a = 5 * 3;          // 15
int b = 10 * 2;         // 20
```

### Division (/)

```flux
int a = 10 / 2;         // 5
int b = 15 / 3;         // 5
int c = 5 / 2;          // 2 (integer division)
```

### Modulo (%)

```flux
int a = 17 % 5;         // 2
int b = 10 % 3;         // 1
int c = 7 % 2;          // 1
```

## Comparison Operators

### Less Than (<)

```flux
if (x < 10) { }
if (5 < 10) { }         // true
if (15 < 10) { }        // false
```

### Greater Than (>)

```flux
if (x > 5) { }
if (10 > 5) { }         // true
if (3 > 5) { }          // false
```

### Less or Equal (<=)

```flux
if (x <= 10) { }
if (5 <= 10) { }        // true
if (10 <= 10) { }       // true
```

### Greater or Equal (>=)

```flux
if (x >= 5) { }
if (10 >= 5) { }        // true
if (5 >= 5) { }         // true
```

### Equal (==)

```flux
if (x == 10) { }
if (5 == 5) { }         // true
if (5 == 3) { }         // false
```

### Not Equal (!=)

```flux
if (x != 10) { }
if (5 != 3) { }         // true
if (5 != 5) { }         // false
```

## Logical Operators

### AND (&&)

```flux
if (x > 5 && y < 10) {  // Both true
    // Execute
}

if (true && true) { }   // true
if (true && false) { }  // false
if (false && false) { } // false
```

### OR (||)

```flux
if (x > 5 || y < 10) {  // At least one true
    // Execute
}

if (true || true) { }   // true
if (true || false) { }  // true
if (false || false) { } // false
```

### NOT (!)

```flux
if (!condition) { }
if (!(x > 5)) { }
if (!flag) { }

if (!true) { }          // false
if (!false) { }         // true
```

## Bitwise Operators

### AND (&)

```flux
int a = 5 & 3;          // 0101 & 0011 = 0001 (1)
int b = 7 & 3;          // 0111 & 0011 = 0011 (3)
```

### OR (|)

```flux
int a = 5 | 3;          // 0101 | 0011 = 0111 (7)
int b = 4 | 2;          // 0100 | 0010 = 0110 (6)
```

### XOR (^)

```flux
int a = 5 ^ 3;          // 0101 ^ 0011 = 0110 (6)
int b = 7 ^ 3;          // 0111 ^ 0011 = 0100 (4)
```

### NOT (~)

```flux
int a = ~5;             // Bitwise complement
int b = ~0;             // All bits set
```

### Left Shift (<<)

```flux
int a = 5 << 1;         // 0101 << 1 = 1010 (10)
int b = 3 << 2;         // 0011 << 2 = 1100 (12)
```

### Right Shift (>>)

```flux
int a = 5 >> 1;         // 0101 >> 1 = 0010 (2)
int b = 12 >> 2;        // 1100 >> 2 = 0011 (3)
```

## Assignment Operators

### Assign (=)

```flux
int x = 10;
x = 20;
x = 30;
```

### Add Assign (+=)

```flux
int x = 10;
x += 5;                 // x = x + 5 (15)
```

### Subtract Assign (-=)

```flux
int x = 20;
x -= 5;                 // x = x - 5 (15)
```

## Increment/Decrement

### Increment (++)

```flux
int x = 5;
x++;                    // x becomes 6
```

### Decrement (--)

```flux
int x = 5;
x--;                    // x becomes 4
```

## Expression Syntax

From grammar:

```
expr = { atom ~ ((arith_op | bitwise_op) ~ atom)* }
```

This means:
- Starts with an atom (value, variable, function call, parentheses)
- Can be followed by arithmetic or bitwise operators
- Operators are left-associative

Examples:
```flux
5 + 3 * 2           // 11 (follows precedence)
(5 + 3) * 2         // 16 (parentheses override)
a & b | c           // (a & b) | c
x >> 2 << 1         // (x >> 2) << 1
```

## Operator Precedence

From grammar, precedence (highest to lowest):

1. Parentheses `()`
2. Array access `[]`
3. Function call `()`
4. Member access `.`
5. Unary `!` `~` (in conditions)
6. Multiplication `*` Division `/` Modulo `%`
7. Addition `+` Subtraction `-`
8. Shift `<<` `>>`
9. Bitwise AND `&`
10. Bitwise XOR `^`
11. Bitwise OR `|`
12. Comparison `<` `>` `<=` `>=` `==` `!=`
13. Logical AND `&&`
14. Logical OR `||`
15. Assignment `=` `+=` `-=`

## Operator Examples

### Arithmetic Precedence

```flux
int a = 5 + 3 * 2;      // 11 (3*2=6, 5+6=11)
int b = (5 + 3) * 2;    // 16 (5+3=8, 8*2=16)
int c = 10 - 5 + 2;     // 7 (left-to-right: 10-5=5, 5+2=7)
```

### Logical Combinations

```flux
if (a > 5 && b < 10 || c == 0) {
    // (a > 5) AND (b < 10) OR (c == 0)
}
```

### Bitwise Operations

```flux
int flags = 0;
flags |= (1 << 0);      // Set bit 0
flags |= (1 << 2);      // Set bit 2
if (flags & (1 << 0)) { } // Check bit 0
```

---

Next: Read [ARRAYS.md](ARRAYS.md)

