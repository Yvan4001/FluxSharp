# FluxSharp Default Functions and ToString() Documentation

## Overview

FluxSharp now includes a comprehensive set of default built-in functions for mathematical operations and type conversion. All primitive types support the universal `ToString()` method for converting to string representation.

## Math Functions

### Basic Arithmetic Functions

#### `abs(value)` - Absolute Value
Returns the absolute value of a number.

```flux
int result = abs(-42);        // Returns 42
float result = abs(-3.14f);   // Returns 3.14
double result = abs(-2.718);  // Returns 2.718
```

**Type Support**: int, float, double  
**Return Type**: Same as input type

#### `pow(base, exponent)` - Power
Calculates base raised to the power of exponent.

```flux
int result = pow(2, 3);           // Returns 8 (2^3)
float result = pow(2.5f, 2.0f);   // Returns 6.25
double result = pow(10.0, 2.0);   // Returns 100.0
```

**Type Support**: int, float, double (all arguments must be same type)  
**Return Type**: Same as input type

#### `max(value1, value2)` - Maximum
Returns the larger of two values.

```flux
int result = max(10, 20);         // Returns 20
float result = max(3.14f, 2.71f); // Returns 3.14
```

**Type Support**: int, float, double (both arguments must be same type)  
**Return Type**: Same as input type

#### `min(value1, value2)` - Minimum
Returns the smaller of two values.

```flux
int result = min(10, 20);         // Returns 10
float result = min(3.14f, 2.71f); // Returns 2.71
```

**Type Support**: int, float, double (both arguments must be same type)  
**Return Type**: Same as input type

### Advanced Math Functions

#### `sqrt(value)` - Square Root
Calculates the square root of a number.

```flux
int result = sqrt(16);     // Returns 4 (as integer)
double result = sqrt(2.0); // Returns 1.414...
```

**Type Support**: int, float, double  
**Return Type**: double (for int input), same type otherwise

#### `floor(value)` - Floor
Returns the largest integer less than or equal to the value.

```flux
float result = floor(3.14f);   // Returns 3.0f
double result = floor(2.99);   // Returns 2.0
```

**Type Support**: int, float, double  
**Return Type**: Same as input type

#### `ceil(value)` - Ceiling
Returns the smallest integer greater than or equal to the value.

```flux
float result = ceil(3.14f);    // Returns 4.0f
double result = ceil(2.01);    // Returns 3.0
```

**Type Support**: int, float, double  
**Return Type**: Same as input type

#### `round(value)` - Round
Rounds the value to the nearest integer.

```flux
float result = round(3.14f);   // Returns 3.0f
float result = round(3.5f);    // Returns 4.0f
double result = round(2.5);    // Returns 2.0 or 3.0 (banker's rounding)
```

**Type Support**: int, float, double  
**Return Type**: Same as input type

## ToString() Method

### Overview
Every primitive type in FluxSharp supports the `ToString()` method to convert values to their string representation.

### Usage Patterns

#### Direct Function Call
```flux
int value = 42;
string result = ToString(value);  // Returns "42"
```

#### Method Call Syntax
```flux
int value = 42;
string result = value.ToString();  // Returns "42"
```

### Supported Types

#### Integer to String
```flux
int number = 42;
string str = number.ToString();    // Returns "42"

int negative = -100;
string str = negative.ToString();  // Returns "-100"
```

#### Float to String
```flux
float pi = 3.14f;
string str = pi.ToString();        // Returns "3.14"

float scientific = 1.23f;
string str = scientific.ToString(); // Returns "1.23"
```

#### Double to String
```flux
double pi = 3.14159265359;
string str = pi.ToString();        // Returns "3.14159265359"

double e = 2.71828;
string str = e.ToString();         // Returns "2.71828"
```

#### String to String
```flux
string text = "Hello";
string str = text.ToString();      // Returns "Hello" (identity)
```

### Custom ToString() Implementation in Classes

Classes can override ToString() to provide custom string representations:

```flux
public class Calculator {
    public string ToString() {
        return "Calculator[Add, Subtract, Multiply]";
    }
}

public class Main {
    public void main() {
        Calculator calc = new Calculator();
        string info = calc.ToString();
        // Returns: "Calculator[Add, Subtract, Multiply]"
    }
}
```

### Integration with print()

ToString() integrates seamlessly with the print() function:

```flux
int value = 42;
print("The value is: ");
print(value.ToString());

// Output:
// The value is:
// 42
```

## Unary Operators

### Unary Minus
Negates a numeric value.

```flux
int a = 42;
int b = -a;              // b = -42

int c = -100;            // Direct negative literal
int d = abs(-42);        // Works in function arguments
```

### Unary Plus
Explicitly marks a positive value (no effect).

```flux
int a = +42;             // a = 42
```

### Logical NOT
Inverts a boolean condition.

```flux
int condition = 1;
int result = !condition; // result = 0 (false)

int result = !0;         // result = 1 (true)
```

## Complete Example

```flux
using "examples/math_helper.fsh";

public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public string ToString() {
        return "Calculator";
    }
}

public class Main {
    public void main() {
        // Math operations
        int sum = max(10, 20);
        int product = pow(2, 3);
        
        print("Math Results:");
        print("max(10, 20) = ");
        print(sum.ToString());
        
        print("pow(2, 3) = ");
        print(product.ToString());
        
        // Using Calculator
        Calculator calc = new Calculator();
        int result = calc.Add(5, 3);
        
        print("Calculator result: ");
        print(result.ToString());
        
        // Type conversion
        float pi = 3.14f;
        double e = 2.71828;
        
        print("Pi = ");
        print(pi.ToString());
        
        print("E = ");
        print(e.ToString());
    }
}
```

## Error Handling

All functions include comprehensive error messages:

```flux
// Type mismatch error
int result = max(10, 3.14f);
// Error: max() requires both arguments to be the same numeric type

// Argument count error
int result = pow(2);
// Error: pow() requires exactly 2 arguments, but got 1

// Type error
string result = sqrt("hello");
// Error: sqrt() requires a numeric argument
```

## Performance Notes

- All math functions are compiled to native x86-64 assembly
- ToString() operations are performed at compile time when possible
- No runtime overhead for type checking due to static typing

## Limitations

1. **Custom toString() Method Dispatch**: Custom ToString() implementations in classes currently require explicit function calls. Method dispatch for ToString() will be enhanced in future versions.

2. **Floating Point Math**: Current implementations use simplified stubs. Full IEEE 754 floating-point support with SSE instructions is planned.

3. **Scientific Notation**: toString() currently uses standard decimal notation. Scientific notation output is not yet supported.

## Future Enhancements

- Format strings: `value.ToString("N2")` for precision control
- Localization support for number formatting
- Custom formatter implementations via interfaces
- Additional math functions (sin, cos, tan, log, etc.)
- Complex number support

