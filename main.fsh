// ============================================================
// FluxSharp - Complete Language Demonstration
// Main Example Program - English Comments
// ============================================================
// This file demonstrates all major features of FluxSharp:
// - Class definition and instantiation
// - Method calls on objects
// - Property access and modification
// - Control flow (loops, conditions)
// - Math operations
// - Function definitions and calls
// ============================================================
// ============================================================
// Helper function: Print separator line
// ============================================================
void print_separator() {
    serial_print("================================================");
}
// ============================================================
// Example 1: Data Container Class
// Store and display basic information
// ============================================================
class Person {
    public string name;
    public int age;
    public void display_info() {
        serial_print("Person: ");
        serial_print(name);
        serial_print("Age: ");
        serial_print(age);
    }
}
// ============================================================
// Example 2: Calculator Class
// Perform basic arithmetic operations
// ============================================================
class Calculator {
    public int value;
    public void add(int x) {
        value = value + x;
    }
    public void multiply(int x) {
        value = value * x;
    }
    public void reset() {
        value = 0;
    }
    public void show() {
        serial_print("Calculator value: ");
        serial_print(value);
    }
}
// ============================================================
// Example 3: Printer Class
// Demonstrate method calls without state modification
// ============================================================
class Printer {
    public void print_title(string title) {
        serial_print("=== ");
        serial_print(title);
        serial_print(" ===");
    }
    public void print_status() {
        serial_print("Status: OK");
    }
}
// ============================================================
// MAIN PROGRAM
// Demonstrates all features
// ============================================================
void main() {
    // ========================================================
    // Demo 1: Class Instantiation and Property Assignment
    // ========================================================
    print_separator();
    serial_print("Demo 1: Person Class");
    print_separator();
    Person p;
    p.name = "Alice";
    p.age = 30;
    p.display_info();
    // ========================================================
    // Demo 2: Calculator with Method Calls
    // ========================================================
    serial_print("");
    print_separator();
    serial_print("Demo 2: Calculator Class");
    print_separator();
    Calculator calc;
    calc.value = 10;
    serial_print("Initial value:");
    serial_print(calc.value);
    calc.add(5);
    serial_print("After add(5):");
    serial_print(calc.value);
    calc.multiply(2);
    serial_print("After multiply(2):");
    serial_print(calc.value);
    calc.show();
    // ========================================================
    // Demo 3: Control Flow with Methods
    // ========================================================
    serial_print("");
    print_separator();
    serial_print("Demo 3: Loop and Methods");
    print_separator();
    Printer printer;
    printer.print_title("Counting");
    int count = 0;
    while (count < 5) {
        serial_print(count);
        count = count + 1;
    }
    // ========================================================
    // Demo 4: Math Operations
    // ========================================================
    serial_print("");
    print_separator();
    serial_print("Demo 4: Math Operations");
    print_separator();
    int a = 15;
    int b = 3;
    serial_print("Arithmetic operations:");
    serial_print(a + b);
    serial_print(a - b);
    serial_print(a * b);
    serial_print(a / b);
    // ========================================================
    // Demo 5: Floating Point and Constants
    // ========================================================
    serial_print("");
    print_separator();
    serial_print("Demo 5: Math Constants");
    print_separator();
    double pi = PI;
    serial_print("PI =");
    serial_print(pi);
    double e = E;
    serial_print("E =");
    serial_print(e);
    double sqrt_val = sqrt(16);
    serial_print("sqrt(16) =");
    serial_print(sqrt_val);
    // ========================================================
    // Program Complete
    // ========================================================
    serial_print("");
    print_separator();
    serial_print("Program Complete!");
    print_separator();
}
