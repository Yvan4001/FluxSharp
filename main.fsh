/* ========================================================
   FluxSharp - Complete Language Feature Demonstration
   Main entry point showing all documentation cases
   ======================================================== */

// ========================================================
// CONSTANTS - const declaration
// ========================================================
const int MAX_SIZE = 100;
const double PI = 3.14159;

// ========================================================
// STRUCTS - struct definition with fields
// ========================================================
public struct Point {
    int x;
    int y;
}

// ========================================================
// CLASSES - class definition with properties and methods
// ========================================================
public class Calculator {
    public int value;
    
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
}

// ========================================================
// TYPES AND VARIABLES - all primitive types
// ========================================================
public void DemoTypes() {
    print("=== Types and Variables ===");
    
    int intVal = 42;
    uint uintVal = 100;
    long longVal = 9999999;
    float floatVal = 3.14f;
    double doubleVal = 3.14159;
    byte byteVal = 255;
    string stringVal = "FluxSharp";
    bool boolVal = true;
    
    print("Types initialized");
}

// ========================================================
// FUNCTIONS - function with parameters and return
// ========================================================
public int Add(int a, int b) {
    return a + b;
}

public string Concatenate(string a, string b) {
    return a + b;
}

public void PrintValue(int val) {
    print("Value");
}

// ========================================================
// OPERATORS - arithmetic, bitwise, logical, comparison
// ========================================================
public void DemoOperators() {
    print("=== Operators ===");
    
    // Arithmetic operators
    int sum = 5 + 3;          // Addition
    int diff = 10 - 3;        // Subtraction
    int prod = 4 * 5;         // Multiplication
    int quot = 20 / 4;        // Division
    int rem = 17 % 5;         // Modulo
    
    // Bitwise operators
    int bitwiseAnd = 5 & 3;   // AND
    int bitwiseOr = 5 | 3;    // OR
    int bitwiseXor = 5 ^ 3;   // XOR
    int leftShift = 5 << 1;   // Left shift
    int rightShift = 10 >> 1; // Right shift
    
    print("Operators done");
}

// ========================================================
// ARRAYS - array declaration, access, manipulation
// ========================================================
public void DemoArrays() {
    print("=== Arrays ===");
    
    int[10] numbers;
    
    // Array element assignment
    numbers[0] = 10;
    numbers[1] = 20;
    numbers[2] = 30;
    numbers[3] = 40;
    numbers[4] = 50;
    
    // Array element access
    int first = numbers[0];
    int second = numbers[1];
    int third = numbers[2];
    
    print("Arrays initialized");
}

// ========================================================
// CONTROL FLOW - if/else statements
// ========================================================
public void DemoIfElse() {
    print("=== If/Else ===");
    
    int x = 15;
    
    if (x > 10) {
        print("Greater");
    } else {
        print("Lesser");
    }
}

// ========================================================
// CONTROL FLOW - for loops
// ========================================================
public void DemoForLoop() {
    print("=== For Loop ===");
    
    for (int i = 0; i < 5; i++) {
        print("Iteration");
    }
}

// ========================================================
// CONTROL FLOW - while loops
// ========================================================
public void DemoWhileLoop() {
    print("=== While Loop ===");
    
    int counter = 0;
    while (counter < 3) {
        print("Loop");
        counter++;
    }
}

// ========================================================
// CLASSES - instantiation with new, method calls
// ========================================================
public void DemoClasses() {
    print("=== Classes ===");
    
    Calculator calc = new Calculator();
    
    calc.value = 42;
    
    int sum = calc.Add(5, 3);
    int product = calc.Multiply(4, 7);
    
    print("Calculator done");
}

// ========================================================
// FUNCTION CALLS - calling functions with parameters
// ========================================================
public void DemoFunctionCalls() {
    print("=== Function Calls ===");
    
    int result = Add(10, 20);
    string combined = Concatenate("Hello", "World");
    PrintValue(42);
    
    print("Functions done");
}

// ========================================================
// MATH FUNCTIONS - standard math operations
// ========================================================
public void DemoMathFunctions() {
    print("=== Math Functions ===");
    
    int abs_val = abs(0 - 42);
    int max_val = max(10, 5);
    int min_val = min(10, 5);
    int power_val = pow(2, 3);
    
    print("Math done");
}

// ========================================================
// STRING OPERATIONS - string manipulation
// ========================================================
public void DemoStrings() {
    print("=== Strings ===");
    
    string text = "FluxSharp";
    string greeting = "Hello" + " " + "World";
    
    print(greeting);
}

// ========================================================
// CONTROL FLOW - break and continue
// ========================================================
public void DemoBreakContinue() {
    print("=== Break/Continue ===");
    
    for (int i = 0; i < 10; i++) {
        if (i == 3) {
            continue;
        }
        if (i == 7) {
            break;
        }
        print("Loop");
    }
}

// ========================================================
// FUNCTION WITH EARLY RETURN
// ========================================================
public int ValidateNumber(int n) {
    if (n < 0) {
        return 0 - 1;
    }
    
    if (n > MAX_SIZE) {
        return 0 - 1;
    }
    
    return n;
}

// ========================================================
// MAIN ENTRY POINT - demonstrates all features
// ========================================================
public static void Main() {
    print("=============================================");
    print("FluxSharp - Complete Language Demonstration");
    print("=============================================");
    
    // Call all demo functions
    DemoTypes();
    
    DemoOperators();
    
    DemoArrays();
    
    DemoIfElse();
    
    DemoForLoop();
    
    DemoWhileLoop();
    
    DemoClasses();
    
    DemoFunctionCalls();
    
    DemoMathFunctions();
    
    DemoStrings();
    
    DemoBreakContinue();
    
    print("=============================================");
    print("All demonstrations complete!");
    print("=============================================");
}

