; ============================
; Flux# compiled from "main.fsh"
; fluxc v0.1.0
; ============================

extern _fsh_print_str
extern _fsh_print_int
extern _fsh_abs
extern _fsh_max
extern _fsh_min
extern _fsh_pow
extern _fsh_floor
extern _fsh_ceil
extern _fsh_sqrt

section .data
str_0: db "=== Types and Variables ===", 0
float_1: dd 0x4048f5c3
double_2: dq 0x400921f9f01b866e
str_3: db "FluxSharp", 0
str_4: db "Types initialized", 0
str_5: db "Value", 0
str_6: db "=== Operators ===", 0
str_7: db "Operators done", 0
str_8: db "=== Arrays ===", 0
str_9: db "Arrays initialized", 0
str_10: db "=== If/Else ===", 0
str_12: db "Greater", 0
str_13: db "Lesser", 0
str_14: db "=== For Loop ===", 0
str_16: db "Iteration", 0
str_17: db "=== While Loop ===", 0
str_19: db "Loop", 0
str_20: db "=== Classes ===", 0
str_21: db "Calculator done", 0
str_22: db "=== Function Calls ===", 0
str_23: db "Hello", 0
str_24: db "World", 0
str_25: db "Functions done", 0
str_26: db "=== Math Functions ===", 0
str_27: db "Math done", 0
str_28: db "=== Strings ===", 0
str_29: db "FluxSharp", 0
str_30: db "Hello World", 0
str_31: db "Hello World", 0
str_32: db "=== Break/Continue ===", 0
str_36: db "Loop", 0
str_39: db "=============================================", 0
str_40: db "FluxSharp - Complete Language Demonstration", 0
str_41: db "=============================================", 0
str_42: db "=============================================", 0
str_43: db "All demonstrations complete!", 0
str_44: db "=============================================", 0

section .text
; === Compiled from "main.fsh" by fluxc ===
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
    lea rdi, [rel str_0]
    call _fsh_print_str

    ; --- int intVal = 42; ---
    sub rsp, 8
    mov qword [rbp-8], 42

    ; --- uint uintVal = 100; ---
    sub rsp, 8
    mov qword [rbp-16], 100

    ; --- long longVal = 9999999; ---
    sub rsp, 8
    mov qword [rbp-24], 9999999

    ; --- float floatVal = 3.14f; ---
    sub rsp, 8
    mov eax, [rel float_1]
    mov dword [rbp-32], eax

    ; --- double doubleVal = 3.14159; ---
    sub rsp, 8
    mov rax, [rel double_2]
    mov qword [rbp-40], rax

    ; --- byte byteVal = 255; ---
    sub rsp, 8
    mov qword [rbp-48], 255

    ; --- string stringVal = "FluxSharp"; ---
    sub rsp, 8
    lea rax, [rel str_3]
    mov [rbp-56], rax

    ; --- bool boolVal = true; ---
    sub rsp, 8
    mov qword [rbp-64], 1

    ; --- print("Types initialized"); ---
    lea rdi, [rel str_4]
    call _fsh_print_str

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
    lea rdi, [rel str_5]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoOperators
DemoOperators:
    push rbp
    mov rbp, rsp

    ; --- print("=== Operators ==="); ---
    lea rdi, [rel str_6]
    call _fsh_print_str

    ; --- int sum = 5 + 3;          // Addition ---
    sub rsp, 8
    mov qword [rbp-8], 8

    ; --- int diff = 10 - 3;        // Subtraction ---
    sub rsp, 8
    mov qword [rbp-16], 7

    ; --- int prod = 4 * 5;         // Multiplication ---
    sub rsp, 8
    mov qword [rbp-24], 20

    ; --- int quot = 20 / 4;        // Division ---
    sub rsp, 8
    mov qword [rbp-32], 5

    ; --- int rem = 17 % 5;         // Modulo ---
    sub rsp, 8
    mov qword [rbp-40], 2

    ; --- int bitwiseAnd = 5 & 3;   // AND ---
    sub rsp, 8
    mov qword [rbp-48], 1

    ; --- int bitwiseOr = 5 | 3;    // OR ---
    sub rsp, 8
    mov qword [rbp-56], 7

    ; --- int bitwiseXor = 5 ^ 3;   // XOR ---
    sub rsp, 8
    mov qword [rbp-64], 6

    ; --- int leftShift = 5 << 1;   // Left shift ---
    sub rsp, 8
    mov qword [rbp-72], 10

    ; --- int rightShift = 10 >> 1; // Right shift ---
    sub rsp, 8
    mov qword [rbp-80], 5

    ; --- print("Operators done"); ---
    lea rdi, [rel str_7]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoArrays
DemoArrays:
    push rbp
    mov rbp, rsp

    ; --- print("=== Arrays ==="); ---
    lea rdi, [rel str_8]
    call _fsh_print_str

    ; --- int[10] numbers; ---

    ; --- numbers[0] = 10; ---

    ; --- numbers[1] = 20; ---

    ; --- numbers[2] = 30; ---

    ; --- numbers[3] = 40; ---

    ; --- numbers[4] = 50; ---

    ; --- int first = numbers[0]; ---
    sub rsp, 8
    mov qword [rbp-8], 0 ; Array access stub: numbers[0]

    ; --- int second = numbers[1]; ---
    sub rsp, 8
    mov qword [rbp-16], 0 ; Array access stub: numbers[1]

    ; --- int third = numbers[2]; ---
    sub rsp, 8
    mov qword [rbp-24], 0 ; Array access stub: numbers[2]

    ; --- print("Arrays initialized"); ---
    lea rdi, [rel str_9]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoIfElse
DemoIfElse:
    push rbp
    mov rbp, rsp

    ; --- print("=== If/Else ==="); ---
    lea rdi, [rel str_10]
    call _fsh_print_str

    ; --- int x = 15; ---
    sub rsp, 8
    mov qword [rbp-8], 15

    ; --- if (x > 10) { ---
    mov rax, [rbp-8]
    cmp rax, 10
    jle .if_false_11

    ; --- print("Greater"); ---
    lea rdi, [rel str_12]
    call _fsh_print_str
    jmp .if_end_11
.if_false_11:

    ; --- print("Lesser"); ---
    lea rdi, [rel str_13]
    call _fsh_print_str
.if_end_11:

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoForLoop
DemoForLoop:
    push rbp
    mov rbp, rsp

    ; --- print("=== For Loop ==="); ---
    lea rdi, [rel str_14]
    call _fsh_print_str

    ; --- for (int i = 0; i < 5; i++) { ---
    sub rsp, 8
    mov rax, 0
    mov qword [rbp-8], rax
.for_start_15:
    mov rax, [rbp-8]
    cmp rax, 5
    jge .for_end_15

    ; --- print("Iteration"); ---
    lea rdi, [rel str_16]
    call _fsh_print_str
.for_continue_15:
    inc qword [rbp-8]
    jmp .for_start_15
.for_end_15:

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoWhileLoop
DemoWhileLoop:
    push rbp
    mov rbp, rsp

    ; --- print("=== While Loop ==="); ---
    lea rdi, [rel str_17]
    call _fsh_print_str

    ; --- int counter = 0; ---
    sub rsp, 8
    mov qword [rbp-8], 0

    ; --- while (counter < 3) { ---
.while_start_18:
    mov rax, [rbp-8]
    cmp rax, 3
    jge .while_end_18

    ; --- print("Loop"); ---
    lea rdi, [rel str_19]
    call _fsh_print_str

    ; --- counter++; ---
    inc qword [rbp-8]
    jmp .while_start_18
.while_end_18:

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoClasses
DemoClasses:
    push rbp
    mov rbp, rsp

    ; --- print("=== Classes ==="); ---
    lea rdi, [rel str_20]
    call _fsh_print_str

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- calc.value = 42; ---
    ; Property assignment: calc.value

    ; --- int sum = calc.Add(5, 3); ---
    sub rsp, 8
    mov qword [rbp-16], 0 ; Method call stub: calc.Add

    ; --- int product = calc.Multiply(4, 7); ---
    sub rsp, 8
    mov qword [rbp-24], 0 ; Method call stub: calc.Multiply

    ; --- print("Calculator done"); ---
    lea rdi, [rel str_21]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoFunctionCalls
DemoFunctionCalls:
    push rbp
    mov rbp, rsp

    ; --- print("=== Function Calls ==="); ---
    lea rdi, [rel str_22]
    call _fsh_print_str

    ; --- int result = Add(10, 20); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 20
    call Add
    mov qword [rbp-8], rax

    ; --- string combined = Concatenate("Hello", "World"); ---
    sub rsp, 8
    lea rdi, [rel str_23]
    lea rsi, [rel str_24]
    call Concatenate
    mov qword [rbp-16], rax

    ; --- PrintValue(42); ---
    ; Function call: PrintValue
    call PrintValue

    ; --- print("Functions done"); ---
    lea rdi, [rel str_25]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoMathFunctions
DemoMathFunctions:
    push rbp
    mov rbp, rsp

    ; --- print("=== Math Functions ==="); ---
    lea rdi, [rel str_26]
    call _fsh_print_str

    ; --- int abs_val = abs(0 - 42); ---
    sub rsp, 8
    mov rdi, -42
    call _fsh_abs
    mov qword [rbp-8], rax

    ; --- int max_val = max(10, 5); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 5
    call _fsh_max
    mov qword [rbp-16], rax

    ; --- int min_val = min(10, 5); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 5
    call _fsh_min
    mov qword [rbp-24], rax

    ; --- int power_val = pow(2, 3); ---
    sub rsp, 8
    mov rdi, 2
    mov rsi, 3
    call _fsh_pow
    mov qword [rbp-32], rax

    ; --- print("Math done"); ---
    lea rdi, [rel str_27]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoStrings
DemoStrings:
    push rbp
    mov rbp, rsp

    ; --- print("=== Strings ==="); ---
    lea rdi, [rel str_28]
    call _fsh_print_str

    ; --- string text = "FluxSharp"; ---
    sub rsp, 8
    lea rax, [rel str_29]
    mov [rbp-8], rax

    ; --- string greeting = "Hello" + " " + "World"; ---
    sub rsp, 8
    lea rax, [rel str_30]
    mov [rbp-16], rax

    ; --- print(greeting); ---
    lea rdi, [rel str_31]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global DemoBreakContinue
DemoBreakContinue:
    push rbp
    mov rbp, rsp

    ; --- print("=== Break/Continue ==="); ---
    lea rdi, [rel str_32]
    call _fsh_print_str

    ; --- for (int i = 0; i < 10; i++) { ---
    sub rsp, 8
    mov rax, 0
    mov qword [rbp-8], rax
.for_start_33:
    mov rax, [rbp-8]
    cmp rax, 10
    jge .for_end_33

    ; --- if (i == 3) { ---
    mov rax, [rbp-8]
    cmp rax, 3
    jne .if_false_34

    ; --- continue; ---
    jmp .for_continue_33
    jmp .if_end_34
.if_false_34:
.if_end_34:

    ; --- if (i == 7) { ---
    mov rax, [rbp-8]
    cmp rax, 7
    jne .if_false_35

    ; --- break; ---
    jmp .for_end_33
    jmp .if_end_35
.if_false_35:
.if_end_35:

    ; --- print("Loop"); ---
    lea rdi, [rel str_36]
    call _fsh_print_str
.for_continue_33:
    inc qword [rbp-8]
    jmp .for_start_33
.for_end_33:

    mov rsp, rbp
    pop rbp
    ret


; --- // ======================================================== ---
global ValidateNumber
ValidateNumber:
    push rbp
    mov rbp, rsp

    ; --- if (n < 0) { ---
    ; condition: n < 0
    test rax, rax
    jz .if_false_37

    ; --- return 0 - 1; ---
    jmp .if_end_37
.if_false_37:
.if_end_37:

    ; --- if (n > MAX_SIZE) { ---
    ; condition: n > MAX_SIZE
    test rax, rax
    jz .if_false_38

    ; --- return 0 - 1; ---
    jmp .if_end_38
.if_false_38:
.if_end_38:

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
    lea rdi, [rel str_39]
    call _fsh_print_str

    ; --- print("FluxSharp - Complete Language Demonstration"); ---
    lea rdi, [rel str_40]
    call _fsh_print_str

    ; --- print("============================================="); ---
    lea rdi, [rel str_41]
    call _fsh_print_str

    ; --- DemoTypes(); ---
    ; Function call: DemoTypes
    call DemoTypes

    ; --- DemoOperators(); ---
    ; Function call: DemoOperators
    call DemoOperators

    ; --- DemoArrays(); ---
    ; Function call: DemoArrays
    call DemoArrays

    ; --- DemoIfElse(); ---
    ; Function call: DemoIfElse
    call DemoIfElse

    ; --- DemoForLoop(); ---
    ; Function call: DemoForLoop
    call DemoForLoop

    ; --- DemoWhileLoop(); ---
    ; Function call: DemoWhileLoop
    call DemoWhileLoop

    ; --- DemoClasses(); ---
    ; Function call: DemoClasses
    call DemoClasses

    ; --- DemoFunctionCalls(); ---
    ; Function call: DemoFunctionCalls
    call DemoFunctionCalls

    ; --- DemoMathFunctions(); ---
    ; Function call: DemoMathFunctions
    call DemoMathFunctions

    ; --- DemoStrings(); ---
    ; Function call: DemoStrings
    call DemoStrings

    ; --- DemoBreakContinue(); ---
    ; Function call: DemoBreakContinue
    call DemoBreakContinue

    ; --- print("============================================="); ---
    lea rdi, [rel str_42]
    call _fsh_print_str

    ; --- print("All demonstrations complete!"); ---
    lea rdi, [rel str_43]
    call _fsh_print_str

    ; --- print("============================================="); ---
    lea rdi, [rel str_44]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

