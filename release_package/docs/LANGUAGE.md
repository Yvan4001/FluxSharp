# FluxSharp Language Reference

Complete reference for the FluxSharp programming language.

## Table of Contents

1. [Types](#types)
2. [Variables](#variables)
3. [Operators](#operators)
4. [Control Flow](#control-flow)
5. [Classes and Objects](#classes-and-objects)
6. [Arrays](#arrays)
7. [Functions and Methods](#functions-and-methods)
8. [Built-in Functions](#built-in-functions)
9. [String Operations](#string-operations)
10. [Error Handling](#error-handling)

## Types

### Primitive Types

| Type | Size | Range | Description |
|------|------|-------|-------------|
| `int` | 64-bit | -2^63 to 2^63-1 | Signed integer |
| `float` | 32-bit | ±1.4e-45 to ±3.4e38 | Single-precision floating point |
| `double` | 64-bit | ±2.2e-308 to ±1.8e308 | Double-precision floating point |
| `string` | Variable | N/A | Null-terminated text |
| `bool` | 8-bit | true/false | Boolean value |
| `null` | 64-bit | 0 | Null pointer |

### Declaring Variables

```flux
int x = 42;
float pi = 3.14f;      // Note the 'f' suffix
double e = 2.71828;    // No suffix for double
string name = "Alice";
bool active = true;
```

## Variables

### Scope Rules

- **Local variables**: Declared inside methods, scope is the method
- **Class variables**: Declared in class body, accessible to all methods
- **Access modifiers**: `public`, `private` (default)

```flux
public class Example {
    // Class variable (default private)
    int classVar = 10;
    
    // Public class variable
    public string publicVar = "visible";
    
    public void method() {
        // Local variable
        int localVar = 5;
    }
}
```

### Initialization

Variables are initialized when declared:

```flux
int x = 0;              // Explicit initialization
int[10] arr;            // Array allocated but not initialized
string text = "hello";  // String literal
```

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Modulo | `a % b` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `a == b` |
| `!=` | Not equal | `a != b` |
| `<` | Less than | `a < b` |
| `>` | Greater than | `a > b` |
| `<=` | Less or equal | `a <= b` |
| `>=` | Greater or equal | `a >= b` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | Logical AND | `a && b` |
| `\|\|` | Logical OR | `a \|\| b` |
| `!` | Logical NOT | `!a` |

### Assignment Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `=` | Assignment | `a = 5` |
| `+=` | Add and assign | `a += 5` |
| `-=` | Subtract and assign | `a -= 5` |
| `*=` | Multiply and assign | `a *= 5` |
| `/=` | Divide and assign | `a /= 5` |

### Increment/Decrement

```flux
i++;    // Increment
i--;    // Decrement
i = i + 1;  // Equivalent
```

## Control Flow

### If/Else Statement

```flux
if (condition) {
    // Code executed if condition is true
} else if (other_condition) {
    // Code executed if other_condition is true
} else {
    // Code executed if all conditions are false
}
```

### For Loop

```flux
for (int i = 0; i < 10; i = i + 1) {
    print(i);
}
```

### While Loop

```flux
while (condition) {
    // Code executed while condition is true
}
```

### Break and Continue

```flux
for (int i = 0; i < 10; i = i + 1) {
    if (i == 5) {
        break;      // Exit loop
    }
    if (i == 3) {
        continue;   // Skip to next iteration
    }
    print(i);
}
```

## Classes and Objects

### Class Definition

```flux
public class Person {
    // Class variables
    string name;
    int age;
    
    // Constructor (not yet supported, use initialization)
    
    // Method
    public void PrintInfo() {
        print("Name: ");
        print(name);
        print("Age: ");
        print(age);
    }
}
```

### Creating Objects

```flux
Person p = new Person();
p.name = "Alice";
p.age = 30;
p.PrintInfo();
```

### Method Definition

```flux
public class MyClass {
    // Method with no parameters
    public void NoParams() {
        print("Hello");
    }
    
    // Method with parameters
    public int Add(int a, int b) {
        return a + b;
    }
    
    // Method with return type
    public string GetMessage() {
        return "Hello, World!";
    }
}
```

### Access Modifiers

- `public`: Accessible from anywhere
- `private`: Accessible only within the class (default)

```flux
public class BankAccount {
    private int balance = 1000;
    
    public int GetBalance() {
        return balance;
    }
    
    public void Deposit(int amount) {
        balance = balance + amount;
    }
}
```

## Arrays

### Array Declaration

```flux
int[10] numbers;           // Array of 10 integers
string[5] names;           // Array of 5 strings
int[100] big_array;        // Larger array
```

### Array Access

```flux
numbers[0] = 42;           // Set element
int value = numbers[0];    // Get element
print(numbers[5]);         // Print element
```

### Array Operations

```flux
public class Main {
    public void main() {
        int[5] arr;
        
        // Initialize
        arr[0] = 10;
        arr[1] = 20;
        arr[2] = 30;
        
        // Iterate
        for (int i = 0; i < 5; i = i + 1) {
            print(arr[i]);
        }
    }
}
```

### Bounds Checking

All array accesses are automatically bounds-checked:

```flux
int[5] arr;
arr[10] = 42;  // ❌ ERROR: Index out of bounds
```

## Functions and Methods

### Function Definition

```flux
public class MathHelper {
    public int Square(int x) {
        return x * x;
    }
    
    public int Cube(int x) {
        return x * x * x;
    }
}
```

### Function Calls

```flux
MathHelper helper = new MathHelper();
int result = helper.Square(5);     // result = 25
print(result);
```

### Return Statements

```flux
public int GetValue() {
    return 42;
}

public void PrintMessage() {
    print("Hello");
    return;  // Optional for void functions
}
```

## Built-in Functions

### I/O Functions

```flux
print(value);           // Print integer or string with newline
serial_print(value);    // Print to serial port (embedded systems)
```

### Math Functions

```flux
int abs_val = abs(-42);           // Absolute value: 42
int power = pow(2, 8);            // 2^8 = 256
int max_val = max(10, 20);        // Maximum: 20
int min_val = min(10, 20);        // Minimum: 10
double sqrt_val = sqrt(16);       // Square root: 4.0
double floor_val = floor(3.7);    // Floor: 3.0
double ceil_val = ceil(3.2);      // Ceiling: 4.0
```

### String Functions

```flux
int len = string_length("hello");     // Length: 5
string concat = Concatenate("a", "b"); // "ab"
```

### Type Conversion

```flux
string s = intValue.ToString();      // Convert int to string
```

## String Operations

### String Literals

```flux
string text = "Hello, World!";
string empty = "";
string with_quotes = "She said \"Hello\"";
```

### String Manipulation

```flux
string greeting = "Hello";
string name = "Alice";
string message = Concatenate(greeting, name);  // "HelloAlice"

int len = string_length(greeting);  // 5
```

## Error Handling

### Null Safety

```flux
string text = null;
if (text != null) {
    int len = string_length(text);
}
```

### Division by Zero

```flux
int a = 10;
int b = 0;
int result = a / b;  // ❌ ERROR: Division by zero
```

### Overflow Detection

```flux
int x = 9223372036854775807;  // Max int value
int y = x + 1;                 // ❌ ERROR: Integer overflow
```

### Array Bounds

```flux
int[5] arr;
arr[10] = 42;  // ❌ ERROR: Out of bounds
```

---

For more examples, see the `examples/` directory.

