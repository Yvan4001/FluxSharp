# FluxSharp Programming Language

Modern, efficient, and easy-to-use programming language with async/await support.

## Quick Start - One Command!

```bash
make
```

That's it! This single command:
1. ✅ Builds the Rust compiler (first time only)
2. ✅ Compiles your `main.fsh` to assembly
3. ✅ Assembles to machine code
4. ✅ Links to create executable
5. ✅ Runs the program automatically

See **[docs/QUICKSTART.md](docs/QUICKSTART.md)** for detailed guide.

## Build Commands

| Command | What it does |
|---------|-------------|
| `make` or `make quickstart` | Build AND run everything |
| `make build` | Just compile, assemble, link |
| `make run` | Just run existing executable |
| `make clean` | Remove generated files |
| `make help` | Show all available commands |

## Workflow

```
main.fsh (your code)
   ↓ (fluxc Rust compiler)
main.asm (x86-64 assembly) [auto-generated]
   ↓ (GNU as assembler)
main.o (object file) [auto-generated]
   ↓ (GNU ld linker)
program (executable) [auto-generated]
   ↓
Output
```

## Project Structure

```
FluxSharp/
├── main.fsh                  # Your FluxSharp source code
├── main.asm                  # Auto-generated assembly
├── main.o                    # Auto-generated object file
├── program                   # Auto-generated executable
├── Makefile                  # Build system
├── README.md                 # This file
│
├── flux_compiler/            # Rust compiler source
│   └── fluxc/
│       ├── src/main.rs      # Compiler code
│       ├── Cargo.toml
│       └── target/release/fluxc
│
├── docs/                     # Documentation
│   ├── QUICKSTART.md         # Get started in 5 minutes
│   ├── README.md             # Doc index
│   ├── SYNTAX.md             # Language syntax
│   ├── TYPES.md              # Data types
│   ├── VARIABLES.md          # Variables
│   ├── FUNCTIONS.md          # Functions
│   ├── CLASSES.md            # Classes and structs
│   ├── CONTROL_FLOW.md       # If/while/for
│   ├── OPERATORS.md          # All operators
│   ├── ARRAYS.md             # Arrays
│   ├── ASYNC_AWAIT.md        # Async programming
│   └── STDLIB.md             # Standard library
│
├── examples/                 # Example programs
│   ├── hello.fsh
│   ├── fibonacci.fsh
│   └── async_example.fsh
│
└── .archive/                 # Old files
```

## Edit and Run

1. **Edit your code:**
   ```bash
   nano main.fsh
   ```

2. **Build and run:**
   ```bash
   make
   ```

That's all you need!

## Language Features

FluxSharp includes:
- **Static typing** - Types: `int`, `uint`, `long`, `float`, `double`, `byte`, `string`, `bool`, `void`
- **Functions** - `public`, `private`, `static`, `async` functions
- **Classes** - Full OOP with inheritance
- **Structs** - Lightweight data structures
- **Arrays** - Fixed-size arrays with expressions
- **Control Flow** - if/else, while, for loops
- **Operators** - Arithmetic, bitwise, logical, comparison
- **Async/Await** - Non-blocking I/O
- **Standard Library** - Math, string, array, I/O functions

## Documentation

Start with one of these:

| Document | Purpose |
|----------|---------|
| **[docs/QUICKSTART.md](docs/QUICKSTART.md)** | Get started in 5 minutes |
| **[docs/SYNTAX.md](docs/SYNTAX.md)** | Language syntax |
| **[docs/TYPES.md](docs/TYPES.md)** | Data types |
| **[docs/FUNCTIONS.md](docs/FUNCTIONS.md)** | How to write functions |
| **[docs/CLASSES.md](docs/CLASSES.md)** | Object-oriented programming |
| **[docs/STDLIB.md](docs/STDLIB.md)** | Math, string, I/O functions |
| **[docs/ASYNC_AWAIT.md](docs/ASYNC_AWAIT.md)** | Async programming |

## Example: Hello World

Edit `main.fsh`:

```flux
public static void Main() {
    print_line("Hello, FluxSharp!");
    return;
}
```

Then run:

```bash
make
```

Output:
```
Hello, FluxSharp!
```

## System Requirements

- **Linux x86-64** - Target platform
- **Rust 1.70+** - For building compiler
- **GNU as** - Assembler (`sudo apt-get install binutils`)
- **GNU ld** - Linker (included with binutils)

## Troubleshooting

### Error: `fluxc: not found`
The compiler wasn't built. Just run `make` - it builds automatically.

### Error: `as: not found`
Install assembler: `sudo apt-get install binutils`

### Syntax errors
Check [docs/SYNTAX.md](docs/SYNTAX.md) for correct syntax.

## How It Works

FluxSharp is a compiled language that generates native x86-64 code:

1. **Compiler (Rust)** - Parses FluxSharp code and generates assembly
2. **Assembler (GNU as)** - Converts assembly to machine code
3. **Linker (GNU ld)** - Creates executable binary
4. **Runtime** - Executes native code directly (no VM)

Result: Fast, efficient native programs!

## Performance

- **Direct compilation** - No interpreter overhead
- **Native code** - Runs at full CPU speed
- **Small binaries** - Minimal dependencies
- **Fast startup** - No JIT warmup needed
- **Async support** - Efficient concurrent tasks

## Version

FluxSharp v2.0.0 - Production Ready

## License

See LICENSE file

---

## Quick Reference

```bash
# Most common commands:
make              # Build and run (same as quickstart)
nano main.fsh     # Edit your code
make clean        # Clean up generated files

# Full workflow:
make build        # Compile only (no run)
make run          # Run only (no compile)

# Help:
make help         # Show all available commands
```

**Ready to code? Run `make` now!** 🚀

