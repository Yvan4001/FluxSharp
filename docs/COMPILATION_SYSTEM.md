# FluxGridOS Compilation System

## Multi-File Compilation

The FluxGridOS compiler combines multiple `.fsh` files into a single compiled output. This is similar to how C/C++ compilers work with multiple source files.

## Compilation Process

### Single File
```bash
fluxc compile os_fs/src/main.fsh
```

### Multiple Files (Linking)
```bash
fluxc compile os_fs/src/main.fsh os_fs/src/lib.fsh os_fs/src/async_example.fsh -o kernel.bin
```

## File Organization

```
os_fs/src/
├── main.fsh           # Entry point (_start function)
├── lib.fsh            # Core library (kernel subsystems, async/await)
├── async_example.fsh  # Example async functions
└── main.o             # Compiled output (if using separate compilation)
```

## Compilation Phases

### Phase 1: Parsing
Each `.fsh` file is parsed according to `flux_grammar.pest`
- Tokenization
- AST generation
- Syntax validation

### Phase 2: Semantic Analysis
- Function signature resolution
- Type checking
- Symbol table creation

### Phase 3: Code Generation
- Intermediate code generation
- Optimization passes
- Assembly generation

### Phase 4: Assembly
- Assembly code compilation
- Object file generation (`.o`)

### Phase 5: Linking
- Combine all object files
- Resolve external symbols
- Generate final executable

## Symbol Resolution

### Function Calls Across Files

**lib.fsh:**
```flux
public void init_drivers() => {
    // Implementation
}
```

**main.fsh:**
```flux
public void main(long boot_info_addr, long magic_number) => {
    init_drivers();  // Resolved from lib.fsh
}
```

### External Symbols

External symbols are resolved during linking:
- `kernel_entry_from_lib()` from lib.fsh
- `async_*()` functions from async system
- `hcf()` halt function
- Platform-specific functions (`outb()`, `inb()`, etc.)

## Build Script Example

Create `build_flux.sh`:

```bash
#!/bin/bash

FLUXC="./target/x86_64-unknown-linux-gnu/debug/fluxc"
OUTPUT_DIR="./os_fs/build"
SOURCE_FILES=(
    "os_fs/src/main.fsh"
    "os_fs/src/lib.fsh"
    "os_fs/src/async_example.fsh"
)

mkdir -p $OUTPUT_DIR

echo "Compiling Flux files..."
$FLUXC compile ${SOURCE_FILES[@]} -o $OUTPUT_DIR/kernel.bin

if [ $? -eq 0 ]; then
    echo "✓ Compilation successful!"
    ls -lh $OUTPUT_DIR/kernel.bin
else
    echo "✗ Compilation failed!"
    exit 1
fi
```

## Dependency Resolution

The compiler automatically resolves dependencies:

1. **Direct Dependencies**: Functions called in current file
2. **Transitive Dependencies**: Functions called by called functions
3. **Circular Dependencies**: Detected and reported as error

## Include Mechanism

While Flux doesn't have explicit `#include`, file inclusion can be achieved through:

### Option 1: Compile Multiple Files
```bash
fluxc compile main.fsh lib.fsh
```

### Option 2: Module Organization
```
os_fs/
├── src/
│   ├── main.fsh
│   ├── lib.fsh
│   └── async_example.fsh
└── build/
    └── kernel.bin
```

### Option 3: Namespace/Module Declaration (Future)
```flux
module kernel.drivers;

public void init_drivers() => {
    // Implementation
}
```

## Linking Details

### Symbol Table
Each compiled file contributes to a global symbol table:

```
Symbol Table:
├── Functions
│   ├── main() -> main.fsh:1
│   ├── init_serial() -> main.fsh:10
│   ├── kernel_entry_from_lib() -> lib.fsh:50
│   └── async_init() -> lib.fsh:200
├── Types
│   ├── Task -> lib.fsh:150
│   ├── Config -> lib.fsh:30
│   └── CustomBootInfo -> lib.fsh:40
└── Constants
    ├── MULTIBOOT2_MAGIC -> main.fsh:5
    └── SERIAL_PORT -> main.fsh:7
```

### Relocation
External symbol addresses are resolved during linking:
- Jump targets
- Function pointers
- Global variable addresses

## Compilation Order

Files can be compiled in any order, but execution flow matters:

```
1. main.fsh is loaded first (entry point at _start)
2. main.fsh calls kernel_entry_from_lib() from lib.fsh
3. lib.fsh initializes all subsystems
4. async_example.fsh provides additional functions
```

## Optimization with Multi-File

Multi-file compilation enables:
- **Whole Program Optimization**: Optimize across file boundaries
- **Link-Time Optimization (LTO)**: Final optimization pass
- **Dead Code Elimination**: Remove unused functions from all files
- **Inline Optimization**: Inline small functions across files

## Error Handling

### Unresolved Symbols
```
Error: Unresolved symbol 'undefined_function'
  referenced in main.fsh:45
```

### Type Mismatch
```
Error: Type mismatch in function call
  main.fsh:30: expected 'uint', got 'int'
```

### Duplicate Definition
```
Error: Duplicate symbol definition
  init_drivers() defined in main.fsh:20 and lib.fsh:45
```

## Advanced Features

### Conditional Compilation (Future)
```flux
#ifdef DEBUG
    log_message(LogLevel::Debug, "Debug enabled");
#endif
```

### Macro Definition (Future)
```flux
#define MAX_TASKS 128
```

## Best Practices

1. **One public interface per module**
   ```flux
   // lib.fsh - core library
   public void init_drivers() => { ... }
   
   // async_example.fsh - example code
   async public void fetch_data() => { ... }
   ```

2. **Minimize inter-file dependencies**
   - Keep files focused
   - Use clear interfaces

3. **Order files logically**
   - Library code first
   - Application code last
   - Entry point last

4. **Document file purposes**
   ```flux
   // main.fsh - Boot sequence and initialization
   // lib.fsh - Core kernel functionality
   // async_example.fsh - Async operation examples
   ```

## Compilation Command Reference

```bash
# Single file
fluxc compile main.fsh

# Multiple files
fluxc compile main.fsh lib.fsh async_example.fsh

# With output file
fluxc compile main.fsh lib.fsh -o kernel.bin

# With optimization
fluxc compile -O2 main.fsh lib.fsh -o kernel.bin

# With debug info
fluxc compile -g main.fsh lib.fsh -o kernel.bin

# Verbose output
fluxc compile -v main.fsh lib.fsh

# Generate assembly
fluxc compile -S main.fsh -o kernel.asm

# Generate object file
fluxc compile -c main.fsh -o main.o
```

## Future Enhancements

1. **Package Manager**: Manage external .fsh libraries
2. **Pre-compiled Libraries**: Link with .a or .so libraries
3. **Incremental Compilation**: Only recompile changed files
4. **Distributed Compilation**: Parallel compilation of files
5. **Module System**: Explicit module/package declarations

