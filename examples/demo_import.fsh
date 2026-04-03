import "math_helper.fsh";

public class Main {
    public void main() {
        MathHelper math = new MathHelper();
        
        int value = math.GetDouble(15);
        print("Result: ");
        print(value);
    }
}

