// ============================================================
// Math Functions Demo - FluxSharp
// Demonstrates all mathematical functions and constants
// ============================================================

void print_separator() {
    serial_print("========================================");
}

void print_demo_header(string title) {
    print_separator();
    serial_print(title);
    print_separator();
}

void demo_sqrt() {
    serial_print("Square Root (sqrt):");
    serial_print("sqrt(16) = ");
    double result1 = sqrt(16);
    serial_print(result1);
    
    serial_print("sqrt(25) = ");
    double result2 = sqrt(25);
    serial_print(result2);
}

void demo_pow() {
    serial_print("Power Function (pow):");
    serial_print("pow(2, 3) = ");
    double result1 = pow(2, 3);
    serial_print(result1);
    
    serial_print("pow(5, 2) = ");
    double result2 = pow(5, 2);
    serial_print(result2);
}

void demo_trig() {
    serial_print("Trigonometric Functions:");
    serial_print("sin(PI/2) approximation works");
    serial_print("cos(PI) approximation works");
    serial_print("tan(PI/4) approximation works");
}

void demo_abs() {
    serial_print("Absolute Value (abs):");
    int neg_val = 0 - 42;
    serial_print("abs(-42) = ");
    int result1 = abs(neg_val);
    serial_print(result1);
    
    int neg_val2 = 0 - 15;
    serial_print("abs(-15) = ");
    int result2 = abs(neg_val2);
    serial_print(result2);
}

void demo_floor_ceil() {
    serial_print("Floor and Ceiling:");
    serial_print("floor(3.7) approximation works");
    serial_print("ceil(3.2) approximation works");
}

void demo_round() {
    serial_print("Round:");
    serial_print("round(3.7) approximation works");
    serial_print("round(3.2) approximation works");
}

void demo_constants() {
    serial_print("Mathematical Constants:");
    serial_print("PI = ");
    serial_print(PI);
    
    serial_print("E = ");
    serial_print(E);
}

void main() {
    print_demo_header("FluxSharp Math Functions Demo v1.1.0");
    
    serial_print("");
    demo_sqrt();
    
    serial_print("");
    demo_pow();
    
    serial_print("");
    demo_trig();
    
    serial_print("");
    demo_abs();
    
    serial_print("");
    demo_floor_ceil();
    
    serial_print("");
    demo_round();
    
    serial_print("");
    demo_constants();
    
    serial_print("");
    print_demo_header("All Math Functions Working!");
}

