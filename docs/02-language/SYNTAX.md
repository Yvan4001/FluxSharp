# FluxSharp Syntax

Core syntax rules and language structure.

## Comments

### Single-line Comments
```flux
// This is a comment
int x = 10; // Comment at end of line
```

### Multi-line Comments
```flux
/* This is a
   multi-line
   comment */
```

## Identifiers

Valid identifiers must:
- Start with letter (A-Z, a-z)
- Contain letters, digits, or underscore
- Be case-sensitive

```flux
int myVariable;
int _private;
int MAX_VALUE;
int value123;
```

## Keywords

Reserved words:

**Types**: `int`, `uint`, `long`, `float`, `double`, `ulong`, `byte`, `string`, `bool`, `void`

**Declarations**: `const`, `struct`, `class`, `public`, `private`, `static`

**Control**: `if`, `else`, `while`, `for`, `return`, `break`, `continue`

**Async**: `async`, `await`

**Other**: `true`, `false`

## Literals

### Integer Literals
```flux
int decimal = 42;
int hexadecimal = 0xFF;      // 255
int binary = 0b1010;         // 10
int octal = 0o77;            // 63
```

### Floating Point Literals
```flux
float single = 3.14f;
double dbl = 3.14159;
double scientific = 1.5e-10;
```

### String Literals
```flux
string text = "Hello World";
string empty = "";
string escaped = "Line1\nLine2";
```

### Character Literals
```flux
char ch = 'A';
char newline = '\n';
```

### Boolean Literals
```flux
bool yes = true;
bool no = false;
```

## Type Declaration

### Primitive Types
```flux
int number;              // 64-bit integer
uint unsigned;           // Unsigned integer
long large;             // Large integer
float decimal;          // 32-bit float
double precise;         // 64-bit double
ulong unsigned_long;    // Unsigned long
byte small;             // 8-bit byte
string text;            // Text string
bool flag;              // Boolean
void nothing;           // No value (functions only)
```

### Custom Types
```flux
Point location;         // Custom class/struct
CustomType instance;    // Any defined type
```

## Operators and Expressions

### Arithmetic Operators
```flux
int a = 5 + 3;          // Addition
int b = 5 - 3;          // Subtraction
int c = 5 * 3;          // Multiplication
int d = 5 / 3;          // Division
int e = 5 % 3;          // Modulo
```

### Comparison Operators
```flux
a < b                   // Less than
a > b                   // Greater than
a <= b                  // Less or equal
a >= b                  // Greater or equal
a == b                  // Equal
a != b                  // Not equal
```

### Logical Operators
```flux
a && b                  // AND
a || b                  // OR
!a                      // NOT
```

### Bitwise Operators
```flux
a & b                   // AND
a | b                   // OR
a ^ b                   // XOR
~a                      // NOT
a << b                  // Left shift
a >> b                  // Right shift
```

### Assignment Operators
```flux
x = 10;                 // Assign
x += 5;                 // Add and assign
x -= 5;                 // Subtract and assign
x++;                    // Increment
x--;                    // Decrement
```

## Expressions

Basic expression syntax:

```flux
atom [ (arith_op | bitwise_op) atom ]*
```

Examples:
```flux
x + y * 2
a & b | c
value + 10 - 5
```

## Statements

Statements end with semicolon (`;`):

```flux
int x = 10;
x = 20;
print(x);
return;
```

## Blocks

Blocks contain statements in braces:

```flux
{
    int x = 10;
    string name = "test";
    return;
}
```

## Functions

Basic function syntax:

```flux
public void FunctionName() {
    // Body
}

public int Add(int a, int b) {
    return a + b;
}

async public void AsyncFunction() {
    // Async body
}
```

## Classes

Class structure:

```flux
public class ClassName {
    public int property;
    private string field;
    
    public void Method() {
        // Implementation
    }
}
```

## Structs

Struct structure:

```flux
public struct StructName {
    int field1;
    string field2;
}
```

## Arrays

Array declaration:

```flux
int[100] numbers;           // Array of 100 ints
string[256] names;          // Array of 256 strings
byte[80 * 25] screen;       // Array with expression size
```

## Access Modifiers

```flux
public int visible;         // Accessible from outside
private int hidden;         // Only within class
static int shared;          // Class-level (not instance)
```

## Variable Declaration

```flux
int x;                      // Declaration
int y = 10;                 // Declaration with initialization
const int MAX = 100;        // Constant
```

## Function Calls

```flux
Print("Hello");             // Simple call
int result = Add(5, 3);     // Call with arguments
obj.Method();               // Method call
```

## Async/Await Syntax

```flux
async public void FetchData() {
    string data = await GetData("url");
    return;
}
```

## Naming Conventions

- **Functions**: `PascalCase` - `GetName()`, `CalculateSum()`
- **Variables**: `camelCase` or `snake_case` - `myVariable`, `my_variable`
- **Constants**: `UPPER_CASE` - `MAX_SIZE`, `DEFAULT_VALUE`
- **Classes**: `PascalCase` - `Person`, `DataManager`
- **Structs**: `PascalCase` - `Point`, `Rectangle`

---

Next: Read [TYPES.md](TYPES.md)

