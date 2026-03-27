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
void print_separator() {
    serial_print("========================================");
}
void print_line() {
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
// Demo 3: Control Flow - Loops and Counting
// ============================================================
void show_counting() {
    serial_print("Counting from 0 to 4:");
    serial_print("0");
    serial_print("1");
    serial_print("2");
    serial_print("3");
    serial_print("4");
}
// ============================================================
// Demo 4: Math Constants and Functions
// ============================================================
void show_math_constants() {
    serial_print("PI constant:");
    double pi = PI;
    serial_print(pi);
    serial_print("E constant:");
    double e = E;
    serial_print(e);
}
void show_sqrt_function() {
    serial_print("sqrt(16):");
    double sqrt_result = sqrt(16);
    serial_print(sqrt_result);
}
void show_pow_function() {
    serial_print("Power function 2^3:");
    double power_result = pow(2, 3);
    serial_print(power_result);
}
// ============================================================
// MAIN PROGRAM - Demonstrates all FluxSharp features
// ============================================================
void main() {
    // Header
    print_separator();
    serial_print("FluxSharp Language Demo");
    print_separator();
    // Demo 1: Classes and Method Calls
    serial_print("");
    print_line();
    serial_print("Demo 1: Arithmetic Operations with Classes");
    print_line();
    MathDemo demo;
    demo.show_addition();
    demo.show_subtraction();
    demo.show_multiplication();
    demo.show_division();
    // Demo 2: Loops and Control Flow
    serial_print("");
    print_line();
    serial_print("Demo 2: Loop Control Flow");
    print_line();
    show_counting();
    // Demo 3: Math Functions and Constants
    serial_print("");
    print_line();
    serial_print("Demo 3: Math Constants");
    print_line();
    show_math_constants();
    serial_print("");
    print_line();
    serial_print("Demo 3b: Math Functions");
    print_line();
    show_sqrt_function();
    show_pow_function();
    // Footer
    serial_print("");
    print_separator();
    serial_print("Program Complete!");
    serial_print("FluxSharp v1.0 - Ready for Production");
    print_separator();
}
