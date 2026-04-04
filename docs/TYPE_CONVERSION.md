# Type Conversion and ToString() in FluxSharp

## Type System Overview

FluxSharp is a statically-typed language with strong type checking. However, it provides automatic and explicit conversion mechanisms through the ToString() function and type casting.

## Primitive Types

FluxSharp supports the following primitive types:

| Type | Size | Range | Example |
|------|------|-------|---------|
| `int` | 64-bit | -2^63 to 2^63-1 | `42`, `-100`, `0` |
| `float` | 32-bit | ±3.4E±38 | `3.14f`, `-2.5f` |
| `double` | 64-bit | ±1.7E±308 | `3.14159`, `2.71828` |
| `string` | Variable | N/A | `"Hello"`, `"FluxSharp"` |
| `bool` | 1-bit | true/false | `true`, `false` |

## Implicit Type Conversion Rules

### Numeric Type Hierarchy
```
int → float → double
```

The compiler automatically converts "smaller" types to "larger" types when mixing types in operations:

```flux
int a = 10;
float b = 3.14f;
double result = a + b;  // int converted to double, float to double
                        // result = 13.14
```

### Rules
1. **int + float** → float
2. **int + double** → double
3. **float + double** → double
4. **Mixing with string** → String concatenation or ToString() required

## Explicit Conversion via ToString()

### Converting to String

#### From Integer
```flux
int value = 42;
string str = value.ToString();
print(str);  // Output: 42
```

#### From Float
```flux
float pi = 3.14f;
string str = pi.ToString();
print(str);  // Output: 3.14
```

#### From Double
```flux
double e = 2.71828;
string str = e.ToString();
print(str);  // Output: 2.71828
```

#### From String (Identity)
```flux
string text = "Hello";
string result = text.ToString();
// result = "Hello" (unchanged)
```

## ToString() in Print Statements

The most common use of ToString() is for formatted output:

```flux
int x = 100;
int y = 200;

print("X = ");
print(x.ToString());

print("Y = ");
print(y.ToString());

print("Sum = ");
int sum = x + y;
print(sum.ToString());

// Output:
// X = 
// 100
// Y = 
// 200
// Sum = 
// 300
```

## Advanced Type Conversion Scenarios

### Converting Between Numeric Types

#### Integer Arithmetic with Float Input
```flux
int a = 10;
float b = 3.14f;

// Automatic conversion happens
float result = a + b;        // = 13.14

// For integer results, truncate explicitly
int truncated = (int)a;      // = 10
```

### Function Argument Type Matching

Math functions require type-consistent arguments:

```flux
int a = 10;
int b = 20;
int max_int = max(a, b);    // ✅ Works: both int

float x = 3.14f;
float y = 2.71f;
float max_float = max(x, y); // ✅ Works: both float

// ❌ Type Error: different types
int bad = max(a, x);         // ERROR: mixing int and float
```

### toString() with Variables

```flux
public class Person {
    public int age;
    
    public string GetAge() {
        return age.ToString();
    }
}

public class Main {
    public void main() {
        Person p = new Person();
        p.age = 25;
        
        string ageStr = p.GetAge();  // "25"
        print("Age: ");
        print(ageStr);
    }
}
```

## Custom toString() Implementation in Classes

Classes can override toString() to provide meaningful string representations:

```flux
public class Point {
    public int x;
    public int y;
    
    public string ToString() {
        return "Point";  // Simplified version
        // Full implementation would concatenate: "Point(x, y)"
    }
}

public class Person {
    public string name;
    public int age;
    
    public string ToString() {
        return "Person";  // Returns type name
    }
}

public class Main {
    public void main() {
        Point p = new Point();
        print("Object: ");
        print(p.ToString());  // Output: Point
        
        Person person = new Person();
        print("Object: ");
        print(person.ToString());  // Output: Person
    }
}
```

## Compilation-Time Type Checking

FluxSharp performs strict type checking at compile time:

```flux
// ❌ Type Error
int x = 3.14f;        // Cannot assign float to int

// ✅ Works: float can hold int implicitly
float y = 42;         // Implicit conversion

// ✅ Works: toString() provides explicit conversion
string z = 42.ToString();  // Explicit to string
```

## toString() Performance Characteristics

### Compile-Time Evaluation
When ToString() is called with compile-time constants, the conversion happens during compilation:

```flux
// Compile-time (no runtime cost)
string pi = 3.14.ToString();

// Runtime (minimal cost)
double value = read_input();
string str = value.ToString();
```

### Memory Efficiency
- toString() creates new string objects
- No caching of converted strings (each call creates new string)
- Consider reusing converted strings if called frequently:

```flux
// Less efficient: converts twice
print(value.ToString());
print(value.ToString());

// More efficient: convert once
string str = value.ToString();
print(str);
print(str);
```

## Type Safety Examples

### Correct Usage
```flux
int a = 10;
int b = 20;
int sum = a + b;              // ✅ int + int = int

float x = 3.14f;
float y = 2.71f;
float product = x * y;        // ✅ float * float = float

string result = sum.ToString() + product.ToString();
                              // ✅ string + string = string
```

### Error Cases
```flux
// ❌ Cannot mix types without conversion
int bad1 = 3.14f + 10;        // ERROR

// ❌ Cannot assign incompatible types
float bad2 = "3.14";          // ERROR

// ❌ toString() on unknown type
string bad3 = some_unknown.ToString();  // ERROR: undefined

// ✅ Explicit conversion
int value = (int)3.14f;       // Explicit cast
string str = value.ToString(); // Explicit to string
```

## Working with Strings

### String Concatenation Pattern
```flux
int x = 42;
string message = "The answer is: " + x.ToString();
print(message);  // Output: The answer is: 42
```

### Building Formatted Output
```flux
int hours = 14;
int minutes = 30;
string time = hours.ToString() + ":" + minutes.ToString();
print(time);  // Output: 14:30
```

## Summary

| Operation | Example | Result |
|-----------|---------|--------|
| int → string | `42.ToString()` | `"42"` |
| float → string | `3.14f.ToString()` | `"3.14"` |
| double → string | `2.718.ToString()` | `"2.718"` |
| string → string | `"hello".ToString()` | `"hello"` |
| Mixed numeric | `10 + 3.14f` | `13.14f` |
| Type error | `int x = 3.14f;` | Compilation error |

## Best Practices

1. **Always use ToString() for string conversion** - It's the standard way in FluxSharp
2. **Group conversions with their use** - Convert immediately before printing or concatenating
3. **Reuse converted strings** - Store result if using multiple times
4. **Use meaningful class toString()** - Override ToString() in custom classes for debugging
5. **Prefer type-safe operations** - Keep types consistent when possible

## Next Features

Planned enhancements to the type conversion system:
- Format strings: `value.ToString("F2")` for precision control
- Parse functions: `int.Parse("42")` for string to type conversion
- Implicit operator overloading in custom classes
- Type inference improvements

