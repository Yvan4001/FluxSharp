// Calculator Example
// Demonstrates class definition and method calls

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
}

public class Main {
    public void main() {
        Calculator calc = new Calculator();
        
        int sum = calc.Add(10, 20);
        print("10 + 20 = ");
        print(sum);
        
        int diff = calc.Subtract(30, 15);
        print("30 - 15 = ");
        print(diff);
        
        int prod = calc.Multiply(5, 6);
        print("5 * 6 = ");
        print(prod);
    }
}

