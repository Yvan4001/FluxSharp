# FluxSharp Programming Language

Modern, efficient, and easy-to-use programming language with async/await support.

## Quick Start - One Command!

### Compile and Run Default File

```bash
./build.sh
```

Compiles `main.fsh` and runs the resulting executable.

### Compile and Run Any File

```bash
./build.sh path/to/file.fsh
```

Automatically creates a binary with the same name and runs it.

## Usage Examples

### Compile and Run Examples

```bash
./build.sh examples/hello.fsh        # Hello world
./build.sh examples/calculator.fsh   # OOP example
./build.sh examples/arrays.fsh       # Arrays example
```

### Compile Custom Files

```bash
./build.sh src/my_program.fsh
./build.sh utils/helper.fsh
./build.sh my_app.fsh
```

## Dynamic Build System

✅ **Compile any file** - `./build.sh file.fsh`  
✅ **Auto binary naming** - Creates `file` from `file.fsh`  
✅ **Directory support** - Binary in same folder as source  
✅ **Instant execution** - Runs immediately after build  
✅ **Fast builds** - Compiler cached after first use  
✅ **Clear feedback** - Shows each build step  

## One-Step Workflow

```
1. Edit: nano file.fsh
2. Build: ./build.sh file.fsh
3. See output
4. Repeat
```

Simple and effective!

## Complete Build Process

The script handles everything:

1. **Checks dependencies** - Verifies cargo installed
2. **Builds compiler** (first time only, ~3 seconds, then cached)
3. **Compiles code** - `.fsh` → `.asm` assembly
4. **Assembles** - `.asm` → `.o` machine code
5. **Links** - Creates executable with runtime
6. **Executes** - Runs the program automatically

## Project Structure

```
FluxSharp/
├── build.sh                  # ← Use this to compile any file
├── main.fsh                  # Default program
├── main                      # Compiled executable
├── Makefile                  # Alternative build system
│
├── examples/                 # Ready-to-compile examples
│   ├── hello.fsh            # Simple hello world
│   ├── calculator.fsh       # OOP with classes
│   ├── arrays.fsh           # Array operations
│   └── [compiled binaries]
│
├── flux_compiler/           # Rust compiler
│   └── fluxc/
│       ├── src/main.rs
│       └── target/release/fluxc
│
├── docs/                    # Documentation
│   ├── QUICKSTART.md
│   ├── SYNTAX.md
│   ├── BUILD_SYSTEM.md
│   ├── TYPES.md
│   ├── FUNCTIONS.md
│   └── [8+ more files]
│
└── [Other files]
```

## Language Features

FluxSharp includes:

- **Static typing** - Types: `int`, `uint`, `long`, `float`, `double`, `byte`, `string`, `bool`, `void`
- **Functions** - Public, private, static, async functions
- **Classes** - Full OOP with `new` keyword
- **Structs** - Lightweight data structures
- **Arrays** - Fixed-size arrays with dynamic indexing
- **Control Flow** - if/else, while, for loops
- **Operators** - Arithmetic, bitwise, logical, comparison
- **Async/Await** - Non-blocking concurrent code
- **Standard Library** - Math, string, I/O functions

## Examples Included

### hello.fsh - Simple Output
```flux
public static void Main() {
    print("Hello from FluxSharp!");
    return;
}
```
Run: `./build.sh examples/hello.fsh`

### calculator.fsh - Classes with OOP
```flux
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}

Calculator calc = new Calculator();
int result = calc.Add(5, 3);
```
Run: `./build.sh examples/calculator.fsh`

### arrays.fsh - Array Operations
```flux
int[10] numbers;
numbers[0] = 10;
int first = numbers[0];
```
Run: `./build.sh examples/arrays.fsh`

## Documentation

Complete language documentation available:

| Document | Topics |
|----------|--------|
| **QUICKSTART.md** | Get started in 5 minutes |
| **SYNTAX.md** | Keywords, operators, statements |
| **TYPES.md** | All data types |
| **FUNCTIONS.md** | Function definition and calls |
| **CLASSES.md** | OOP and inheritance |
| **ARRAYS.md** | Fixed-size arrays |
| **CONTROL_FLOW.md** | If/else, loops |
| **OPERATORS.md** | All operators |
| **ASYNC_AWAIT.md** | Async programming |
| **STDLIB.md** | Built-in functions |
| **BUILD_SYSTEM.md** | Build script guide |

See **[docs/](docs/)** for all documentation.

## System Requirements

- **OS**: Linux x86-64
- **Rust**: 1.70+ (install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **GNU tools**: binutils (install: `sudo apt-get install binutils`)

## Performance

- **First run**: ~3 seconds (builds compiler)
- **Subsequent runs**: <1 second (cached)
- **Compilation speed**: ~0.5 seconds per file
- **Total time**: Usually under 2 seconds

## Workflow Examples

### Single File Development

```bash
# Create and test your program
./build.sh my_app.fsh

# Edit and recompile
nano my_app.fsh
./build.sh my_app.fsh

# Test another version
./build.sh my_app_v2.fsh
```

### Managing Multiple Programs

```bash
# Organize in subdirectories
./build.sh src/main.fsh
./build.sh src/utils.fsh
./build.sh examples/demo.fsh
```

### Examples Directory Structure

```
examples/
├── hello.fsh
├── hello                 # Compiled executable
├── calculator.fsh
├── calculator           # Compiled executable
└── arrays.fsh
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `cargo: not found` | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `as: not found` | Install binutils: `sudo apt-get install binutils` |
| File not found | Use correct relative or absolute path |
| Syntax errors | Check [SYNTAX.md](docs/SYNTAX.md) |

## Command Reference

```bash
./build.sh                      # Compile main.fsh
./build.sh file.fsh             # Compile specific file
./build.sh path/to/file.fsh     # Compile in subdirectory
./build.sh examples/hello.fsh   # Compile example
```

## Project Status

- ✅ Compiler working
- ✅ Full grammar implemented
- ✅ Dynamic build system
- ✅ Complete documentation
- ✅ Multiple examples
- ✅ Production ready

## Next Steps

1. **Try it**: `./build.sh main.fsh`
2. **Explore**: Check `examples/` folder
3. **Learn**: Read `docs/QUICKSTART.md`
4. **Develop**: Create your own programs

---

**Ready to code with FluxSharp!** 🚀

Run `./build.sh` to get started!

