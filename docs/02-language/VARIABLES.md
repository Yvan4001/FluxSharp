# FluxSharp Variables and Constants

Variable and constant declaration based on grammar.

## Variable Declaration

### Basic Declaration

```flux
int x;              // Declaration without initialization
int y = 10;         // Declaration with initialization
string name = "Alice";
double value = 3.14;
```

### Type Inference

Type must always be specified:

```flux
int count = 42;
string text = "hello";
bool flag = true;
```

### Multiple Variables

Declare each separately:

```flux
int x = 10;
int y = 20;
int z = 30;
```

## Scope

Variables are scoped to their block:

```flux
public void Example() {
    int x = 10;         // Function scope
    
    if (x > 5) {
        int y = 20;     // Block scope
        // y visible here
    }
    // y not visible here
}
```

## Constants

### Constant Declaration

```flux
const int MAX = 100;
const string APP_NAME = "MyApp";
const double PI = 3.14159;
```

Constants:
- Must be initialized
- Cannot be changed
- Type must be specified
- Must use literal values

### Valid Constant Values

```flux
const int SIZE = 100;           // Integer literal
const string NAME = "Test";     // String literal
const double VALUE = 3.14;      // Float literal
const bool DEBUG = true;        // Boolean literal
```

## Initialization

### With Assignment

```flux
int x = 10;
string name = "Alice";
```

### Without Initialization

```flux
int x;              // Can be used later
x = 20;             // Must assign before use
```

### Multiple Assignment

```flux
int x = 10;
int y = 20;
x = y;              // Assign one variable to another
```

## Type Rules

- Every variable has explicit type
- Cannot change type after declaration
- Type checking at compile time

```flux
int x = 10;
// x = "string";     // ERROR - type mismatch

string text = "hello";
// text = 42;         // ERROR - type mismatch
```

## Visibility Modifiers

### Public Variables

Within classes, accessible from outside:

```flux
public class Person {
    public string name;     // Public access
    public int age;
}

Person p = new Person();
p.name = "Alice";           // Can access
```

### Private Variables

Within classes, accessible only inside:

```flux
public class Account {
    private int balance;    // Private access
    
    public int GetBalance() {
        return balance;     // Can access inside
    }
}

Account acc = new Account();
// acc.balance = 100;       // ERROR - private
```

## Static Variables

Class-level variables:

```flux
public class Counter {
    public static int count = 0;    // Class-level
    
    public void Increment() {
        count++;            // Shared across instances
    }
}
```

## Array Variables

### Array Declaration and Use

```flux
int[100] numbers;               // Array variable
numbers[0] = 42;                // Set element
int first = numbers[0];         // Get element

string[256] names;
names[0] = "Alice";
```

### Multi-Dimensional Arrays

```flux
int[10][20] matrix;
matrix[0][0] = 1;
int element = matrix[5][10];
```

## Variable in Loops

### For Loop Variables

```flux
for (int i = 0; i < 10; i++) {
    // i available in loop
}
// i not available here
```

### While Loop Variables

```flux
int counter = 0;
while (counter < 10) {
    // counter available
    counter++;
}
```

## Assignment Operations

### Simple Assignment

```flux
int x = 10;
x = 20;
x = 30;
```

### Arithmetic Assignment

```flux
int x = 10;
x += 5;                 // x = x + 5 (15)
x -= 3;                 // x = x - 3 (12)
x *= 2;                 // x = x * 2 (24)
x /= 4;                 // x = x / 4 (6)
```

### Increment/Decrement

```flux
int x = 10;
x++;                    // x becomes 11
x--;                    // x becomes 10
```

## Naming Conventions

- **Variables**: Use meaningful names - `count`, `name`, `totalAmount`
- **Constants**: UPPER_CASE - `MAX_SIZE`, `DEFAULT_VALUE`
- **Private fields**: _prefix or private modifier

```flux
int myVariable;
const int MAX_ITEMS = 100;

private int _internalValue;
public int PublicProperty;
```

## Variable Initialization Order

Variables initialized when declared:

```flux
const int X = 10;           // Constant initialized
int y = X + 5;              // y = 15
```

## Null Values

For array and class references:

```flux
int[100] arr;               // Can be null initially
Person p;                   // Can be null

if (arr != null) {
    arr[0] = 42;
}
```

---

Next: Read [FUNCTIONS.md](FUNCTIONS.md)

