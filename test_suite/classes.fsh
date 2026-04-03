public class Main {
    public int value;
    
    public void setValue(int v) {
        value = v;
    }
    
    public void main() {
        Main obj = new Main();
        obj.setValue(42);
        print("✅ Classes: PASS");
    }
}
