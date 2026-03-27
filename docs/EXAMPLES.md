# FluxSharp Code Examples

A collection of working examples to learn FluxSharp programming.

---

## Table of Contents

1. [Hello World](#hello-world)
2. [Variables & Types](#variables--types)
3. [Math Operations](#math-operations)
4. [Control Flow](#control-flow)
5. [Functions](#functions)
6. [Classes](#classes)
7. [Advanced](#advanced)

---

## Hello World

### Simplest Program

```flux
void main() {
    serial_print("Hello, World!");
}
```

**Output:**
```
Hello, World!
```

---

## Variables & Types

### Integer Types

```flux
void main() {
    int x = 42;
    uint y = 100;
    byte b = 255;
    
    serial_print("int: ");
    serial_print(x);
    
    serial_print("uint: ");
    serial_print(y);
    
    serial_print("byte: ");
    serial_print(b);
}
```

**Output:**
```
int: 42
uint: 100
byte: 255
```

### Floating Point

```flux
void main() {
    float f = 3.14f;      // Note the 'f' suffix
    double d = 2.71828;   // No suffix needed
    
    serial_print("float: ");
    serial_print(f);
    
    serial_print("double: ");
    serial_print(d);
}
```

**Output:**
```
float: 3.14
double: 2.71828
```

### Strings & Booleans

```flux
void main() {
    string greeting = "Welcome!";
    bool is_active = true;
    
    serial_print(greeting);
    serial_print("Active: ");
    serial_print(is_active);
}
```

**Output:**
```
Welcome!
Active: true
```

---

## Math Operations

### Basic Arithmetic

```flux
void main() {
    int a = 10;
    int b = 3;
    
    int sum = a + b;       // 13
    int diff = a - b;      // 7
    int product = a * b;   // 30
    int quotient = a / b;  // 3
    int remainder = a % b; // 1
    
    serial_print("Sum: ");
    serial_print(sum);
    serial_print("Difference: ");
    serial_print(diff);
    serial_print("Product: ");
    serial_print(product);
    serial_print("Quotient: ");
    serial_print(quotient);
    serial_print("Remainder: ");
    serial_print(remainder);
}
```

**Output:**
```
Sum: 13
Difference: 7
Product: 30
Quotient: 3
Remainder: 1
```

### Math Functions

```flux
void main() {
    // Basic functions
    int abs_val = abs(-5);          // 5
    double sqrt_val = sqrt(16);     // 4.0
    double floor_val = floor(3.7);  // 3.0
    double ceil_val = ceil(3.2);    // 4.0
    double round_val = round(3.5);  // 4.0
    
    serial_print("abs(-5) = ");
    serial_print(abs_val);
    
    serial_print("sqrt(16) = ");
    serial_print(sqrt_val);
    
    serial_print("floor(3.7) = ");
    serial_print(floor_val);
    
    serial_print("ceil(3.2) = ");
    serial_print(ceil_val);
    
    serial_print("round(3.5) = ");
    serial_print(round_val);
}
```

### Trigonometry & Logarithm

```flux
void main() {
    // Math constants
    double pi = PI;
    double e = E;
    
    // Trigonometric
    double sin_val = sin(0);     // 0
    double cos_val = cos(0);     // 1
    double tan_val = tan(0);     // 0
    
    // Logarithmic
    double ln_val = ln(e);       // ~1
    double log_val = log10(100); // 2
    
    // Power
    double power_val = pow(2, 3); // 8
    
    serial_print("PI = ");
    serial_print(pi);
    serial_print("E = ");
    serial_print(e);
    serial_print("sin(0) = ");
    serial_print(sin_val);
    serial_print("cos(0) = ");
    serial_print(cos_val);
    serial_print("pow(2, 3) = ");
    serial_print(power_val);
}
```

---

## Control Flow

### If-Else

```flux
void check_age(int age) {
    if (age < 13) {
        serial_print("Child");
    } else if (age < 18) {
        serial_print("Teenager");
    } else {
        serial_print("Adult");
    }
}

void main() {
    check_age(10);  // Output: Child
    check_age(15);  // Output: Teenager
    check_age(25);  // Output: Adult
}
```

### While Loop

```flux
void main() {
    serial_print("Count: ");
    int i = 0;
    while (i < 5) {
        serial_print(i);
        i = i + 1;
    }
    // Output: 0 1 2 3 4
}
```

### Conditional Operations

```flux
void check_number(int x) {
    if (x > 0) {
        serial_print("Positive");
    } else if (x < 0) {
        serial_print("Negative");
    } else {
        serial_print("Zero");
    }
}

void main() {
    check_number(5);   // Positive
    check_number(-3);  // Negative
    check_number(0);   // Zero
}
```

---

## Functions

### Simple Function

```flux
void greet() {
    serial_print("Hello!");
}

void main() {
    greet();  // Output: Hello!
}
```

### Function with Parameters

```flux
void add(int a, int b) {
    int sum = a + b;
    serial_print("Sum: ");
    serial_print(sum);
}

void main() {
    add(5, 3);   // Sum: 8
    add(10, 20); // Sum: 30
}
```

### Multiple Functions

```flux
void multiply(int x, int y) {
    int result = x * y;
    serial_print("Product: ");
    serial_print(result);
}

void divide(int x, int y) {
    if (y != 0) {
        int result = x / y;
        serial_print("Quotient: ");
        serial_print(result);
    } else {
        serial_print("Error: Division by zero");
    }
}

void main() {
    multiply(4, 5);  // Product: 20
    divide(10, 2);   // Quotient: 5
    divide(10, 0);   // Error: Division by zero
}
```

---

## Classes

### Simple Class

```flux
class Counter {
    public int value;
    
    public void increment() {
        value = value + 1;
    }
    
    public void print_value() {
        serial_print("Value: ");
        serial_print(value);
    }
}

void main() {
    Counter c;
    c.value = 0;
    c.increment();
    c.print_value();  // Value: 1
}
```

### Class with Methods

```flux
class Rectangle {
    public int width;
    public int height;
    
    public void calculate_area() {
        int area = width * height;
        serial_print("Area: ");
        serial_print(area);
    }
    
    public void calculate_perimeter() {
        int perimeter = 2 * (width + height);
        serial_print("Perimeter: ");
        serial_print(perimeter);
    }
}

void main() {
    Rectangle r;
    r.width = 5;
    r.height = 3;
    r.calculate_area();       // Area: 15
    r.calculate_perimeter();  // Perimeter: 16
}
```

---

## Advanced

### Combining Features

```flux
void calculate_circle_stats(double radius) {
    double pi = PI;
    double area = pi * radius * radius;
    double circumference = 2 * pi * radius;
    
    serial_print("Radius: ");
    serial_print(radius);
    serial_print("Area: ");
    serial_print(area);
    serial_print("Circumference: ");
    serial_print(circumference);
}

void main() {
    calculate_circle_stats(5);  // r=5
}
```

### String Operations

```flux
void main() {
    string first = "Hello";
    string second = " ";
    string third = "World";
    
    string full = first + second + third;
    serial_print(full);  // Hello World
}
```

### Complex Calculation

```flux
void calculate_savings(int monthly, int months, double rate) {
    int total = 0;
    int month = 0;
    
    while (month < months) {
        total = total + monthly;
        month = month + 1;
    }
    
    serial_print("Total saved: ");
    serial_print(total);
    
    double interest_earned = total * rate;
    serial_print("Interest: ");
    serial_print(interest_earned);
}

void main() {
    calculate_savings(100, 12, 0.05);
    // Total saved: 1200
    // Interest: 60
}
```

---

## Running These Examples

1. Copy any example code to a file (e.g., `example.fsh`)
2. Compile: `fluxc compile example.fsh -o example`
3. Run: `./example`

---

## Learning Suggestions

**Beginner:**
- Start with Hello World
- Try Variables & Types examples
- Experiment with Math Operations

**Intermediate:**
- Learn Control Flow (if/while)
- Write your own functions
- Combine multiple features

**Advanced:**
- Create classes
- Combine multiple features
- Build complex programs

---

See **LANGUAGE_GUIDE.md** for complete reference of all features.

