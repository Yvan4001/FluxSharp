# FluxSharp Language Documentation

Complete technical reference based on actual grammar.

## Navigation

- **[QUICKSTART.md](QUICKSTART.md)** - Get started in 5 minutes ⭐ START HERE
- **[SYNTAX.md](SYNTAX.md)** - Core syntax and language structure
- **[TYPES.md](TYPES.md)** - Primitive and composite types
- **[VARIABLES.md](VARIABLES.md)** - Variable and constant declaration
- **[FUNCTIONS.md](FUNCTIONS.md)** - Function definition and calls
- **[CLASSES.md](CLASSES.md)** - Class and struct definitions
- **[CONTROL_FLOW.md](CONTROL_FLOW.md)** - If/else, loops, return
- **[OPERATORS.md](OPERATORS.md)** - Operators and expressions
- **[ARRAYS.md](ARRAYS.md)** - Array declaration and access
- **[ASYNC_AWAIT.md](ASYNC_AWAIT.md)** - Async functions and await
- **[STDLIB.md](STDLIB.md)** - Standard library and math functions

## Language Overview

FluxSharp features:
- **Static typing** - Types: int, uint, long, float, double, ulong, byte, string, bool, void
- **Comments** - `// single line` and `/* multi-line */`
- **Object-oriented** - Classes with properties and methods
- **Structured** - Structs with field alignment
- **Arrays** - Fixed-size arrays with bounds checking
- **Async support** - async functions with await expressions
- **Operators** - Arithmetic, bitwise, logical, comparison

## File Structure

```
file = {
    const_decl*
    struct_def*
    class_def*
    function*
}
```

Root file contains:
- `const` declarations
- `struct` definitions
- `class` definitions
- Function definitions

## Basic Syntax

```flux
// Comments with double slash
/* Multi-line
   comments */

// Constant
const int MAX = 100;

// Struct definition
public struct Point {
    int x;
    int y;
}

// Class definition
public class Person {
    public string name;
    public int age;
    
    public void greet() {
        // Method
    }
}

// Function
public static void Main() {
    int x = 42;
    return;
}

// Async function
async public void FetchData() {
    string response = await GetURL("http://example.com");
    return;
}
```

## Program Entry Point

```flux
public static void Main() {
    // Program starts here
    return;
}
```

---

**Start with [SYNTAX.md](SYNTAX.md) to learn the language structure.**

