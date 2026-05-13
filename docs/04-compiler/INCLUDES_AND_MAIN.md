# FluxSharp Compiler - Main Class Validation & Import System

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
    }
}
```

#### Errors Detected:
- ❌ **MISSING MAIN CLASS**: No Main class found
- ❌ **MULTIPLE MAIN CLASSES**: Multiple Main classes declared
- ❌ **MISSING MAIN METHOD**: No main() method in the Main class
- ❌ **MULTIPLE MAIN METHODS**: Multiple main() methods declared

### 2. **Import System for External .fsh Files**

You can import external `.fsh` files using one of two equivalent syntaxes:

#### Option 1: Using Directive (Recommended - C# Style)
```fsh
using "examples/math_helper.fsh";
```

#### Option 2: Import Directive (Alternative - C# Style)
```fsh
import "examples/math_helper.fsh";
```

#### Import Rules:

1. **Only `.fsh` files** are allowed
2. Imports are processed **recursively** (an imported file can import other files)
3. **Protection against circular imports** - Automatic detection
4. The files can use **relative paths** from the importing file location

#### Example (Using Imports):
```fsh
using "examples/math_helper.fsh";
using "examples/string_utils.fsh";

public class Main {
    public void main() {
        print("Program with imports");
        MathHelper helper = new MathHelper();
        int result = helper.GetDouble(21);
    }
}
```

#### Import Errors:
- ❌ **INVALID IMPORT FILE**: The file does not have the `.fsh` extension
- ❌ **IMPORT FILE NOT FOUND**: The specified file does not exist
- ❌ **CIRCULAR IMPORT**: A circular import was detected

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
- ✅ Circular import protection
- ✅ Validation of generated ASM size (100 MB max)
- ✅ Declaration limit (10,000 max per block)
- ✅ Expression depth limit (100 max)

## Example Project Structure

```
project/
├── main.fsh                    # Main file with the Main class
├── examples/
│   ├── math_helper.fsh         # Helper file
│   ├── string_utils.fsh        # String utilities
│   └── utils.fsh               # General utilities
└── subdir/
    └── database.fsh            # Database utilities
```

### main.fsh (Modern Syntax):
```fsh
using "examples/math_helper.fsh";
using "examples/string_utils.fsh";
using "subdir/database.fsh";

public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}

public class Main {
    public void main() {
        print("Program started");
        
        // Use imported classes
        MathHelper helper = new MathHelper();
        int doubled = helper.GetDouble(21);
        
        StringUtils strings = new StringUtils();
        Database db = new Database();
        
        print("All systems operational");
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

### process_includes_internal()
Processes import directives before compilation in this order:
1. **Recognizes two formats:**
   - `using "filename.fsh";` (C# style - recommended)
   - `import "filename.fsh";` (C# style - alternative)

2. **For each import:**
   - Validates that it is a `.fsh` file
   - Loads and validates the external file
   - Checks for circular imports
   - Processes nested imports recursively
   - Combines the contents for compilation

3. **Security checks:**
   - File path validation (no path traversal)
   - Circular import detection
   - File size validation

## Notes

- **Recommended:** Use `using` directive as it is the standard C# style
- **Alternative:** `import` directive also works and is fully supported
- Imports are processed in the order they appear
- Imported files must have valid class structure (no duplicate Main class)
- An imported file should not contain a `Main` class (otherwise, multiple Main class error will occur)
- Use **relative paths** from the importing file's directory

## Supported Import Formats

| Format | Status | Recommended |
|--------|--------|-------------|
| `using "file.fsh";` | ✅ Supported | Yes (Primary) |
| `import "file.fsh";` | ✅ Supported | Yes (Alternative) |

Both formats are fully supported and produce identical results during compilation.
