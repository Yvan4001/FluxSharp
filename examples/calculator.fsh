/* FluxSharp - Calculator Example */
public class Calculator {
    public int value;
    public int Add(int a, int b) {
        return a + b;
    }
    public int Multiply(int a, int b) {
        return a * b;
    }
}
public class Main {
    public void main() {
        print("Calculator Example");
        Calculator calc = new Calculator();
        calc.value = 10;
        int sum = calc.Add(5, 3);
        int product = calc.Multiply(4, 7);
        print("Done");
    }
}
