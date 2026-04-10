# FluxSharp v0.1.0 - Release Documentation

Welcome to **FluxSharp**, a modern compiled language designed for security, performance, and safety.

## 📦 Package Contents

```
fluxsharp-v0.1.0-linux-x64/
├── bin/
│   └── fluxc                 # FluxSharp Compiler (pre-compiled)
├── lib/
│   └── runtime.o             # Pre-compiled Runtime Library
├── include/
│   └── core.fsh              # Core Library Definitions
├── examples/                 # Example Programs
├── docs/                     # Full Documentation
└── build.sh                  # Build Script (No Cargo Required!)
```

## 🚀 Quick Start

### 1. Create a FluxSharp Program

Create `hello.fsh`:
```flux
public class Main {
    public void main() {
        print("Hello, FluxSharp!");
    }
}
```

### 2. Compile and Run

```bash
./build.sh hello.fsh
```

### 3. Compile to Binary (without running)

```bash
./build.sh hello.fsh -o hello_binary
./hello_binary
```

## ✨ Key Features

### Security First 🔒
- **Bounds Checking**: All array accesses are validated
- **Null Safety**: Prevents null pointer dereferences
- **Type Safety**: Strict type checking
- **Overflow Detection**: Integer overflow protection
- **Path Security**: Prevents directory traversal attacks

### Performance ⚡
- Compiles directly to x86-64 Assembly
- No Virtual Machine overhead
- Zero-cost abstractions
- Native binary execution

### Modern Language Features
- **Object-Oriented**: Classes and inheritance
- **Static Typing**: Compile-time type checking
- **Built-in Functions**: Math, String, I/O operations
- **Memory Safe**: Automatic safety checks

## 📚 Language Features

### Basic Types
```flux
int x = 42;                    // 64-bit signed integer
float f = 3.14f;              // 32-bit floating point
double d = 2.71828;           // 64-bit floating point
string s = "Hello";           // Null-terminated string
bool b = true;                // Boolean value
```

### Classes and Methods
```flux
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}

public class Main {
    public void main() {
        Calculator calc = new Calculator();
        int result = calc.Add(10, 20);
        print(result);          // Output: 30
    }
}
```

### Arrays
```flux
public class Main {
    public void main() {
        int[10] numbers;
        numbers[0] = 42;
        print(numbers[0]);      // Output: 42
    }
}
```

### Control Flow
```flux
// If/Else
if (x > 0) {
    print("Positive");
} else {
    print("Non-positive");
}

// For Loop
for (int i = 0; i < 10; i = i + 1) {
    print(i);
}

// While Loop
while (x > 0) {
    print(x);
    x = x - 1;
}
```

### Built-in Functions

#### Math
```flux
int result = abs(-42);           // 42
int power = pow(2, 8);           // 256
int maximum = max(10, 20);       // 20
int minimum = min(10, 20);       // 10
```

#### String Operations
```flux
string text = "Hello";
int length = string_length(text);  // 5
```

#### Type Conversion
```flux
string text = "42".ToString();
```

## 🔧 System Requirements

### Runtime Requirements (Minimal)
- Linux x86-64
- glibc (standard C library)
- No Rust installation needed!
- No JVM, no interpreter

### Build Requirements
- `nasm` - Assembler
- `ld` - Linker (part of binutils)

Install on Ubuntu/Debian:
```bash
sudo apt install nasm binutils
```

## 📖 Full Documentation

For complete language documentation, see:
- `docs/LANGUAGE.md` - Language reference
- `docs/SECURITY.md` - Security features
- `docs/PERFORMANCE.md` - Performance guidelines
- `docs/EXAMPLES.md` - Code examples

## 🎯 Integration with FluxGridOS

### For OS Kernel Integration:

1. **Compile user programs**:
   ```bash
   fluxc compile user_program.fsh -o program.o
   ```

2. **Use in kernel build**:
   ```bash
   ld -r kernel.o program.o -o combined.o
   ```

3. **Link with kernel runtime**:
   ```bash
   ld combined.o kernel_runtime.o -o kernel
   ```

The `.o` (object) files are relocatable and can be easily linked with custom kernel runtimes.

## 🐛 Troubleshooting

### "nasm not found"
```bash
sudo apt install nasm
```

### "ld not found"
```bash
sudo apt install binutils
```

### Build fails with "Symbol not defined"
Ensure all class methods are properly defined in your source file.

### Program produces wrong output
Check for:
- Missing semicolons
- Incorrect array bounds
- Type mismatches

## 📜 License

FluxSharp is released under the MIT License.

## 🤝 Contributing

Report issues or suggest features through the project repository.

---

**FluxSharp v0.1.0** - Built for Safety, Speed, and Security

