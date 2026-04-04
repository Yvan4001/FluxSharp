// ====================================================
// FluxSharp - Main Program - Complete Test Suite
// ====================================================

using "examples/math_helper.fsh";

public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Subtract(int a, int b) {
        return a - b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
    
    // Custom ToString() method
    public string ToString() {
        return "Calculator";
    }
}

public class Main {
    public void main() {
        // ===== Test 1: Helper Methods =====
        print("=== Test 1: Helper Methods ===");
        
        MathHelper helper = new MathHelper();
        int double21 = helper.GetDouble(21);
        print("GetDouble(21) = ");
        print(double21);
        
        int triple7 = helper.GetTriple(7);
        print("GetTriple(7) = ");
        print(triple7);
        
        // ===== Test 2: Calculator Methods =====
        print("=== Test 2: Calculator Methods ===");
        
        Calculator calc = new Calculator();
        int sum = calc.Add(10, 20);
        print("Add(10, 20) = ");
        print(sum);
        
        int diff = calc.Subtract(30, 15);
        print("Subtract(30, 15) = ");
        print(diff);
        
        int prod = calc.Multiply(5, 6);
        print("Multiply(5, 6) = ");
        print(prod);
        
        // ===== Test 3: Default ToString() for Primitive Types =====
        print("=== Test 3: ToString() for Primitive Types ===");
        
        int intValue = 42;
        float floatValue = 3.14f;
        double doubleValue = 2.71828;
        string strValue = "Hello";
        
        print("int value: ");
        print(intValue.ToString());
        
        print("float value: ");
        print(floatValue.ToString());
        
        print("double value: ");
        print(doubleValue.ToString());
        
        print("string value: ");
        print(strValue.ToString());
        
        // ===== Test 4: Math Functions =====
        print("=== Test 4: Math Functions ===");
        
        int powResult = pow(2, 3);
        print("pow(2, 3) = ");
        print(powResult);
        
        int maxResult = max(10, 20);
        print("max(10, 20) = ");
        print(maxResult);
        
        int minResult = min(10, 20);
        print("min(10, 20) = ");
        print(minResult);
        
        // ===== Test 5: Variables =====
        print("=== Test 5: Variables ===");
        
        int x = 42;
        int y = 8;
        print("Variable x = ");
        print(x);
        print("Variable y = ");
        print(y);
        
        // ===== Test 6: String Operations =====
        print("=== Test 6: String Operations ===");
        print("Hello, FluxSharp!");
        print("Program completed successfully!");
    }
}

