# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-04-10

### Added
- **Core Language Features**
  - Classes with public/private methods and properties
  - Functions and method calls with proper x86-64 calling convention support
  - Variable declarations with type inference
  - Array support with bounds checking
  - Control flow: if/else statements, for loops, while loops, break/continue
  - String literals and string manipulation
  - Numeric types: int, float, double with proper precision handling

- **Security Features**
  - Bounds checking for array access - prevents out-of-bounds access
  - Null safety with null pointer detection
  - Type validation and type safety checks
  - Division by zero detection
  - Buffer overflow protection
  - Path security for file operations
  - Include security for file imports

- **Built-in Functions**
  - Math functions: `abs()`, `max()`, `min()`, `pow()`, `sqrt()`, `floor()`, `ceil()`, `round()`
  - String functions: `length()` for string length
  - I/O functions: `print()`, `serial_print()` for output
  - Type conversion: `ToString()` for converting primitives to strings

- **Compiler Features**
  - FluxSharp to x86-64 assembly compilation
  - Runtime linking with security checks
  - Error reporting with syntax error detection
  - File import system with security validation
  - Optimization for release builds

- **Testing & Documentation**
  - Comprehensive test suite with 15 tests covering all major features
  - All security checks validated with test cases
  - Complete language documentation
  - Quick start guide for new users
  - Examples for all language features

### Fixed
- For loop assignment-style increments now properly compiled (e.g., `i = i + 1`)
- User-defined function names that collide with NASM mnemonics are now properly prefixed
- String method calls like `length()` now correctly map to runtime functions
- If statement blocks are now properly compiled and executed
- Null literal handling in conditional expressions
- Condition evaluation for comparison operators

### Technical Details
- **Runtime**: x86-64 assembly runtime with syscalls
- **Compiler**: Rust-based compiler using Pest parser
- **Build System**: Makefile-based with Rust Cargo
- **Linking**: ld.bfd linker with runtime integration

### Known Limitations
- Single-threaded execution only
- No async/await support yet (planned for future releases)
- Limited floating-point operations (basic support only)
- No object inheritance yet
- No generics or templates

### Performance
- Compile-time: < 5 seconds for typical programs
- Runtime: Direct x86-64 assembly execution
- Memory: Efficient stack-based variable management

## Future Roadmap

### Version 1.1.0 (Planned)
- Async/await support
- Enhanced floating-point operations
- More math functions (trigonometric functions, logarithms)
- Exception handling improvements

### Version 1.2.0 (Planned)
- Object inheritance
- Generic types/templates
- More built-in string methods
- File I/O operations

### Version 2.0.0 (Long-term)
- Multi-threading support
- Standard library expansion
- Package manager integration
- IDE support

