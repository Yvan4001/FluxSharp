# FluxSharp Build Script - Dynamic Compilation Guide

## Overview

The `build.sh` script has been enhanced to support dynamic compilation of any FluxSharp file.

## Usage

### Compile Default File (main.fsh)

```bash
./build.sh
```

This compiles `main.fsh` in the project root and creates `main` executable.

### Compile Specific File

```bash
./build.sh path/to/file.fsh
```

The script automatically:
1. Compiles the specified `.fsh` file
2. Generates a binary with the same name (minus `.fsh`)
3. Runs the compiled program

## Examples

### Hello World Example

```bash
./build.sh examples/hello.fsh
```

Creates: `examples/hello` executable  
Runs: `./examples/hello`

### Calculator Example

```bash
./build.sh examples/calculator.fsh
```

Creates: `examples/calculator` executable  
Runs: `./examples/calculator`

### Custom File

```bash
./build.sh my_program.fsh
```

Creates: `my_program` executable  
Runs: `./my_program`

### Subdirectory File

```bash
./build.sh src/utils/helper.fsh
```

Creates: `src/utils/helper` executable  
Runs: `./src/utils/helper`

## Output

For each build, the script displays:

```
🚀 FluxSharp - Complete Build and Run
=======================================

📄 Input:  /path/to/file.fsh
📦 Output: /path/to/file

🔨 Step 1: Building Rust compiler...
📝 Step 2: Compiling FluxSharp → Executable
🚀 Step 3: Executing Program

[Program output here]

✅ Build complete!
```

## Automatic Features

The script automatically:

✅ **Checks dependencies** - Ensures cargo is installed  
✅ **Builds compiler** - Compiles Rust compiler (cached after first run)  
✅ **Generates assembly** - Creates `.asm` file  
✅ **Assembles code** - Generates `.o` object file  
✅ **Links binary** - Creates executable with runtime  
✅ **Executes program** - Runs the compiled binary  
✅ **Cleanup** - Intermediate files stored in source directory  

## File Organization

```
FluxSharp/
├── build.sh              # Main build script
├── main.fsh              # Default program
├── program               # Executable (from main.fsh)
├── examples/
│   ├── hello.fsh         # Hello world example
│   ├── hello             # Hello executable
│   ├── calculator.fsh    # Calculator example
│   ├── calculator        # Calculator executable
│   ├── arrays.fsh        # Arrays example
│   └── arrays            # Arrays executable
└── src/
    └── [your files]
```

## Advanced Usage

### Compile Multiple Files

```bash
./build.sh main.fsh           # Build main program
./build.sh examples/hello.fsh # Build hello example
./build.sh examples/calc.fsh  # Build calculator
```

### Build Without Execution

Build script always executes the compiled binary. To build without running:

```bash
bash -x build.sh file.fsh 2>&1 | grep "Executable created"
```

Or modify the script to skip the execution step.

### Clean Generated Files

Remove intermediate build files:

```bash
rm -f main.asm main.o main               # For main.fsh
rm -f examples/hello.asm examples/hello.o examples/hello  # For examples
```

## Error Handling

If compilation fails, the script displays:

```
❌ Compilation failed
```

Check:
1. **Syntax errors** - Verify FluxSharp syntax matches [SYNTAX.md](docs/SYNTAX.md)
2. **File path** - Ensure file exists at specified path
3. **Compiler** - Verify cargo is installed: `cargo --version`

## Performance

- **First run**: ~3 seconds (builds compiler)
- **Subsequent runs**: <1 second (cached compiler)
- **Compilation**: <1 second for typical files
- **Total time**: ~1 second (cached)

## Examples in Repository

The `examples/` folder contains ready-to-compile examples:

### hello.fsh
Basic "Hello World" program demonstrating:
- Simple function calls
- String printing

```bash
./build.sh examples/hello.fsh
```

### calculator.fsh
Object-oriented programming example with:
- Class definition
- Constructor and methods
- Object instantiation with `new`

```bash
./build.sh examples/calculator.fsh
```

### arrays.fsh
Array usage example with:
- Array declaration
- Element assignment
- Element access

```bash
./build.sh examples/arrays.fsh
```

## Custom Projects

To create your own FluxSharp project:

1. **Create file**:
   ```bash
   cat > my_app.fsh << 'EOF'
   public static void Main() {
       print("My FluxSharp App");
       return;
   }
   EOF
   ```

2. **Build and run**:
   ```bash
   ./build.sh my_app.fsh
   ```

3. **Executable created**:
   ```bash
   ./my_app    # Run the compiled program
   ```

## Workflow

1. **Edit file**:
   ```bash
   nano examples/my_program.fsh
   ```

2. **Build and run**:
   ```bash
   ./build.sh examples/my_program.fsh
   ```

3. **See output** - Program runs automatically

4. **Iterate** - Go back to step 1

## Integration with Development Tools

### Run in IDE

Most IDEs allow custom build commands:

```
./build.sh examples/my_file.fsh
```

### Makefile Integration

Add to your Makefile:

```makefile
%.run: %.fsh
	./build.sh $<

test:
	./build.sh examples/hello.fsh
	./build.sh examples/calculator.fsh
```

Run with:
```bash
make my_file.run
make test
```

---

**The build system is now fully dynamic and flexible!** 🚀

