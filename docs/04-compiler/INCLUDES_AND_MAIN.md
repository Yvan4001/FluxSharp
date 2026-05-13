# FluxSharp Compiler - Main Class Validation & Include System

## Overview

The FluxSharp compiler (fluxc) now applies the following checks:

### 1. **Main Class & Method Validation**

Every program must have:
- ✅ Exactly **ONE Main class**
- ✅ Exactly **ONE main() method** in that class

#### Required Format:
```fsh
public class Main {
public void main() {
print("Hello, World!");

` ... }
}
```

#### Errors Detected:
- ❌ **MISSING MAIN CLASS**: No Main class found
- ❌ **MULTIPLE MAIN CLASSES**: Multiple Main classes declared
- ❌ **MISSING MAIN METHOD**: No main() method in the Main class
- ❌ **MULTIPLE MAIN METHODS**: Multiple main() methods declared

### 2. **Include System for External .fsh Files**

You can now include external `.fsh` files using:

```fsh
// #include "filename.fsh"

```

#### Inclusion Rules:

1. **Only `.fsh` files** are allowed
2. Inclusions are processed **recursively** (an included file can include other files)
3. **Protection against circular inclusions** Automatic
4. The files must be in the **same directory** or in subdirectories (relative paths)

#### Example:
```fsh
// #include "helper.fsh"

// #include "math_utils.fsh"

public class Main {

public void main() {

print("Program with includes");

}
}
```

#### Inclusion Errors:
- ❌ **INVALID INCLUDE FILE**: The file does not have the `.fsh` extension
- ❌ **INCLUDE FILE NOT FOUND**: The specified file does not exist
- ❌ **CIRCULAR INCLUDE**: A circular include was detected

### 3. **Compilation Workflow**

```bash
# Compile a single file
cargo run --release -- compile main.fsh -o program

# Compile all .fsh files in a directory
cargo run --release -- compile --all ./examples -o program

# Compile and run
cargo run --release -- compile main.fsh -o program --run
```

### 4. **Security Features**

- ✅ Path validation (no path traversal)
- ✅ File size limit (50 MB max)
- ✅ Circular inclusion protection
- ✅ Validation of generated ASM size (100 MB max)
- ✅ Declaration limit (10,000 max per block)
- ✅ Expression depth limit (100 max)

## Example Project Structure

```
project/
├── main.fsh # Main file with the Main class
├── helper.fsh # Helper file (optional)
├── utils.fsh # Utility file (optional)
└── subdir/

└── database.fsh # Inclusion supported
```

### main.fsh:
```fsh
// #include "helper.fsh"

// #include "subdir/database.fsh"

public class Main {

public void main() {

print("Program started");

/ Use the classes defined in helper.fsh

}
}
```

## Error Messages

The compiler provides clear error messages:

```
❌ MISSING MAIN CLASS

Your program must have exactly one 'Main' class with a 'void main()' method.

Example:
public class Main {

public void main() {

print("Hello, World!");

}
}
```

## Implementation Details

### validate_main_class()
Checks that the compiled code contains:
- Exactly one `class Main` declaration
- Exactly one `void main()` declaration

### process_includes()
Processes include directives (`// #include "..."`) before compilation:
1. Scans lines for include directives
2. Validates that it is a `.fsh` file
3. Loads and validates the external file
4. Processes nested includes
5. Combines the contents for compilation

## Notes

- Include comments must be in the format: `// #include "filename.fsh"`
- Includes are processed in the order they appear
- Included files must have a valid class structure
- An included file must not contain a `Main` class (otherwise, a multiple Main class error will occur)
