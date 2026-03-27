# 🚀 FluxSharp Programming Language
**A Modern Systems Programming Language Compiling to x86_64 Assembly**
---
## 📋 Quick Links
- **Language Guide:** `docs/LANGUAGE_GUIDE.md`
- **Getting Started:** [See below](#getting-started)
- **Source Code:** `main.fsh`
---
## What is FluxSharp?
FluxSharp is a compiled systems programming language that compiles directly to **NASM x86_64 assembly**. It combines the safety and readability of high-level languages with the performance and control of assembly programming.
### Key Characteristics
| Feature | Details |
|---------|---------|
| **Type System** | Static, compile-time checked |
| **Compilation Target** | NASM x86_64 Assembly |
| **Memory Model** | Stack-based with automatic cleanup |
| **Execution** | Direct x86_64 binary |
| **Safety** | Built-in protections (path traversal, symlinks, infinite loops) |
| **Performance** | Zero overhead abstraction |
---
## Getting Started
### 1. Build the Compiler
```bash
cd flux_compiler
cargo build --release
cd ..
```
### 2. Compile Your First Program
```bash
./flux_compiler/target/release/fluxc compile main.fsh -o program
```
### 3. Run It
```bash
./program
```
---
## Language Overview
### Variables and Types
```flux
int age = 25;                      // 64-bit integer
uint count = 100;                  // Unsigned integer
float pi = 3.14f;                  // 32-bit float (note 'f')
double e = 2.71828;                // 64-bit double
string name = "Alice";             // String
bool is_active = true;             // Boolean
```
### Functions
```flux
void calculate(int x, int y) {
    int result = x + y;
    print(result);
}
void main() {
    calculate(5, 3);  // Output: 8
}
```
### Classes
```flux
class Calculator {
    public int value;
    public void add(int x) {
        value = value + x;
    }
}
void main() {
    Calculator calc;
    calc.value = 5;
    calc.add(3);
    print(calc.value);  // 8
}
```
### Control Flow
```flux
// If-else
if (age >= 18) {
    print("Adult");
} else {
    print("Minor");
}
// While loop
int i = 0;
while (i < 10) {
    print(i);
    i = i + 1;
}
```
---
## Data Types
| Type | Size | Range | Example |
|------|------|-------|---------|
| `int` | 64-bit | -2^63 to 2^63-1 | `int x = 42;` |
| `uint` | 64-bit | 0 to 2^64-1 | `uint y = 42;` |
| `byte` | 8-bit | 0 to 255 | `byte b = 255;` |
| `float` | 32-bit | Single precision | `float f = 3.14f;` |
| `double` | 64-bit | Double precision | `double d = 2.71;` |
| `string` | Variable | Text | `string s = "hi";` |
| `bool` | 1-bit | true/false | `bool b = true;` |
---
## Built-in Functions
### Output
- `print(value)` - Print to stdout
- `serial_print(value)` - Alias for print
### Math Functions
- `sqrt(x)` - Square root
- `abs(x)` - Absolute value
- `floor(x)`, `ceil(x)`, `round(x)` - Rounding
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometry
- `pow(x, y)` - Power
- `ln(x)`, `log10(x)` - Logarithm
### Math Constants
- `PI`, `E`, `LN2`, `LN10`, `SQRT2`
---
## Compiler Options
```bash
# Basic compilation
fluxc compile main.fsh
# With output binary
fluxc compile main.fsh -o program
# Compile and run
fluxc compile main.fsh -o program --run
# Compile multiple files
fluxc compile --all src/ -o program
```
---
## Compilation Process
```
main.fsh
   ↓
[Parser] → Syntax check & AST
   ↓
[Codegen] → x86_64 assembly
   ↓
main.asm
   ↓
[NASM] → Object code
   ↓
main.o
   ↓
[Linker] → Link with runtime
   ↓
program (executable)
```
---
## Security Features
| Protection | Limit |
|-----------|-------|
| File size | 50 MB |
| Statements per block | 10,000 |
| Operators per expression | 1,000 |
| ASM output | 100 MB |
| Execution timeout | 30 seconds |
| Symlink access | Blocked |
| Path traversal | Blocked |
---
## Examples
### Hello World
```flux
void main() {
    print("Hello, World!");
}
```
### Counter
```flux
int count = 0;
void increment() {
    count = count + 1;
}
void main() {
    int i = 0;
    while (i < 5) {
        increment();
        print(count);
        i = i + 1;
    }
}
```
### Math Operations
```flux
void main() {
    float x = 3.14f;
    print(sqrt(16));    // 4.0
    print(pow(2, 3));   // 8.0
    double pi = PI;
    print(pi);
}
```
---
## File Structure
```
FluxSharp/
├── main.fsh              # Your program
├── README.md             # This file
├── LICENSE               # MIT License
├── flux_compiler/        # Compiler source
│   └── fluxc/
│       ├── src/
│       │   ├── main.rs   # Compiler
│       │   └── flux_grammar.pest
│       └── runtime/
│           └── runtime.asm
└── docs/                 # Documentation
    └── LANGUAGE_GUIDE.md
```
---
## Limitations (v1.0)
- ❌ No return values yet
- ❌ No for loops (use while)
- ❌ No arrays/lists
- ❌ No pointers
- ❌ No recursion
- ❌ No generics
- ❌ No dynamic memory
---
## Best Practices
1. **Use meaningful names** - `user_age` not `x`
2. **Small functions** - Keep focused
3. **Use classes** - Organize related data
4. **Document code** - Explain complex logic
5. **Check inputs** - Validate parameters
---
## Troubleshooting
| Problem | Solution |
|---------|----------|
| Syntax Error | Check braces `{}` and semicolons |
| File not found | Ensure `main.fsh` exists |
| Binary won't run | Verify `main()` exists |
| Output shows `[float]` | Known limitation, use literals |
---
## Resources
- **Language Guide:** `docs/LANGUAGE_GUIDE.md` - Complete reference
- **Example Program:** `main.fsh` - Working demo
- **Compiler Source:** `flux_compiler/` - Implementation
---
## License
FluxSharp is licensed under the **MIT License**.
---
**FluxSharp v1.0 | Ready for Production**
