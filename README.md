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

**Start Here:**
- 📖 [QUICKSTART.md](docs/QUICKSTART.md) - 5-minute tutorial
- ❓ [ERROR_GUIDE.md](docs/ERROR_GUIDE.md) - Understanding errors

**Language Reference:**
- 🔤 [SYNTAX.md](docs/SYNTAX.md) - Language syntax
- 📋 [TYPES.md](docs/TYPES.md) - All data types
- 📦 [VARIABLES.md](docs/VARIABLES.md) - Variables and constants
- ⚙️ [OPERATORS.md](docs/OPERATORS.md) - Operators guide
- 📜 [FUNCTIONS.md](docs/FUNCTIONS.md) - Function definitions
- 🏛️ [CLASSES.md](docs/CLASSES.md) - OOP with classes
- 🔄 [CONTROL_FLOW.md](docs/CONTROL_FLOW.md) - Loops and conditions
- 📊 [ARRAYS.md](docs/ARRAYS.md) - Array operations
- 🔧 [STDLIB.md](docs/STDLIB.md) - Built-in functions
- ⏱️ [ASYNC_AWAIT.md](docs/ASYNC_AWAIT.md) - Async programming

## 📂 Project Layout

```
FluxSharp/
├── main.fsh                 # Example program
├── build.sh                 # Build script
├── README.md               # This file
├── LICENSE
│
├── flux_compiler/          # Rust compiler source
│   ├── fluxc/src/
│   │   ├── main.rs        # Compiler implementation  
│   │   ├── error_handler.rs
│   │   └── flux_grammar.pest
│   └── target/release/fluxc  # Compiled compiler
│
├── docs/                   # Documentation (13 guides)
│   ├── QUICKSTART.md
│   ├── ERROR_GUIDE.md
│   └── ... (more guides)
│
└── examples/               # Example programs
    ├── hello.fsh
    ├── calculator.fsh
    └── arrays.fsh
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

**See [ERROR_GUIDE.md](docs/ERROR_GUIDE.md) for all error types.**

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

1. **Read** - Start with [QUICKSTART.md](docs/QUICKSTART.md)
2. **Try** - Modify `main.fsh` and run `./build.sh`
3. **Explore** - Check out `examples/` directory
4. **Learn** - Read language guides in `docs/`
5. **Build** - Create your own FluxSharp programs!

---

**Happy coding!** 🎉

For questions or issues, check the [documentation](docs/) first.

