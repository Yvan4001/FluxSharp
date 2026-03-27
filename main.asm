; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh"
; fluxc v0.1.0
; ============================

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Calculator_Add
Calculator_Add:
    push rbp
    mov rbp, rsp

    ; --- return a + b; ---

    mov rsp, rbp
    pop rbp
    ret

global Calculator_Multiply
Calculator_Multiply:
    push rbp
    mov rbp, rsp

    ; --- return a * b; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoTypes
DemoTypes:
    push rbp
    mov rbp, rsp

    ; --- print("=== Types and Variables ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int intVal = 42; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- uint uintVal = 100; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- long longVal = 9999999; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- float floatVal = 3.14f; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- double doubleVal = 3.14159; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- byte byteVal = 255; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- string stringVal = "FluxSharp"; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- bool boolVal = true; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Types initialized"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global Add
Add:
    push rbp
    mov rbp, rsp

    ; --- return a + b; ---

    mov rsp, rbp
    pop rbp
    ret


; ---  ---
global Concatenate
Concatenate:
    push rbp
    mov rbp, rsp

    ; --- return a + b; ---

    mov rsp, rbp
    pop rbp
    ret


; ---  ---
global PrintValue
PrintValue:
    push rbp
    mov rbp, rsp

    ; --- print("Value"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoOperators
DemoOperators:
    push rbp
    mov rbp, rsp

    ; --- print("=== Operators ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int sum = 5 + 3;          // Addition ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int diff = 10 - 3;        // Subtraction ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int prod = 4 * 5;         // Multiplication ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int quot = 20 / 4;        // Division ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int rem = 17 % 5;         // Modulo ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int bitwiseAnd = 5 & 3;   // AND ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int bitwiseOr = 5 | 3;    // OR ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int bitwiseXor = 5 ^ 3;   // XOR ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int leftShift = 5 << 1;   // Left shift ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int rightShift = 10 >> 1; // Right shift ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Operators done"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoArrays
DemoArrays:
    push rbp
    mov rbp, rsp

    ; --- print("=== Arrays ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int[10] numbers; ---

    ; --- numbers[0] = 10; ---

    ; --- numbers[1] = 20; ---

    ; --- numbers[2] = 30; ---

    ; --- numbers[3] = 40; ---

    ; --- numbers[4] = 50; ---

    ; --- int first = numbers[0]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int second = numbers[1]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int third = numbers[2]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Arrays initialized"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoIfElse
DemoIfElse:
    push rbp
    mov rbp, rsp

    ; --- print("=== If/Else ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int x = 15; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- if (x > 10) { ---

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoForLoop
DemoForLoop:
    push rbp
    mov rbp, rsp

    ; --- print("=== For Loop ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- for (int i = 0; i < 5; i++) { ---

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoWhileLoop
DemoWhileLoop:
    push rbp
    mov rbp, rsp

    ; --- print("=== While Loop ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int counter = 0; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- while (counter < 3) { ---

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoClasses
DemoClasses:
    push rbp
    mov rbp, rsp

    ; --- print("=== Classes ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- calc.value = 42; ---

    ; --- int sum = calc.Add(5, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int product = calc.Multiply(4, 7); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Calculator done"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoFunctionCalls
DemoFunctionCalls:
    push rbp
    mov rbp, rsp

    ; --- print("=== Function Calls ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int result = Add(10, 20); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- string combined = Concatenate("Hello", "World"); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- PrintValue(42); ---
    ; Function call: PrintValue
    call PrintValue

    ; --- print("Functions done"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoMathFunctions
DemoMathFunctions:
    push rbp
    mov rbp, rsp

    ; --- print("=== Math Functions ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- int abs_val = abs(0 - 42); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int max_val = max(10, 5); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int min_val = min(10, 5); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int power_val = pow(2, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Math done"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoStrings
DemoStrings:
    push rbp
    mov rbp, rsp

    ; --- print("=== Strings ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- string text = "FluxSharp"; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- string greeting = "Hello" + " " + "World"; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print(greeting); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoBreakContinue
DemoBreakContinue:
    push rbp
    mov rbp, rsp

    ; --- print("=== Break/Continue ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- for (int i = 0; i < 10; i++) { ---

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global ValidateNumber
ValidateNumber:
    push rbp
    mov rbp, rsp

    ; --- if (n < 0) { ---

    ; --- if (n > MAX_SIZE) { ---

    ; --- return n; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- print("============================================="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("FluxSharp - Complete Language Demonstration"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("============================================="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoTypes(); ---
    ; Function call: DemoTypes
    call DemoTypes

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoOperators(); ---
    ; Function call: DemoOperators
    call DemoOperators

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoArrays(); ---
    ; Function call: DemoArrays
    call DemoArrays

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoIfElse(); ---
    ; Function call: DemoIfElse
    call DemoIfElse

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoForLoop(); ---
    ; Function call: DemoForLoop
    call DemoForLoop

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoWhileLoop(); ---
    ; Function call: DemoWhileLoop
    call DemoWhileLoop

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoClasses(); ---
    ; Function call: DemoClasses
    call DemoClasses

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoFunctionCalls(); ---
    ; Function call: DemoFunctionCalls
    call DemoFunctionCalls

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoMathFunctions(); ---
    ; Function call: DemoMathFunctions
    call DemoMathFunctions

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoStrings(); ---
    ; Function call: DemoStrings
    call DemoStrings

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- DemoBreakContinue(); ---
    ; Function call: DemoBreakContinue
    call DemoBreakContinue

    ; --- print(""); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("============================================="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("All demonstrations complete!"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("============================================="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

