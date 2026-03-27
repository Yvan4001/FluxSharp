# FluxSharp Programming Language
Modern, efficient, and easy-to-use programming language with async/await support.
## Quick Start
### Build and Run
```bash
make              # Compile, assemble, and run (default)
make build        # Just build the executable
make run          # Just run the executable
make clean        # Remove generated files
```
### Workflow
```
main.fsh (FluxSharp source)
   ↓ (fluxc Rust compiler)
main.asm (x86-64 assembly) [auto-generated]
   ↓ (GNU as assembler)
main.o (object file) [auto-generated]
   ↓ (GNU ld linker)
program (executable binary) [auto-generated]
```
## Project Structure
```
FluxSharp/
├── main.fsh              # Your FluxSharp source code
├── main.asm              # Generated assembly (auto-created)
├── main.o                # Generated object file (auto-created)
├── program               # Compiled executable (auto-created)
├── Makefile              # Build system
├── README.md             # This file
│
├── flux_compiler/        # Rust compiler implementation
│   └── fluxc/
│       ├── src/
│       │   └── main.rs   # Compiler entry point
│       ├── Cargo.toml
│       └── target/release/fluxc
│
├── docs/                 # Documentation
│   ├── GETTING_STARTED.md
│   ├── LANGUAGE_GUIDE.md
│   ├── ASYNC_AWAIT.md
│   └── API_REFERENCE.md
│
├── examples/             # Example programs
│   ├── hello.fsh
│   ├── fibonacci.fsh
│   └── async_example.fsh
│
└── .archive/             # Old documentation (archive)
```
## Key Files
### `main.fsh`
Your FluxSharp source code. Edit this to write your program:
```flux
fn main() {
    print("Hello, FluxSharp!");
    return 0;
}
```
### `main.asm` (Auto-generated)
x86-64 assembly code created by the Rust compiler.
- Do not edit manually - regenerated on each build
- AT&T syntax - GNU as compatible
### `program` (Auto-generated)
The executable binary - run it with `./program`
## Usage Examples
### Run Hello World (already set up)
```bash
make
```
### Edit Your Code
```bash
nano main.fsh
make
```
### Clean Rebuild
```bash
make clean
make build
```
### Full Clean (including compiler)
```bash
make distclean
```
## Language Features
### Functions
```flux
fn add(a: int, b: int) -> int {
    return a + b;
}
```
### Data Types
- `int` - 64-bit integer
- `float` - 64-bit floating point
- `string` - Text string
- `bool` - Boolean
### Control Flow
```flux
if condition {
    ; code
} else {
    ; code
}
while condition {
    ; code
}
```
### Async/Await
```flux
async fn fetch(url: string) -> string {
    let response = await http_get(url);
    return response;
}
```
## Documentation
- docs/GETTING_STARTED.md - Tutorial for beginners
- docs/LANGUAGE_GUIDE.md - Complete language reference
- docs/ASYNC_AWAIT.md - Async/await guide
- docs/API_REFERENCE.md - Standard library
## System Requirements
- Rust: 1.70+ (for rebuilding compiler)
- GNU as: Assembler
- GNU ld: Linker
- Linux x86-64: Target platform
## Troubleshooting
| Problem | Solution |
|---------|----------|
| `fluxc: not found` | Run `cd flux_compiler && cargo build --release` |
| `as: not found` | Run `sudo apt-get install binutils` |
| Assembly syntax errors | Check `main.fsh` syntax |
## License
See LICENSE file
## Version
FluxSharp v2.0.0
---
**Ready to start? Run `make` to build and execute!**
