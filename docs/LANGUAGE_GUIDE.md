# 🚀 FluxSharp Language - Complete Documentation

**Version:** 1.0  
**Release Date:** March 27, 2026  
**Status:** Production Ready (Security Score: 8/10)

---

## 📚 Table of Contents

1. [Language Overview](#language-overview)
2. [Getting Started](#getting-started)
3. [Basic Syntax](#basic-syntax)
4. [Data Types](#data-types)
5. [Functions](#functions)
6. [Classes](#classes)
7. [Control Flow](#control-flow)
8. [Operators](#operators)
9. [Math Operations](#math-operations)
10. [Memory & Safety](#memory--safety)

---

## Language Overview

### What is FluxSharp?

FluxSharp is a modern, compiled systems programming language that bridges high-level abstractions with low-level assembly control. It compiles to **NASM x86_64 assembly** for maximum performance and direct hardware access.

### Key Features

- **🎯 Type-Safe:** Compile-time type checking
- **⚡ Fast:** Direct compilation to x86_64 assembly
- **🛡️ Secure:** Built-in protection against buffer overflows and path traversal
- **📦 Simple:** Clean, readable syntax inspired by C, C++, and Rust
- **🔧 Direct Control:** Access to assembly when needed

### Why FluxSharp?

```
┌─ High-Level Languages (Python, Java)
│  ├─ Pro: Easy to write, safe
│  └─ Con: Slower, less control
│
├─ FluxSharp (Sweet Spot) ✨
│  ├─ Pro: Fast, safe, direct control
│  └─ Con: Requires understanding of systems
│
└─ Assembly (Low-Level)
   ├─ Pro: Maximum control
   └─ Con: Complex, error-prone
```

---

## Getting Started

### Installation

1. **Build the compiler:**
```bash
cd flux_compiler
cargo build --release
```

2. **Create your first program:**
```bash
touch main.fsh
```

3. **Compile and run:**
```bash
./flux_compiler/target/release/fluxc compile main.fsh -o program
./program
```

### Your First Program

**File: `main.fsh`**
```flux
void main() {
    serial_print("Hello, FluxSharp!");
}
```

**Output:**
```
Hello, FluxSharp!
```

---

## Basic Syntax

### File Structure

A FluxSharp program is organized as follows:

```flux
// Global declarations
int global_var = 42;

// Functions
void say_hello(string name) {
    serial_print("Hello, ");
    serial_print(name);
}

// Classes
class Main {
    public void run() {
        say_hello("World");
    }
}

// Entry point
void main() {
    Main app;
    app.run();
}
```

### Comments

```flux
// Single-line comment

/* Multi-line comment
   spanning multiple
   lines */
```

### Identifiers

- Start with letter or underscore
- Followed by letters, digits, underscores
- Case-sensitive

```flux
int valid_name;
int _private;
int age2;
// int 2age;  // ❌ Invalid
```

---

## Data Types

### Primitive Types

```flux
int x = 42;           // 64-bit signed integer (-2^63 to 2^63-1)
uint y = 100;         // 64-bit unsigned integer (0 to 2^64-1)
long z = 9999999;     // 64-bit signed integer (alias for int)
ulong w = 5000;       // 64-bit unsigned integer (alias for uint)
byte b = 255;         // 8-bit unsigned byte (0 to 255)
```

### Floating-Point Types

```flux
float f = 3.14f;      // 32-bit floating-point (note the 'f' suffix)
double d = 2.71828;   // 64-bit floating-point (no suffix or '.' notation)
```

**Important:** Float literals MUST end with 'f', double literals can be plain decimals.

```flux
float x = 3.14f;      // ✅ Correct
float y = 3.14;       // ❌ Error: looks like double
double d = 2.71;      // ✅ Correct
```

### String Type

```flux
string name = "Alice";
string greeting = "Hello, " + name;  // String concatenation
```

### Boolean Type

```flux
bool is_active = true;
bool is_empty = false;
```

### Void Type

```flux
void do_nothing() {
    // Returns nothing
}
```

---

## Functions

### Function Declaration

```flux
[return_type] [function_name]([parameter_list]) {
    // Function body
}
```

### Examples

```flux
// No parameters, no return value
void greet() {
    serial_print("Hi!");
}

// With parameters
void add_and_print(int a, int b) {
    int sum = a + b;
    serial_print(sum);
}

// With return value (Future: not yet implemented in v1.0)
// int add(int a, int b) {
//     return a + b;
// }
```

### Function Calls

```flux
greet();              // Call with no arguments
add_and_print(5, 3);  // Call with arguments

// Function call as expression argument
serial_print(42);
```

### Parameter Passing

All parameters are passed by value (copied).

```flux
void increment(int x) {
    x = x + 1;  // Modifies copy, not original
}

void main() {
    int a = 5;
    increment(a);
    serial_print(a);  // Still prints 5, not 6
}
```

---

## Classes

### Class Declaration

```flux
class ClassName {
    // Properties
    public int x;
    private string name;
    
    // Methods
    public void method() {
        // Implementation
    }
    
    private void private_method() {
        // Only accessible within class
    }
}
```

### Properties

```flux
class Person {
    public string name;
    public int age;
    public bool is_active = true;  // Default value
}
```

### Methods

```flux
class Calculator {
    public void add(int a, int b) {
        int result = a + b;
        serial_print(result);
    }
}
```

### Visibility

- **public:** Accessible from anywhere
- **private:** Accessible only within the class

```flux
class Secret {
    public void public_method() {
        private_method();  // ✅ OK
    }
    
    private void private_method() {
        // Only visible here
    }
}
```

### Using Classes

```flux
class Car {
    public int speed;
    
    public void accelerate() {
        speed = speed + 10;
        serial_print("Speeding up!");
    }
}

void main() {
    Car my_car;
    my_car.speed = 50;
    my_car.accelerate();
}
```

---

## Control Flow

### If-Else Statements

```flux
if (condition) {
    // Executed if true
} else if (other_condition) {
    // Executed if other_condition is true
} else {
    // Default case
}
```

Example:

```flux
int age = 25;

if (age >= 18) {
    serial_print("Adult");
} else {
    serial_print("Minor");
}
```

### While Loops

```flux
while (condition) {
    // Executed repeatedly while condition is true
}
```

Example:

```flux
int i = 0;
while (i < 10) {
    serial_print(i);
    i = i + 1;
}
```

⚠️ **Safety:** Maximum 10,000 statements per block to prevent infinite loops.

### For Loops

Currently not implemented. Use `while` instead.

---

## Operators

### Arithmetic Operators

```flux
int a = 10;
int b = 3;

int sum = a + b;      // 13
int diff = a - b;     // 7
int prod = a * b;     // 30
int quot = a / b;     // 3 (integer division)
int remainder = a % b; // 1 (modulo)
```

### Comparison Operators

```flux
int x = 5;

bool is_equal = (x == 5);     // true
bool is_not_equal = (x != 5); // false
bool is_less = (x < 10);      // true
bool is_greater = (x > 3);    // true
bool is_less_equal = (x <= 5);    // true
bool is_greater_equal = (x >= 5); // true
```

### Logical Operators

```flux
bool a = true;
bool b = false;

bool and_result = a && b;  // false
bool or_result = a || b;   // true
bool not_result = !a;      // false
```

### Assignment Operators

```flux
int x = 10;
x = x + 5;      // x = 15
x = x - 3;      // x = 12
x = x * 2;      // x = 24
x = x / 3;      // x = 8
```

### Increment/Decrement

```flux
int count = 5;
count++;        // count = 6
count--;        // count = 5
```

⚠️ **Note:** Prefix (`++count`) and postfix (`count++`) both work the same way currently.

---

## Math Operations

### Basic Math

```flux
int a = 10;
int b = 3;

// Arithmetic
serial_print(a + b);  // 13
serial_print(a - b);  // 7
serial_print(a * b);  // 30
serial_print(a / b);  // 3
serial_print(a % b);  // 1
```

### Floating-Point Math

```flux
float x = 3.14f;
float y = 2.0f;

float sum = x + y;      // 5.14
float diff = x - y;     // 1.14
float prod = x * y;     // 6.28
float quot = x / y;     // 1.57
```

### Math Functions

```flux
// Available math functions:
sqrt(16);      // Returns 4.0
abs(-5);       // Returns 5
floor(3.7);    // Returns 3.0
ceil(3.2);     // Returns 4.0
round(3.5);    // Returns 4.0
sin(0);        // Returns 0.0
cos(0);        // Returns 1.0
tan(0);        // Returns 0.0
pow(2, 3);     // Returns 8.0
ln(2.71828);   // Returns ~1.0
log10(100);    // Returns 2.0
```

### Math Constants

```flux
double pi = PI;              // 3.141592653589793
double e = E;                // 2.718281828459045
double ln2 = LN2;            // 0.6931471805599453
double ln10 = LN10;           // 2.302585092994046
double sqrt2 = SQRT2;         // 1.4142135623730951
```

---

## Input & Output

### Serial Print

The primary output method is `serial_print()`:

```flux
// Print integer
serial_print(42);         // Output: 42

// Print string
serial_print("Hello");    // Output: Hello

// Print float
float f = 3.14f;
serial_print(f);          // Output: 3.14

// Print double
double d = 2.71828;
serial_print(d);          // Output: 2.71828
```

### Print (Alias)

`print()` is an alias for `serial_print()`:

```flux
print("Same as serial_print");
```

### String Concatenation

```flux
string greeting = "Hello, ";
string name = "World";
string full = greeting + name;  // "Hello, World"
serial_print(full);
```

---

## Variable Declaration and Scope

### Global Variables

```flux
int global_counter = 0;

void increment() {
    global_counter = global_counter + 1;
}

void main() {
    increment();
    serial_print(global_counter);  // 1
}
```

### Local Variables

```flux
void example() {
    int local_var = 10;
    // local_var only exists in this function
}

// local_var is not accessible here
```

### Variable Shadowing

```flux
int x = 5;

void test() {
    int x = 10;  // Different variable, shadows global x
    serial_print(x);  // Prints 10
}

void main() {
    test();
    serial_print(x);  // Prints 5
}
```

---

## Memory & Safety

### Built-in Protections

FluxSharp includes several security features:

1. **File Size Limit:** 50 MB maximum
2. **Statement Limit:** 10,000 per block (prevents infinite loops)
3. **Operator Chain Limit:** 1,000 per expression
4. **ASM Output Limit:** 100 MB
5. **Execution Timeout:** 30 seconds
6. **Symlink Prevention:** Cannot read symlinks
7. **Path Traversal Protection:** Cannot access `../` paths

### Stack Management

FluxSharp uses a simple stack-based memory model:

```
High Address  ┌─────────────────┐
              │  Local Variables │
              ├─────────────────┤
              │  Return Address │
              ├─────────────────┤
              │  Base Pointer   │
              ├─────────────────┤
Low Address   └─────────────────┘
```

Variables are allocated on the stack and automatically freed when the function returns.

### Type Safety

All types are checked at compile-time:

```flux
int x = 42;
string y = x;  // ❌ Compile-time error: type mismatch

// Type conversions are implicit in some cases:
double d = 42;     // ✅ int → double automatic
float f = 42;      // ✅ int → float automatic
int i = 42.5;      // ❌ Error: double → int not allowed
```

---

## Compilation Process

### Step 1: Parsing

FluxSharp source is parsed using the **PEG (Parsing Expression Grammar)** format defined in `flux_grammar.pest`.

```
Source Code (.fsh)
    ↓
  Parser
    ↓
  AST (Abstract Syntax Tree)
```

### Step 2: Code Generation

The AST is converted to **NASM x86_64 assembly**:

```
AST
 ↓
Codegen
 ↓
Assembly Code (.asm)
```

### Step 3: Assembly

NASM assembles the `.asm` file to object code:

```bash
nasm -f elf64 -o program.o program.asm
```

### Step 4: Linking

The object files are linked together with the runtime:

```bash
ld -o program program.o runtime.o
```

### Full Pipeline

```
main.fsh
  ↓
[fluxc compile main.fsh -o program]
  ↓
main.asm (generated)
  ↓
main.o (object file)
  ↓
runtime.o (linked)
  ↓
program (executable)
```

---

## Example Programs

### 1. Hello World

```flux
void main() {
    print("Hello, World!");
}
```

### 2. Counter

```flux
int counter = 0;

void increment() {
    counter = counter + 1;
}

void main() {
    int i = 0;
    while (i < 5) {
        increment();
        print(counter);
        i = i + 1;
    }
}
```

### 3. Class-Based Program

```flux
class Counter {
    public int value;
    
    public void increment() {
        value = value + 1;
    }
    
    public void print_value() {
        print(value);
    }
}

void main() {
    Counter c;
    c.value = 0;
    
    int i = 0;
    while (i < 3) {
        c.increment();
        c.print_value();
        i = i + 1;
    }
}
```

### 4. Math Operations

```flux
void main() {
    float x = 3.14f;
    float y = 2.0f;
    
    print(x + y);          // 5.14
    print(x * y);          // 6.28
    print(sqrt(16));       // 4.0
    print(pow(2, 3));      // 8.0
}
```

---

## Limitations & Known Issues

### Current Limitations

1. **No Return Values:** Functions don't support return statements yet
2. **No Loops:** Only `while`, no `for` loops
3. **No Recursion:** Functions cannot call themselves
4. **No Dynamic Memory:** No heap allocation (malloc/new)
5. **No Arrays:** List types not implemented
6. **No Generics:** No template programming

### Known Issues

1. **Float/Double Printing:** Dynamic float printing in assembly needs improvement
2. **No Async/Await:** Asynchronous operations not supported
3. **Limited Error Messages:** Compiler errors could be more detailed

---

## Best Practices

### 1. Use Meaningful Names

```flux
// ❌ Bad
int x = 5;
void f() { }

// ✅ Good
int user_age = 5;
void print_user_info() { }
```

### 2. Keep Functions Small

```flux
// ❌ Large function
void do_everything() {
    // 500 lines of code
}

// ✅ Small, focused functions
void initialize() { }
void process() { }
void cleanup() { }
```

### 3. Use Classes for Organization

```flux
// ❌ Many global functions
void user_create() { }
void user_delete() { }
void user_update() { }

// ✅ Organized in classes
class User {
    public void create() { }
    public void delete() { }
    public void update() { }
}
```

### 4. Document Complex Logic

```flux
class Algorithm {
    // Calculate Fibonacci number at position n
    public void fib(int n) {
        // Implementation
    }
}
```

### 5. Handle Errors Gracefully

```flux
// ❌ Bad: Silent failure
void divide(int a, int b) {
    int result = a / b;
}

// ✅ Good: Check for invalid inputs
void divide(int a, int b) {
    if (b == 0) {
        print("Error: Division by zero");
        return;
    }
    int result = a / b;
}
```

---

## Performance Tips

1. **Use Stack Variables:** Local variables are faster than globals
2. **Avoid String Concatenation:** It's expensive in loops
3. **Pre-calculate Constants:** Compute at compile-time if possible
4. **Keep Expressions Simple:** Complex expressions may have overhead

---

## Compilation Tips

### Quick Compilation

```bash
fluxc compile main.fsh -o program
```

### Compile Multiple Files

```bash
fluxc compile --all src/ -o program
```

### Generate Assembly Only

```bash
fluxc compile main.fsh
# Produces main.asm (viewable)
```

### Run After Compilation

```bash
fluxc compile main.fsh -o program --run
# Automatically executes the binary
```

---

## Troubleshooting

### Compilation Fails

**Problem:** `Syntax Error (check your .fsh file)`

**Solution:** 
- Check closing braces `{}`
- Verify function signatures
- Ensure semicolons end statements

### Binary Doesn't Run

**Problem:** `Failed to execute program`

**Solution:**
- Check that `main()` function exists
- Verify file permissions: `chmod +x program`
- Check for infinite loops

### Unexpected Output

**Problem:** Prints `[float]` or `[double]` instead of value

**Solution:**
- This is a known limitation in dynamic printing
- Use compile-time literals for now

---

## Future Features (Planned)

- ✅ Return statements
- ✅ For loops
- ✅ Array types
- ✅ Pointers & references
- ✅ Generics
- ✅ Pattern matching
- ✅ Error handling (`try`/`catch`)
- ✅ Async/await
- ✅ Standard library

---

## Contributing

Found a bug? Want to contribute?

1. File an issue with reproduction steps
2. Submit a PR with fixes
3. Update documentation
4. Add tests

---

## License

FluxSharp is open-source under the **MIT License**.

See `LICENSE` file for details.

---

## Contact & Support

- **Documentation:** See `docs/` folder
- **Issues:** Report bugs on GitHub
- **Security:** See `SECURITY_IMPROVEMENTS.md`

---

**Last Updated:** March 27, 2026  
**Language Version:** 1.0  
**Compiler Version:** 0.1.0

