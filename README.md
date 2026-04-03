# 🚀 FluxSharp Programming Language

**FluxSharp** is a modern, expressive programming language compiled to x86-64 assembly. It features clean syntax, object-oriented programming, and rich standard library functions.

## ✨ Features

- **Object-Oriented** - Classes, methods, properties
- **Type Safe** - Static typing (int, float, double, string, bool, etc.)
- **Math Functions** - abs, max, min, pow, sqrt, floor, ceil, round
- **Control Flow** - if/else, for, while with break/continue
- **Arrays** - Fixed-size arrays with indexing
- **Functions** - User-defined and built-in
- **Clear Errors** - Helpful compiler error messages in English

## 🏃 Quick Start (5 minutes)

### 1. Build the Compiler
```bash
cd flux_compiler
cargo build --release
cd ..
```

### 2. Create Your Program
Create `program.fsh`:
```rust
class Main {
    public void main() {
        int result = max(10, 20);
        print(result);  // Output: 20
    }
}
```

### 3. Compile and Run
```bash
./build.sh program.fsh
```

## 📚 Documentation

### 🎯 **New: Organized Documentation System**
📖 **[Complete Documentation Index](docs/INDEX.md)** - Start here for navigation!

Our documentation is now organized into **5 categories** for easy navigation:

#### 🚀 **[Quick Start](docs/01-quickstart/)** (5-10 min read)
Get started in minutes:
- [README.md](docs/01-quickstart/README.md) - Overview and first steps
- [QUICKSTART.md](docs/01-quickstart/QUICKSTART.md) - Quick reference guide

#### 📖 **[Language Reference](docs/02-language/)** (45-60 min read)
Learn FluxSharp syntax and features:
- [SYNTAX.md](docs/02-language/SYNTAX.md) - Complete syntax guide
- [TYPES.md](docs/02-language/TYPES.md) - Type system
- [VARIABLES.md](docs/02-language/VARIABLES.md) - Variables and declarations
- [OPERATORS.md](docs/02-language/OPERATORS.md) - All operators
- [FUNCTIONS.md](docs/02-language/FUNCTIONS.md) - Function definitions and calls
- [CLASSES.md](docs/02-language/CLASSES.md) - Object-oriented programming
- [ARRAYS.md](docs/02-language/ARRAYS.md) - Array handling
- [CONTROL_FLOW.md](docs/02-language/CONTROL_FLOW.md) - If/else, loops, conditions

#### 🎓 **[Advanced Topics](docs/03-advanced/)** (20-30 min read)
Deep dive into advanced features:
- [ASYNC_AWAIT.md](docs/03-advanced/ASYNC_AWAIT.md) - Asynchronous programming
- [CSHARP_IMPORTS_UPDATE.md](docs/03-advanced/CSHARP_IMPORTS_UPDATE.md) - Import system and modules

#### 🔧 **[Compiler Guide](docs/04-compiler/)** (25-35 min read)
How to use and understand the compiler:
- [BUILD_SYSTEM.md](docs/04-compiler/BUILD_SYSTEM.md) - Build system and compilation
- [ERROR_GUIDE.md](docs/04-compiler/ERROR_GUIDE.md) - Debugging guide
- [COMPILER_ERRORS.md](docs/04-compiler/COMPILER_ERRORS.md) - Error messages explained
- [ERROR_MESSAGES_IMPLEMENTATION.md](docs/04-compiler/ERROR_MESSAGES_IMPLEMENTATION.md) - Implementation details
- [INCLUDES_AND_MAIN.md](docs/04-compiler/INCLUDES_AND_MAIN.md) - Include processing and main class

#### 📋 **[Reference & Reports](docs/05-reference/)** (30-45 min read)
Technical reference and project reports:
- [STDLIB.md](docs/05-reference/STDLIB.md) - Standard library functions
- [BEFORE_AFTER_COMPARISON.md](docs/05-reference/BEFORE_AFTER_COMPARISON.md) - Version comparison
- [CHANGES_SUMMARY.md](docs/05-reference/CHANGES_SUMMARY.md) - Change history
- [IMPLEMENTATION_SUMMARY.md](docs/05-reference/IMPLEMENTATION_SUMMARY.md) - Technical implementation
- [VERIFICATION_REPORT.md](docs/05-reference/VERIFICATION_REPORT.md) - Verification and testing results

### 📊 Quick Links by Task

| I want to... | Read this |
|-------------|-----------|
| **Get started** | [01-quickstart/README.md](docs/01-quickstart/README.md) |
| **Learn syntax** | [02-language/SYNTAX.md](docs/02-language/SYNTAX.md) |
| **Understand types** | [02-language/TYPES.md](docs/02-language/TYPES.md) |
| **Define functions** | [02-language/FUNCTIONS.md](docs/02-language/FUNCTIONS.md) |
| **Use classes** | [02-language/CLASSES.md](docs/02-language/CLASSES.md) |
| **Work with arrays** | [02-language/ARRAYS.md](docs/02-language/ARRAYS.md) |
| **Control flow** | [02-language/CONTROL_FLOW.md](docs/02-language/CONTROL_FLOW.md) |
| **Fix errors** | [04-compiler/ERROR_GUIDE.md](docs/04-compiler/ERROR_GUIDE.md) |
| **Use imports** | [03-advanced/CSHARP_IMPORTS_UPDATE.md](docs/03-advanced/CSHARP_IMPORTS_UPDATE.md) |
| **Async programming** | [03-advanced/ASYNC_AWAIT.md](docs/03-advanced/ASYNC_AWAIT.md) |
| **See all changes** | [05-reference/CHANGES_SUMMARY.md](docs/05-reference/CHANGES_SUMMARY.md) |
| **Standard library** | [05-reference/STDLIB.md](docs/05-reference/STDLIB.md) |

### 🛤️ Suggested Reading Paths

**For Beginners (1.5 hours):**
1. [01-quickstart/README.md](docs/01-quickstart/README.md)
2. [02-language/SYNTAX.md](docs/02-language/SYNTAX.md)
3. [02-language/FUNCTIONS.md](docs/02-language/FUNCTIONS.md)
4. [02-language/CLASSES.md](docs/02-language/CLASSES.md)
5. [04-compiler/BUILD_SYSTEM.md](docs/04-compiler/BUILD_SYSTEM.md)

**For Intermediate Users (2 hours):**
1. [02-language/CONTROL_FLOW.md](docs/02-language/CONTROL_FLOW.md)
2. [02-language/ARRAYS.md](docs/02-language/ARRAYS.md)
3. [02-language/OPERATORS.md](docs/02-language/OPERATORS.md)
4. [03-advanced/CSHARP_IMPORTS_UPDATE.md](docs/03-advanced/CSHARP_IMPORTS_UPDATE.md)
5. [04-compiler/ERROR_GUIDE.md](docs/04-compiler/ERROR_GUIDE.md)

**For Advanced Users (2.5 hours):**
1. [05-reference/IMPLEMENTATION_SUMMARY.md](docs/05-reference/IMPLEMENTATION_SUMMARY.md)
2. [05-reference/VERIFICATION_REPORT.md](docs/05-reference/VERIFICATION_REPORT.md)
3. [05-reference/CHANGES_SUMMARY.md](docs/05-reference/CHANGES_SUMMARY.md)
4. [04-compiler/COMPILER_ERRORS.md](docs/04-compiler/COMPILER_ERRORS.md)
5. [03-advanced/ASYNC_AWAIT.md](docs/03-advanced/ASYNC_AWAIT.md)

## 📂 Project Layout

```
FluxSharp/
├── main.fsh                       # Example program
├── build.sh                       # Build script
├── README.md                      # This file
├── LICENSE
│
├── flux_compiler/                 # Rust compiler source
│   ├── fluxc/src/
│   │   ├── main.rs               # Compiler implementation  
│   │   ├── error_handler.rs
│   │   └── flux_grammar.pest
│   └── target/release/fluxc       # Compiled compiler
│
├── docs/                          # Complete Documentation (22 files)
│   ├── INDEX.md                   # 📖 START HERE - Documentation index
│   │
│   ├── 01-quickstart/             # 🚀 Get started in 5-10 minutes
│   │   ├── README.md
│   │   └── QUICKSTART.md
│   │
│   ├── 02-language/               # 📖 Language reference (8 guides)
│   │   ├── SYNTAX.md
│   │   ├── TYPES.md
│   │   ├── VARIABLES.md
│   │   ├── OPERATORS.md
│   │   ├── FUNCTIONS.md
│   │   ├── CLASSES.md
│   │   ├── ARRAYS.md
│   │   └── CONTROL_FLOW.md
│   │
│   ├── 03-advanced/               # 🎓 Advanced topics (2 guides)
│   │   ├── ASYNC_AWAIT.md
│   │   └── CSHARP_IMPORTS_UPDATE.md
│   │
│   ├── 04-compiler/               # 🔧 Build & compiler (5 guides)
│   │   ├── BUILD_SYSTEM.md
│   │   ├── ERROR_GUIDE.md
│   │   ├── COMPILER_ERRORS.md
│   │   ├── ERROR_MESSAGES_IMPLEMENTATION.md
│   │   └── INCLUDES_AND_MAIN.md
│   │
│   └── 05-reference/              # 📋 Technical reference (5 guides)
│       ├── STDLIB.md
│       ├── BEFORE_AFTER_COMPARISON.md
│       ├── CHANGES_SUMMARY.md
│       ├── IMPLEMENTATION_SUMMARY.md
│       └── VERIFICATION_REPORT.md
│
└── examples/                      # Example programs
    ├── hello.fsh
    ├── calculator.fsh
    ├── arrays.fsh
    └── ... (more examples)
```

## 🎯 Basic Examples

### Hello World
```rust
class Main {
    public void main() {
        print("Hello, World!");
    }
}
```

### Variables
```rust
int count = 42;
float price = 19.99f;
double ratio = 3.14159;
string name = "FluxSharp";
bool active = true;
```

### Functions
```rust
public int multiply(int a, int b) {
    return a * b;
}
```

### Classes
```rust
class Calculator {
    int total = 0;
    
    public void add(int value) {
        total = total + value;
    }
    
    public int getTotal() {
        return total;
    }
}
```

### Loops
```rust
// For loop
for (int i = 0; i < 10; i = i + 1) {
    print(i);
}

// While loop
while (x > 0) {
    x = x - 1;
}
```

### Arrays
```rust
int numbers[5];
numbers[0] = 10;
numbers[1] = 20;
print(numbers[0]);  // Output: 10
```

### Math Functions
```rust
int abs_value = abs(-42);      // 42
int maximum = max(10, 20);     // 20
int minimum = min(10, 20);     // 10
int power = pow(2, 3);         // 8
double root = sqrt(16.0);      // 4.0
```

## 🔧 Build System

The `build.sh` script handles everything:

```bash
# Build and run main.fsh
./build.sh

# Build and run any file
./build.sh examples/calculator.fsh
```

The script:
1. Builds the Rust compiler (first time only)
2. Compiles .fsh to assembly
3. Assembles with NASM
4. Links into executable
5. Runs the program

## ⚠️ Common Errors

### Missing Semicolon
```
❌ WRONG:
int x = 10
print(x);

✅ CORRECT:
int x = 10;
print(x);
```

### Float Format
```
❌ WRONG:
float f = 3.14;

✅ CORRECT:
float f = 3.14f;
```

### Undefined Variable
```
❌ WRONG:
print(x);           // x not declared

✅ CORRECT:
int x = 10;
print(x);
```

**See [04-compiler/ERROR_GUIDE.md](docs/04-compiler/ERROR_GUIDE.md) for all error types.**

## 📖 Type System

| Type | Example | Notes |
|------|---------|-------|
| `int` | `42` | 64-bit integer |
| `float` | `3.14f` | 32-bit, must have 'f' suffix |
| `double` | `3.14` | 64-bit floating point |
| `string` | `"hello"` | Text |
| `bool` | `true` | Boolean |
| `byte` | `255` | Single byte |
| `long` | `999999` | Large integer |

## 🧪 Test Your Code

### Run Examples
```bash
./build.sh examples/hello.fsh
./build.sh examples/calculator.fsh
./build.sh examples/arrays.fsh
```

### Run Tests
```bash
# Compile main.fsh and see all features
./build.sh main.fsh
```

## 🔍 Compiler Features

### Error Detection
- Missing semicolons ✅
- Float literal format ✅
- Unmatched parentheses ✅
- Unmatched brackets ✅
- Undefined variables ✅
- Type mismatches ✅

### All errors include:
- ✅ Exact line number
- ✅ Clear explanation
- ✅ Hint on how to fix
- ✅ Example of correct syntax

## 📝 File Extensions

- `.fsh` - FluxSharp source code
- `.asm` - Generated assembly (x86-64)
- `.o` - Compiled object files
- (no extension) - Executable binary

## 🤝 Language Design

FluxSharp aims to be:

- **Easy to Learn** - Clear syntax similar to Java/C#
- **Type Safe** - Compile-time type checking
- **Efficient** - Compiles to native x86-64
- **Helpful** - Great error messages
- **Complete** - Rich standard library

## 📄 License

MIT License - See [LICENSE](LICENSE)

## 🚀 Next Steps

1. **Navigate** - Start with [docs/INDEX.md](docs/INDEX.md) for the complete documentation structure
2. **Read** - Choose your path: [Beginner](docs/01-quickstart/README.md) | [Intermediate](docs/02-language/OPERATORS.md) | [Advanced](docs/05-reference/IMPLEMENTATION_SUMMARY.md)
3. **Try** - Modify `main.fsh` and run `./build.sh`
4. **Explore** - Check out `examples/` directory
5. **Learn** - Read language guides in `docs/02-language/`
6. **Build** - Create your own FluxSharp programs!

---

**Happy coding!** 🎉

📖 **[Start with the Documentation Index](docs/INDEX.md)** for easy navigation through all 22 documentation files organized in 5 categories!

