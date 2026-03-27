// ============================================================
// FluxSharp - Complete Language Demonstration  
// Main Example Program - English Comments
// ============================================================
// This file demonstrates all major features of FluxSharp:
// - Function definitions and calls
// - Classes and method calls
// - Control flow (loops, conditions)
// - Math operations and constants
// - Variable manipulation and arithmetic
// ============================================================
// ============================================================
// Demo 1: Basic Functions and Output
// ============================================================
void print_header() {
    serial_print("========================================");
    serial_print("FluxSharp Language Demo");
    serial_print("========================================");
}
void print_section(string name) {
    serial_print("");
    serial_print(name);
    serial_print("----------------------------------------");
}
// ============================================================
// Demo 2: Classes with Methods
// ============================================================
class MathDemo {
    // Demonstrate arithmetic operations
    public void show_addition() {
        int a = 10;
        int b = 5;
        int sum = a + b;
        serial_print("10 + 5 = ");
        serial_print(sum);
    }
    public void show_subtraction() {
        int a = 20;
        int b = 8;
        int diff = a - b;
        serial_print("20 - 8 = ");
        serial_print(diff);
    }
    public void show_multiplication() {
        int a = 7;
        int b = 6;
        int prod = a * b;
        serial_print("7 * 6 = ");
        serial_print(prod);
    }
    public void show_division() {
        int a = 20;
        int b = 4;
        int quot = a / b;
        serial_print("20 / 4 = ");
        serial_print(quot);
    }
}
// ============================================================
// Demo 3: Loop Control Flow
// ============================================================
void show_counting() {
    serial_print("Counting 0 to 4:");
    int i = 0;
    while (i < 5) {
        serial_print(i);
        i = i + 1;
    }
}
// ============================================================
// Demo 4: Math Constants and Functions
// ============================================================
void show_math_constants() {
    serial_print("");
    serial_print("Math Constants and Functions:");
    serial_print("----------------------------------------");
    double pi = PI;
    serial_print("PI = ");
    serial_print(pi);
    double e = E;
    serial_print("E = ");
    serial_print(e);
    double sqrt16 = sqrt(16);
    serial_print("sqrt(16) = ");
    serial_print(sqrt16);
    double pow23 = pow(2, 3);
    serial_print("pow(2, 3) = ");
    serial_print(pow23);
}
// ============================================================
// MAIN PROGRAM
// ============================================================
void main() {
    // Program header
    print_header();
    // Demo 1: Arithmetic operations with classes
    print_section("Demo 1: Arithmetic Operations");
    MathDemo demo;
    demo.show_addition();
    demo.show_subtraction();
    demo.show_multiplication();
    demo.show_division();
    // Demo 2: Control flow
    print_section("Demo 2: Loops");
    show_counting();
    // Demo 3: Math functions
    show_math_constants();
    // Program footer
    serial_print("");
    serial_print("========================================");
    serial_print("Program Complete!");
    serial_print("FluxSharp v1.0 - Ready for Production");
    serial_print("========================================");
}
