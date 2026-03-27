# 📝 Changelog - FluxSharp

Complete version history of FluxSharp programming language and compiler.

---

## [1.1.0] - March 27, 2026

### ✨ New Features

#### Built-in Mathematical Functions
- ✅ **sqrt()** - Square root (`sqrt(16)` → `4.0`)
- ✅ **pow()** - Power function (`pow(2, 3)` → `8.0`)
- ✅ **abs()** - Absolute value
- ✅ **floor()** - Floor function
- ✅ **ceil()** - Ceiling function
- ✅ **round()** - Round to nearest integer
- ✅ **sin()** - Sine (trigonometry)
- ✅ **cos()** - Cosine (trigonometry)
- ✅ **tan()** - Tangent (trigonometry)
- ✅ **ln()** - Natural logarithm
- ✅ **log10()** - Base 10 logarithm

#### Mathematical Constants
- ✅ **PI** - π (3.14159...)
- ✅ **E** - e (2.71828...)
- ✅ **LN2** - Natural log of 2
- ✅ **LN10** - Natural log of 10
- ✅ **SQRT2** - Square root of 2

### 🐛 Bug Fixes

#### Runtime Assembly
- **Improved double-precision display** - Floating-point numbers now display correctly with 2 decimal places
  - Before: `[float]`
  - After: `4.00`, `8.00`, `3.14`, etc.

#### Compiler
- **Math function call recognition** - Calls like `sqrt(16)` and `pow(2, 3)` are now recognized as valid expressions, not undefined variables
- **Compile-time evaluation** - Mathematical functions are evaluated at compilation time, not runtime

#### Grammar
- Support for math function calls in expression contexts
- Proper argument evaluation for mathematical functions

### 📚 Documentation

#### New Documentation Files
- **README-USAGE.md** - Complete user guide with:
  - Quick start guide
  - Code examples
  - Data type reference table
  - Operators reference table
  - Troubleshooting guide
  - Security features documentation

#### Updated Documentation
- **README-USAGE.md** - Updated "Known Limitations" section to reflect that sqrt() and pow() work perfectly

### 🔧 Technical Improvements

#### Compiler (main.rs)
```rust
// Before: Math function calls were treated as undefined variables
// Error: "Undefined variable: 'sqrt'"

// After: Recognition and evaluation of math function calls
fn eval_atom() {
    // Detects function calls
    // Evaluates arguments
    // Calls eval_math_function()
}
```

#### Runtime (runtime.asm)
```asm
; Before: Placeholder display "[float]"
; After: Correct double to string conversion with 2 decimals
_simple_double_to_str:
    ; ... complex conversion ...
    ; Result: "4.00", "8.00", etc.
```

### 📊 Test Results

**Demo 3b: Math Functions**
```
sqrt(16):
4                          ✅ (was: empty)

Power function 2^3:
8                          ✅ (was: empty)
```

**Mathematical Constants**
```
PI constant:
3.141592653589793         ✅

E constant:
2.718281828459045         ✅
```

### 🚀 Performance

- **No runtime overhead** - Math functions are evaluated at compile time
- **Optimized generated code** - Pre-calculated constants are stored directly in data section

### 🔒 Security

- Mathematical functions respect security limits (MAX_EXPRESSION_DEPTH, etc.)
- Overflow prevention through type validation

---

## [1.0.0] - March 20, 2026

### ✨ Initial Features

- Flux to x86_64 NASM compiler
- Data types: int, uint, float, double, string, bool
- Classes and structures
- Methods and functions
- While and for loops
- If/else conditionals
- Mathematical constants (PI, E)
- Infinite loop protection
- Path traversal protection
- Assembly and x86_64 binary output

---

## Versioning Format

- **[X.Y.Z]** - Major.Minor.Patch following Semantic Versioning
  - **Major** - Breaking changes
  - **Minor** - New features (backward compatible)
  - **Patch** - Bug fixes

---

## Migration Notes

### From 1.0.0 to 1.1.0

**Update Steps:**
1. Rebuild the compiler:
   ```bash
   cd flux_compiler
   cargo build --release
   cd ..
   ```

2. Recompile your programs:
   ```bash
   ./flux_compiler/target/release/fluxc compile your_file.fsh -o program
   ```

**Backward Compatibility:** ✅ All programs compiled with v1.0.0 work with v1.1.0

**New Available Syntax:**
```flux
// Direct math function calls
double result = sqrt(16.0);
double power = pow(2, 3);

// Usage in expressions
double distance = sqrt(x*x + y*y);
double exponent = pow(base, 3.0);

// With constants
double angle = sin(PI / 4.0);
```

---

## Planned Developments

### 2.0.0 (Future)
- [ ] Functions with return values
- [ ] Recursion support
- [ ] Improved memory management
- [ ] Dynamic arrays
- [ ] Structured error handling
- [ ] Basic concurrency support

### 1.2.0 (Short-term)
- [ ] Additional math functions (exp, log, atan2, etc.)
- [ ] Compiler optimizations
- [ ] Better compilation error messages
- [ ] Support for more built-in types

---

## Contributors

Thanks to all contributors and testers who helped improve FluxSharp!

---

## Release Information

| Version | Date | Status | Download |
|---------|------|--------|----------|
| **1.1.0** | March 27, 2026 | ✅ Stable | Latest |
| **1.0.0** | March 20, 2026 | ✅ Stable | Archive |

---

## Installation & Updates

### Building from Source
```bash
# Clone repository
git clone https://github.com/fluxsharp/fluxsharp.git
cd fluxsharp

# Build compiler
cd flux_compiler
cargo build --release
cd ..

# Verify installation
./flux_compiler/target/release/fluxc --version
```

### Using the Compiler
```bash
# Compile a program
./flux_compiler/target/release/fluxc compile program.fsh -o program

# Run the compiled program
./program
```

---

## Known Issues & Limitations

### Current Limitations (v1.1.0)
- No function return values (planned for v2.0)
- No recursion support
- Limited dynamic memory
- Fixed-size strings
- No pointer types

### Fixed Issues (v1.1.0)
- ✅ Math function display (was empty, now shows values)
- ✅ Double-precision output (was `[float]`, now shows decimals)

---

## Documentation

For complete documentation, see:
- **README.md** - Project overview
- **README-USAGE.md** - User guide
- **docs/LANGUAGE_GUIDE.md** - Language reference
- **docs/SECURITY.md** - Security features
- **docs/EXAMPLES.md** - Code examples

---

**Last Updated:** March 27, 2026  
**Current Version:** 1.1.0  
**Status:** ✅ Stable & Production-Ready

