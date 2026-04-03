# FluxSharp Compiler - Error Messages Guide

## Quick Reference

When you compile FluxSharp code with `fluxc`, the compiler will show you clear error messages if something is wrong.

## Common Errors and How to Fix Them

### ❌ Missing Semicolon

**What you write:**
```rust
int x = 10
```

**Error message:**
```
❌ POSSIBLE SYNTAX ERROR at line 3:
  int x = 10
  Hint: Statement appears to be missing a semicolon (;) at the end
  Expected format: int x = 10;
```

**How to fix it:**
Add a semicolon at the end of the line:
```rust
int x = 10;
```

---

### ⚠️ Float Literal Format

**What you write:**
```rust
float f = 3.14;
```

**Error message:**
```
⚠️  FLOAT LITERAL ERROR at line 3:
  float f = 3.14;
  Hint: Float literals must end with 'f' or 'F'
  Correct format: 3.14f or 3.14F
```

**How to fix it:**
Add 'f' or 'F' after the decimal number:
```rust
float f = 3.14f;  // or 3.14F
```

**Note:** Double literals can use scientific notation without 'f':
```rust
double d = 3.14;      // OK
double d = 3.14e-2;   // OK
```

---

### ❌ Unmatched Parenthesis

**What you write:**
```rust
int x = max(10, 20;
```

**Error message:**
```
❌ UNMATCHED PARENTHESIS at line 3:
  int x = max(10, 20;
  Hint: Found 1 opening '(' but only 0 closing ')'
```

**How to fix it:**
Add the missing closing parenthesis:
```rust
int x = max(10, 20);
```

---

### ❌ Unmatched Bracket

**What you write:**
```rust
int arr[10;
```

**Error message:**
```
❌ UNMATCHED BRACKET at line 3:
  int arr[10;
  Hint: Found 1 opening '[' but only 0 closing ']'
```

**How to fix it:**
Add the missing closing bracket:
```rust
int arr[10];
```

---

### ❌ Undefined Variable

**What you write:**
```rust
print(y);  // y was never declared
```

**Error message:**
```
❌ Undefined variable: 'y'
   Make sure this variable is declared before use with: type y = value;
```

**How to fix it:**
Declare the variable before using it:
```rust
int y = 10;
print(y);
```

---

### ❌ Undefined Function

**What you write:**
```rust
int result = my_function(10);
```

**Error message:**
```
❌ Undefined function: 'my_function'
   Available math functions: abs, max, min, pow, floor, ceil, round, sqrt
```

**How to fix it:**
Use only available functions or check the spelling:
```rust
int result = abs(-10);  // Use a valid function
```

---

### ❌ Function Argument Count

**What you write:**
```rust
int x = max(10);  // max() needs 2 arguments
```

**Error message:**
```
❌ Function Error: max() requires exactly 2 arguments, but got 1
   Usage: max(value1, value2)
```

**How to fix it:**
Provide the correct number of arguments:
```rust
int x = max(10, 20);
```

---

### ⚠️ Type Mismatch in Function

**What you write:**
```rust
int x = max(10, 3.14f);  // Can't mix int and float
```

**Error message:**
```
⚠️  Type Error: max() requires both arguments to be the same numeric type
   Use: max(intA, intB), max(floatA, floatB), or max(doubleA, doubleB)
```

**How to fix it:**
Use the same type for both arguments:
```rust
int x = max(10, 20);           // Both int
// OR
float x = max(3.14f, 2.71f);   // Both float
```

---

## Available Math Functions

All these functions are available in FluxSharp:

| Function | Purpose | Example |
|----------|---------|---------|
| `abs(x)` | Absolute value | `abs(-42)` returns `42` |
| `max(a, b)` | Maximum of two values | `max(10, 20)` returns `20` |
| `min(a, b)` | Minimum of two values | `min(10, 20)` returns `10` |
| `pow(base, exp)` | Power function | `pow(2, 3)` returns `8` |
| `sqrt(x)` | Square root | `sqrt(16)` returns `4` |
| `floor(x)` | Round down | `floor(3.7f)` returns `3` |
| `ceil(x)` | Round up | `ceil(3.2f)` returns `4` |
| `round(x)` | Round to nearest | `round(3.6f)` returns `4` |

---

## Type System

FluxSharp supports these primitive types:

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer (32-bit) | `int x = 42;` |
| `long` | Long integer (64-bit) | `long x = 9999999;` |
| `float` | Floating-point (32-bit) | `float x = 3.14f;` |
| `double` | Double precision (64-bit) | `double x = 3.14159;` |
| `string` | Text | `string s = "hello";` |
| `bool` | Boolean | `bool b = true;` |
| `byte` | Single byte | `byte b = 255;` |
| `void` | No value (functions only) | `void main() { }` |

---

## Writing Valid FluxSharp Code

### Rule 1: Always end statements with semicolons
```rust
int x = 10;    // ✓ Correct
print(x);      // ✓ Correct
int y = 20     // ✗ Wrong - missing semicolon
```

### Rule 2: Match all parentheses and brackets
```rust
int x = (10 + 20);     // ✓ Correct
int arr[10];           // ✓ Correct
int y = (10 + 20;      // ✗ Wrong - missing )
```

### Rule 3: Declare variables before using them
```rust
int x = 10;    // ✓ Correct
print(x);      // ✓ Now x is defined
print(y);      // ✗ Wrong - y doesn't exist
```

### Rule 4: Use correct type suffixes for floats
```rust
float f = 3.14f;      // ✓ Correct
float g = 2.71f;      // ✓ Correct
float h = 1.41;       // ✗ Wrong - missing 'f'
```

### Rule 5: Match function argument types
```rust
int x = max(10, 20);          // ✓ Correct - both int
float y = max(3.14f, 2.71f);  // ✓ Correct - both float
int z = max(10, 3.14f);       // ✗ Wrong - mixed types
```

---

## Tips for Error-Free Code

1. **Use an editor with syntax highlighting** - It helps you spot errors visually
2. **Check semicolons first** - Most errors are missing semicolons
3. **Pay attention to float literals** - Don't forget the 'f' suffix
4. **Count parentheses and brackets** - Every ( needs a ), every [ needs a ]
5. **Declare before using** - Always declare variables before you use them
6. **Check function names** - Use only the available built-in functions
7. **Match argument types** - Functions require specific types

---

## Testing Your Code

The compiler provides a test script to verify all error detection:

```bash
bash test_compiler_errors.sh
```

This runs 5 tests demonstrating:
1. Missing semicolon detection
2. Float literal format detection
3. Unmatched parenthesis detection
4. Valid compilation (no errors)
5. Unmatched bracket detection

---

## Getting Help

If you see an error message:

1. **Read the line number** - Find the exact line with the problem
2. **Read the hint** - The hint explains what went wrong
3. **Check the example** - The message shows the correct format
4. **Refer to this guide** - Look up the error type above

Remember: **Error messages are your friends!** They tell you exactly what's wrong so you can fix it quickly.

