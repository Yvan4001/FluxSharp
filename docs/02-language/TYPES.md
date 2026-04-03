# FluxSharp Types

Complete reference for all type system.

## Primitive Types

### Numeric Types

**Integer Types:**
```flux
int number;             // 64-bit signed integer
uint unsigned;          // Unsigned integer
long large;            // Large integer
ulong unsigned_long;   // Unsigned long
byte small;            // 8-bit byte
```

**Floating-Point Types:**
```flux
float single = 3.14f;        // 32-bit float
double precise = 3.14159;    // 64-bit double (default)
```

### Text Types

**String:**
```flux
string text = "Hello World";
string empty = "";
string lines = "Line1\nLine2";
```

**Char:**
```flux
char single = 'A';
char newline = '\n';
```

### Boolean Type

```flux
bool yes = true;
bool no = false;
```

### Void Type

```flux
public void DoNothing() {
    // Returns nothing
}
```

## Type Literals

### Integer Literals
```flux
int decimal = 42;
int hex = 0xFF;        // Hexadecimal
int binary = 0b1010;   // Binary
int octal = 0o77;      // Octal
```

### Float Literals
```flux
float single = 3.14f;
double dbl = 3.14159;
double scientific = 1.5e-10;
```

### String Literals
```flux
string text = "Hello";
string with_escape = "Line1\nLine2\tTabbed";
```

### Boolean Literals
```flux
bool yes = true;
bool no = false;
```

## Array Types

### Fixed-Size Arrays

```flux
int[100] numbers;           // 100 integers
string[256] names;          // 256 strings
byte[80 * 25] screen;       // 2000 bytes (80 * 25)
```

### Array Access

```flux
int[10] values;
values[0] = 42;             // Set element
int first = values[0];      // Get element
```

### Multi-Dimensional Arrays

```flux
int[10][20] matrix;         // 2D array
matrix[0][0] = 1;
int element = matrix[5][10];
```

## Custom Types

### Class Type

```flux
public class Person {
    public string name;
    public int age;
    
    public void Greet() {
        // Method
    }
}

Person person = new Person();
```

### Struct Type

```flux
public struct Point {
    int x;
    int y;
}

Point location = new Point();
location.x = 10;
```

## Type Modifiers

### Array Modifier

```flux
int[100] array;         // Array of 100 elements
string[256] strings;    // 256-element string array
```

### Visibility

```flux
public int visible;     // Public access
private int hidden;     // Private access
```

### Static

```flux
public static int count;    // Class-level variable
```

## Type Conversion

Explicit casting (when needed):

```flux
int i = 42;
float f = (float)i;         // int to float
double d = (double)i;       // int to double
```

## Default Values

When variables are declared without initialization:

```flux
int x;           // 0
float f;         // 0.0
string s;        // null
bool b;          // false
```

## Type Checking

FluxSharp performs static type checking:

```flux
int x = 42;
// x = "string";    // ERROR - type mismatch
// float y = 3.14;  // ERROR - precision loss (use double)
```

## Nullable Types

Pointers can be null:

```flux
int[100] arr;
if (arr != null) {
    // Use array
}
```

## Type Compatibility

- Integer types can be mixed in operations
- Floating types can be mixed
- String concatenation with +
- Comparison across compatible types

Examples:
```flux
int a = 10;
long b = a;                 // Compatible

float x = 3.14f;
double y = x;              // Compatible

string s = "Hello" + "World";  // Concatenation
```

## Size Expressions in Arrays

Arrays can use computed sizes:

```flux
int[100] simple;
int[80 * 25] calculated;    // 2000 elements
byte[512] block;
int[16 + 32] combined;      // 48 elements
```

## Literal Formats

### Numbers
```flux
42          // Decimal
0xFF        // Hexadecimal
0b1010      // Binary
0o77        // Octal
3.14        // Float
1.5e-10     // Scientific notation
```

### Strings
```flux
"normal"
"with\nescape"
"empty" == ""
```

### Characters
```flux
'A'
'\n'
'\t'
```

---

Next: Read [VARIABLES.md](VARIABLES.md)

