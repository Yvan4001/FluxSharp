/* FluxSharp - Complete Feature Demonstration */

// Constants
const int MAX_SIZE = 100;
const double PI = 3.14159;

// Struct definition
public struct Point {
    int x;
    int y;
}

// Class definition
public class Calculator {
    public int value;
    
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
}

// Simple function
public void PrintMessage(string msg) {
    print(msg);
    return;
}

// Function with array
public void DemoArrays() {
    int[10] numbers;
    
    numbers[0] = 10;
    numbers[1] = 20;
    numbers[2] = 30;
    
    int first = numbers[0];
    int second = numbers[1];
    int third = numbers[2];
    
    print("Array values:");
    return;
}

// Function with class
public void DemoClass() {
    Calculator calc = new Calculator();
    
    calc.value = 42;
    
    int sum = calc.Add(5, 3);
    int product = calc.Multiply(4, 7);
    
    print("Calculator demo");
    return;
}

// Control flow demo
public void DemoControlFlow() {
    for (int i = 0; i < 5; i++) {
        print("Loop");
    }
    
    return;
}

// Main entry point
public static void Main() {
    print("=== FluxSharp Features ===");
    
    PrintMessage("Constants and basic types work");
    
    DemoArrays();
    
    DemoClass();
    
    DemoControlFlow();
    
    print("=== Complete ===");
    return;
}

