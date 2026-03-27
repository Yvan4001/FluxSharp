# 📖 FluxSharp Usage Guide
Complete guide to using the FluxSharp programming language and compiler.
---
## 🚀 Quick Start
### Prerequisites
- Rust (to build the compiler)
- NASM (for assembling x86_64 code)
- GCC or LD (for linking binaries)
- Linux x86_64 system
### Installation
1. **Build the compiler:**
```bash
cd flux_compiler
cargo build --release
cd ..
```
2. **Verify installation:**
```bash
./flux_compiler/target/release/fluxc --help
```
---
## 📝 Basic Usage
### Compile a Program
```bash
# Compile main.fsh into executable binary
./flux_compiler/target/release/fluxc compile main.fsh -o program
# Run the compiled program
./program
```
### Compilation Pipeline
The FluxSharp compiler performs these steps:
1. **Parsing** - Analyzes Flux syntax
2. **Validation** - Type checking and semantic verification
3. **Code Generation** - Generates x86_64 NASM assembly
4. **Assembly** - Assembles to `.o` object files with NASM
5. **Linking** - Links into final executable binary
**Generated files:**
- `main.asm` - x86_64 assembly code
- `main.o` - Compiled object file
- `program` - Final executable binary
---
## 🔧 Compiler Commands
```bash
# Compile a Flux program
./flux_compiler/target/release/fluxc compile <input.fsh> -o <output>
# Display help
./flux_compiler/target/release/fluxc --help
# Show version
./flux_compiler/target/release/fluxc --version
```
---
## 📚 Language Features
### Variables and Types
```flux
int age = 25;                      // 64-bit signed integer
uint count = 100;                  // Unsigned integer
float pi = 3.14f;                  // 32-bit float (note 'f')
double e = 2.71828;                // 64-bit double
string name = "Alice";             // String
bool is_active = true;             // Boolean
```
### Functions
```flux
void greet(string name) {
    serial_print("Hello, ");
    serial_print(name);
}
greet("World");
```
### Control Flow
```flux
// If-else statement
if (age > 18) {
    serial_print("Adult");
} else {
    serial_print("Minor");
}
// While loop
while (i < 10) {
    serial_print(i);
    i = i + 1;
}
// For loop
for (int j = 0; j < 5; j++) {
    serial_print(j);
}
```
### Classes and Structures
```flux
class Point {
    public int x;
    public int y;
}
Point p;
p.x = 10;
p.y = 20;
```
### Arrays
```flux
int arr[10];
arr[0] = 42;
arr[1] = 84;
```
---
## 🔢 Data Types Reference
| Type | Size | Range | Example |
|------|------|-------|---------|
| `int` | 64-bit | -2^63 to 2^63-1 | `int x = 42;` |
| `uint` | 64-bit | 0 to 2^64-1 | `uint x = 100;` |
| `float` | 32-bit | IEEE 754 single | `float x = 3.14f;` |
| `double` | 64-bit | IEEE 754 double | `double x = 3.14;` |
| `string` | Variable | Text | `string s = "Hello";` |
| `bool` | 1-bit | true/false | `bool x = true;` |
| `void` | N/A | No value | Used in functions |
---
## 🧮 Mathematical Functions
### Available Functions
| Function | Purpose | Example |
|----------|---------|---------|
| `sqrt(x)` | Square root | `sqrt(16)` → 4.0 |
| `pow(base, exp)` | Power | `pow(2, 3)` → 8.0 |
| `abs(x)` | Absolute value | `abs(-5)` → 5 |
| `floor(x)` | Floor function | `floor(3.7)` → 3.0 |
| `ceil(x)` | Ceiling function | `ceil(3.2)` → 4.0 |
| `round(x)` | Rounding | `round(3.7)` → 4.0 |
| `sin(x)` | Sine | `sin(PI/2)` → 1.0 |
| `cos(x)` | Cosine | `cos(0)` → 1.0 |
| `tan(x)` | Tangent | `tan(PI/4)` → 1.0 |
| `ln(x)` | Natural logarithm | `ln(E)` → 1.0 |
| `log10(x)` | Base 10 logarithm | `log10(100)` → 2.0 |
### Mathematical Constants
| Constant | Value | Example |
|----------|-------|---------|
| `PI` | 3.141592653589793 | `double angle = PI / 2;` |
| `E` | 2.718281828459045 | `double exp = E;` |
| `LN2` | 0.693147180559945 | ln(2) |
| `LN10` | 2.302585092994046 | ln(10) |
| `SQRT2` | 1.414213562373095 | √2 |
---
## 🎯 Operators
### Arithmetic Operators
```flux
int sum = a + b;        // Addition
int diff = a - b;       // Subtraction
int prod = a * b;       // Multiplication
int quot = a / b;       // Division
int remainder = a % b;  // Modulo
```
### Comparison Operators
```flux
if (a == b) { }         // Equal
if (a != b) { }         // Not equal
if (a < b) { }          // Less than
if (a > b) { }          // Greater than
if (a <= b) { }         // Less than or equal
if (a >= b) { }         // Greater than or equal
```
### Logical Operators
```flux
if (a && b) { }         // Logical AND
if (a || b) { }         // Logical OR
if (!a) { }             // Logical NOT
```
### Assignment Operators
```flux
x = 10;                 // Assignment
x += 5;                 // Add and assign (x = x + 5)
x -= 3;                 // Subtract and assign (x = x - 3)
x *= 2;                 // Multiply and assign (x = x * 2)
x /= 4;                 // Divide and assign (x = x / 4)
```
---
## 🏥 Input/Output
### Printing Output
```flux
// Print integer
serial_print(42);
// Print string
serial_print("Hello World");
// Print double
double x = 3.14;
serial_print(x);
// Print result of expression
serial_print(10 + 5);
```
---
## 🛡️ Security Features
FluxSharp includes built-in security protections:
### Infinite Loop Prevention
The compiler detects and prevents infinite loops:
```flux
// This will be rejected:
while (true) {
    serial_print("Infinite");
}
```
### Path Traversal Protection
File operations are secured against path traversal:
```flux
// This will be rejected:
open("../../../etc/passwd");
```
### Symlink Protection
Symbolic link access is validated.
### Stack Overflow Protection
Automatic protection against stack buffer overflows.
---
## 📊 Examples
### Example 1: Hello World
```flux
void main() {
    serial_print("Hello, World!");
}
```
### Example 2: Arithmetic
```flux
void main() {
    int x = 10;
    int y = 20;
    int sum = x + y;
    serial_print(sum);  // Output: 30
}
```
### Example 3: Loops
```flux
void main() {
    for (int i = 0; i < 5; i++) {
        serial_print(i);
    }
}
```
### Example 4: Functions
```flux
void greet(string name) {
    serial_print("Hello, ");
    serial_print(name);
}
void main() {
    greet("Alice");
    greet("Bob");
}
```
### Example 5: Math Functions
```flux
void main() {
    double result1 = sqrt(16);   // 4.0
    double result2 = pow(2, 3);  // 8.0
    serial_print(result1);
    serial_print(result2);
}
```
### Example 6: Conditionals
```flux
void main() {
    int age = 25;
    if (age >= 18) {
        serial_print("Adult");
    } else {
        serial_print("Minor");
    }
}
```
---
## 🧪 Testing
### Run Simple Tests
```bash
./test_simple
```
### Run Method Tests
```bash
./test_methods
```
### Run Security Tests
```bash
bash security_tests.sh
```
---
## ⚠️ Known Limitations
### 1. Limited Output Display
Some complex values may have limited display:
- Some advanced string operations
- Certain loop counters
**Workaround:** Use intermediate variables for verification.
### 2. Function Return Values
Functions don't return values (planned for v2.0):
```flux
// Not yet supported:
int result = add(5, 3);
```
### 3. No Recursion
Recursive functions are not supported in v1.1.0.
### 4. String Limitations
- Fixed maximum length
- No dynamic concatenation
- Limited string operations
### 5. No Pointers
Pointer types are not supported in v1.x.
---
## 🔍 Troubleshooting
### Compiler not found
```bash
# Check if compiler is built
./flux_compiler/target/release/fluxc --version
# If not found, rebuild:
cd flux_compiler && cargo build --release && cd ..
```
### NASM not installed
```bash
# Check installation
nasm -version
# Install on Ubuntu/Debian
sudo apt-get install nasm
# Install on Fedora/RHEL
sudo dnf install nasm
```
### GCC/Linker errors
```bash
# Check GCC installation
gcc --version
# Install on Ubuntu/Debian
sudo apt-get install gcc
# Install on Fedora/RHEL
sudo dnf install gcc
```
### Program won't execute
```bash
# Make it executable
chmod +x program
# Run directly
./program
```
### Compilation errors
1. Check syntax in your .fsh file
2. Verify all variable types match usage
3. Review LANGUAGE_GUIDE.md for syntax rules
---
## 📁 Project Structure
```
FluxSharp/
├── README.md              # Project overview
├── README-USAGE.md        # This file
├── CHANGELOG.md           # Version history
├── INDEX.md               # Documentation index
├── SUMMARY.txt            # Executive summary
├── docs/
│   ├── LANGUAGE_GUIDE.md
│   ├── SECURITY.md
│   ├── EXAMPLES.md
│   └── QUICKSTART.md
├── main.fsh               # Example program
├── math_demo.fsh          # Math functions demo
└── flux_compiler/
    └── fluxc/
        ├── src/
        │   ├── main.rs
        │   └── flux_grammar.pest
        └── runtime/
            └── runtime.asm
```
---
## 📚 Documentation Links
- **[Language Guide](docs/LANGUAGE_GUIDE.md)** - Complete language reference
- **[Quick Start](docs/QUICKSTART.md)** - Quick start guide
- **[Examples](docs/EXAMPLES.md)** - Advanced examples
- **[Security](docs/SECURITY.md)** - Security features
- **[Index](INDEX.md)** - Documentation navigation
---
## 💡 Tips
1. ✅ Always compile the compiler first with `cargo build --release`
2. ✅ Test with the provided examples before writing new code
3. ✅ Use `serial_print()` for debugging
4. ✅ Check variable types match their usage
5. ✅ Read security documentation before file operations
6. ✅ Run tests before deployment: `./test_simple && ./test_methods`
---
## 🤝 Getting Help
- Review the [Language Guide](docs/LANGUAGE_GUIDE.md)
- Check [Examples](docs/EXAMPLES.md) for code patterns
- Read [Troubleshooting](#-troubleshooting) section
- Consult [Index](INDEX.md) for documentation overview
---
**Last Updated:** March 27, 2026  
**Version:** FluxSharp 1.1.0  
**Status:** Stable & Production-Ready
