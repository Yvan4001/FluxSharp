; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh"
; fluxc v0.1.0
; ============================

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
str_23: db "Functions done", 0
str_24: db "=== Math Functions ===", 0
str_25: db "Math done", 0
str_26: db "=== Strings ===", 0
str_27: db "FluxSharp", 0
str_28: db "Hello World", 0
str_29: db "Hello World", 0
str_30: db "=== Break/Continue ===", 0
str_34: db "Loop", 0
str_37: db "=============================================", 0
str_38: db "FluxSharp - Complete Language Demonstration", 0
str_39: db "=============================================", 0
str_40: db "", 0
str_41: db "", 0
str_42: db "", 0
str_43: db "", 0
str_44: db "", 0
str_45: db "", 0
str_46: db "", 0
str_47: db "", 0
str_48: db "", 0
str_49: db "", 0
str_50: db "", 0
str_51: db "", 0
str_52: db "=============================================", 0
str_53: db "All demonstrations complete!", 0
str_54: db "=============================================", 0

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
    lea rdi, [rel str_5]
    call _fsh_print_str

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
    ; ERROR evaluating expr: Unknown operator: &

    ; --- int bitwiseOr = 5 | 3;    // OR ---
    sub rsp, 8
    ; ERROR evaluating expr: Unknown operator: |

    ; --- int bitwiseXor = 5 ^ 3;   // XOR ---
    sub rsp, 8
    ; ERROR evaluating expr: Unknown operator: ^

    ; --- int leftShift = 5 << 1;   // Left shift ---
    sub rsp, 8
    ; ERROR evaluating expr: Unknown operator: <<

    ; --- int rightShift = 10 >> 1; // Right shift ---
    sub rsp, 8
    ; ERROR evaluating expr: Unknown operator: >>

    ; --- print("Operators done"); ---
    lea rdi, [rel str_7]
    call _fsh_print_str

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
    ; ERROR evaluating expr: Undefined variable: 'numbers'

    ; --- int second = numbers[1]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'numbers'

    ; --- int third = numbers[2]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'numbers'

    ; --- print("Arrays initialized"); ---
    lea rdi, [rel str_9]
    call _fsh_print_str

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
    inc qword [rbp-8]
    jmp .for_start_15
.for_end_15:

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
    lea rdi, [rel str_20]
    call _fsh_print_str

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- calc.value = 42; ---
    ; Property assignment: calc.value

    ; --- int sum = calc.Add(5, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'calc'

    ; --- int product = calc.Multiply(4, 7); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'calc'

    ; --- print("Calculator done"); ---
    lea rdi, [rel str_21]
    call _fsh_print_str

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
    lea rdi, [rel str_22]
    call _fsh_print_str

    ; --- int result = Add(10, 20); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'Add'

    ; --- string combined = Concatenate("Hello", "World"); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'Concatenate'

    ; --- PrintValue(42); ---
    ; Function call: PrintValue
    call PrintValue

    ; --- print("Functions done"); ---
    lea rdi, [rel str_23]
    call _fsh_print_str

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
    lea rdi, [rel str_24]
    call _fsh_print_str

    ; --- int abs_val = abs(0 - 42); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'abs'

    ; --- int max_val = max(10, 5); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'max'

    ; --- int min_val = min(10, 5); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'min'

    ; --- int power_val = pow(2, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'pow'

    ; --- print("Math done"); ---
    lea rdi, [rel str_25]
    call _fsh_print_str

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
    lea rdi, [rel str_26]
    call _fsh_print_str

    ; --- string text = "FluxSharp"; ---
    sub rsp, 8
    lea rax, [rel str_27]
    mov [rbp-8], rax

    ; --- string greeting = "Hello" + " " + "World"; ---
    sub rsp, 8
    lea rax, [rel str_28]
    mov [rbp-16], rax

    ; --- print(greeting); ---
    lea rdi, [rel str_29]
    call _fsh_print_str

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
    lea rdi, [rel str_30]
    call _fsh_print_str

    ; --- for (int i = 0; i < 10; i++) { ---
    sub rsp, 8
    mov rax, 0
    mov qword [rbp-8], rax
.for_start_31:
    mov rax, [rbp-8]
    cmp rax, 10
    jge .for_end_31

    ; --- if (i == 3) { ---
    mov rax, [rbp-8]
    cmp rax, 3
    jne .if_false_32

    ; --- continue; ---
    ; continue (not yet fully implemented)
    jmp .if_end_32
.if_false_32:
.if_end_32:

    ; --- if (i == 7) { ---
    mov rax, [rbp-8]
    cmp rax, 7
    jne .if_false_33

    ; --- break; ---
    ; break (not yet fully implemented)
    jmp .if_end_33
.if_false_33:
.if_end_33:

    ; --- print("Loop"); ---
    lea rdi, [rel str_34]
    call _fsh_print_str
    inc qword [rbp-8]
    jmp .for_start_31
.for_end_31:

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
    ; condition: n < 0
    test rax, rax
    jz .if_false_35

    ; --- return 0 - 1; ---
    jmp .if_end_35
.if_false_35:
.if_end_35:

    ; --- if (n > MAX_SIZE) { ---
    ; condition: n > MAX_SIZE
    test rax, rax
    jz .if_false_36

    ; --- return 0 - 1; ---
    jmp .if_end_36
.if_false_36:
.if_end_36:

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
    lea rdi, [rel str_37]
    call _fsh_print_str

    ; --- print("FluxSharp - Complete Language Demonstration"); ---
    lea rdi, [rel str_38]
    call _fsh_print_str

    ; --- print("============================================="); ---
    lea rdi, [rel str_39]
    call _fsh_print_str

    ; --- print(""); ---
    lea rdi, [rel str_40]
    call _fsh_print_str

    ; --- DemoTypes(); ---
    ; Function call: DemoTypes
    call DemoTypes

    ; --- print(""); ---
    lea rdi, [rel str_41]
    call _fsh_print_str

    ; --- DemoOperators(); ---
    ; Function call: DemoOperators
    call DemoOperators

    ; --- print(""); ---
    lea rdi, [rel str_42]
    call _fsh_print_str

    ; --- DemoArrays(); ---
    ; Function call: DemoArrays
    call DemoArrays

    ; --- print(""); ---
    lea rdi, [rel str_43]
    call _fsh_print_str

    ; --- DemoIfElse(); ---
    ; Function call: DemoIfElse
    call DemoIfElse

    ; --- print(""); ---
    lea rdi, [rel str_44]
    call _fsh_print_str

    ; --- DemoForLoop(); ---
    ; Function call: DemoForLoop
    call DemoForLoop

    ; --- print(""); ---
    lea rdi, [rel str_45]
    call _fsh_print_str

    ; --- DemoWhileLoop(); ---
    ; Function call: DemoWhileLoop
    call DemoWhileLoop

    ; --- print(""); ---
    lea rdi, [rel str_46]
    call _fsh_print_str

    ; --- DemoClasses(); ---
    ; Function call: DemoClasses
    call DemoClasses

    ; --- print(""); ---
    lea rdi, [rel str_47]
    call _fsh_print_str

    ; --- DemoFunctionCalls(); ---
    ; Function call: DemoFunctionCalls
    call DemoFunctionCalls

    ; --- print(""); ---
    lea rdi, [rel str_48]
    call _fsh_print_str

    ; --- DemoMathFunctions(); ---
    ; Function call: DemoMathFunctions
    call DemoMathFunctions

    ; --- print(""); ---
    lea rdi, [rel str_49]
    call _fsh_print_str

    ; --- DemoStrings(); ---
    ; Function call: DemoStrings
    call DemoStrings

    ; --- print(""); ---
    lea rdi, [rel str_50]
    call _fsh_print_str

    ; --- DemoBreakContinue(); ---
    ; Function call: DemoBreakContinue
    call DemoBreakContinue

    ; --- print(""); ---
    lea rdi, [rel str_51]
    call _fsh_print_str

    ; --- print("============================================="); ---
    lea rdi, [rel str_52]
    call _fsh_print_str

    ; --- print("All demonstrations complete!"); ---
    lea rdi, [rel str_53]
    call _fsh_print_str

    ; --- print("============================================="); ---
    lea rdi, [rel str_54]
    call _fsh_print_str

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

