# FluxSharp Classes and Structs

Class and struct definitions.

## Class Declaration

### Basic Class

```flux
public class Person {
    public string name;
    public int age;
}
```

### Class with Methods

```flux
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
}
```

### Class with Constructor

```flux
public class Point {
    public int x;
    public int y;
    
    public Point(int px, int py) {
        this.x = px;
        this.y = py;
    }
}
```

## Class Properties

### Public Properties

```flux
public class Person {
    public string name;     // Accessible from outside
    public int age;
    public string email;
}

Person p = new Person();
p.name = "Alice";
```

### Private Properties

```flux
public class Account {
    private int balance;    // Only accessible inside class
    
    public int GetBalance() {
        return balance;
    }
}
```

### Static Properties

```flux
public class Config {
    public static int MaxConnections = 100;
}

Config.MaxConnections;
```

## Class Methods

### Instance Methods

```flux
public class Person {
    public string name;
    
    public void Greet() {
        PrintLine(name);
    }
}

Person p = new Person();
p.name = "Alice";
p.Greet();
```

### Static Methods

```flux
public class Math {
    public static int Abs(int x) {
        if (x < 0) {
            return -x;
        }
        return x;
    }
}

int value = Math.Abs(-10);
```

### Method with Parameters

```flux
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
}

Calculator c = new Calculator();
int sum = c.Add(5, 3);
```

## Constructor

### Basic Constructor

```flux
public class Person {
    public string name;
    public int age;
    
    public Person() {
        this.name = "Unknown";
        this.age = 0;
    }
}

Person p = new Person();  // Create instance
```

### Constructor with Parameters

```flux
public class Point {
    public int x;
    public int y;
    
    public Point(int px, int py) {
        this.x = px;
        this.y = py;
    }
}

Point p = new Point(10, 20);
```

## Visibility Modifiers

### Public Class

```flux
public class PublicClass {
    // Accessible from anywhere
}
```

### Private Class

```
private class InternalClass {
    // Only in module
}
```

### Public vs Private Members

```flux
public class Example {
    public int public_field;      // Accessible outside
    private int private_field;    // Only inside
}
```

## Inheritance

### Extending a Class

```flux
public class Animal {
    public string name;
    
    public void Speak() {
        PrintLine("Animal sound");
    }
}

public class Dog : Animal {
    public void Speak() {
        PrintLine("Woof!");
    }
}

Dog dog = new Dog();
dog.name = "Rex";
dog.Speak();  // "Woof!"
```

## Structs

### Struct Declaration

```flux
public struct Point {
    int x;
    int y;
}
```

### Struct with Initialization

```flux
public struct Rectangle {
    int width;
    int height;
    
    public int Area() {
        return width * height;
    }
}
```

### Struct with Attributes

```flux
[Align(4096)]
public struct Page {
    byte[4096] data;
}
```

## Class vs Struct

| Aspect | Class | Struct |
|--------|-------|--------|
| Instance creation | `new Class()` | `new Struct()` |
| Memory | Heap | Stack |
| Fields | Properties | Fields |
| Methods | Yes | Yes |

## This Keyword

Reference to current instance:

```flux
public class Person {
    public string name;
    
    public void SetName(string name) {
        this.name = name;  // Distinguish from parameter
    }
}
```

## Creating Instances

### With Constructor

```flux
Person p = new Person();

Point point = new Point(10, 20);
```

### With Field Initialization

```flux
Person p = new Person();
p.name = "Alice";
p.age = 30;
```

## Class Examples

### Simple Data Class

```flux
public class Student {
    public string name;
    public int grade;
    public int student_id;
}

Student s = new Student();
s.name = "Bob";
s.grade = 10;
```

### Class with Methods

```flux
public class BankAccount {
    private int balance;
    
    public void Deposit(int amount) {
        balance += amount;
    }
    
    public int GetBalance() {
        return balance;
    }
}

BankAccount acc = new BankAccount();
acc.Deposit(100);
int bal = acc.GetBalance();
```

### Inheritance Example

```flux
public class Shape {
    public int Area() {
        return 0;
    }
}

public class Square : Shape {
    public int size;
    
    public int Area() {
        return size * size;
    }
}

Square sq = new Square();
sq.size = 5;
int area = sq.Area();  // 25
```

## Attributes

Metadata for classes and structs:

```flux
[Align(8)]
public class AlignedClass {
    // Fields
}

[Packed]
public struct PackedStruct {
    // Fields
}
```

---

Next: Read [CONTROL_FLOW.md](CONTROL_FLOW.md)

