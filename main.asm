; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh"
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
extern _fsh_round
extern _fsh_sqrt

section .data
str_0: db "Calculator", 0
str_1: db "=== Test 1: Helper Methods ===", 0
str_2: db "GetDouble(21) = ", 0
str_3: db "GetTriple(7) = ", 0
str_4: db "=== Test 2: Calculator Methods ===", 0
str_5: db "Add(10, 20) = ", 0
str_6: db "Subtract(30, 15) = ", 0
str_7: db "Multiply(5, 6) = ", 0
str_8: db "=== Test 3: ToString() for Primitive Types ===", 0
float_9: dd 0x4048f5c3
double_10: dq 0x4005bf0995aaf790
str_11: db "Hello", 0
str_12: db "int value: ", 0
str_13: db "float value: ", 0
float_14: db "3.14", 0
str_15: db "double value: ", 0
double_16: db "2.71828", 0
str_17: db "string value: ", 0
str_18: db "Hello", 0
str_19: db "=== Test 4: Math Functions ===", 0
str_20: db "pow(2, 3) = ", 0
str_21: db "max(10, 20) = ", 0
str_22: db "min(10, 20) = ", 0
str_23: db "=== Test 5: Variables ===", 0
str_24: db "Variable x = ", 0
str_25: db "Variable y = ", 0
str_26: db "=== Test 6: String Operations ===", 0
str_27: db "Hello, FluxSharp!", 0
str_28: db "Program completed successfully!", 0

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global MathHelper_GetDouble
MathHelper_GetDouble:
    push rbp
    mov rbp, rsp

    ; --- return x * 2; ---
    mov rax, rdi
    imul rax, 2
    mov rsp, rbp
    pop rbp
    ret
global MathHelper_GetTriple
MathHelper_GetTriple:
    push rbp
    mov rbp, rsp

    ; --- return x * 3; ---
    mov rax, rdi
    imul rax, 3
    mov rsp, rbp
    pop rbp
    ret
global Calculator_Add
Calculator_Add:
    push rbp
    mov rbp, rsp

    ; --- return a + b; ---
    mov rax, rdi
    mov rcx, rsi
    add rax, rcx
    mov rsp, rbp
    pop rbp
    ret
global Calculator_Subtract
Calculator_Subtract:
    push rbp
    mov rbp, rsp

    ; --- return a - b; ---
    mov rax, rdi
    mov rcx, rsi
    sub rax, rcx
    mov rsp, rbp
    pop rbp
    ret
global Calculator_Multiply
Calculator_Multiply:
    push rbp
    mov rbp, rsp

    ; --- return a * b; ---
    mov rax, rdi
    mov rcx, rsi
    imul rax, rcx
    mov rsp, rbp
    pop rbp
    ret
global Calculator_ToString
Calculator_ToString:
    push rbp
    mov rbp, rsp

    ; --- return "Calculator"; ---
    lea rax, [rel str_0]
    mov rsp, rbp
    pop rbp
    ret

    mov rsp, rbp
    pop rbp
    ret

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- print("=== Test 1: Helper Methods ==="); ---
    lea rdi, [rel str_1]
    call _fsh_print_str

    ; --- MathHelper helper = new MathHelper(); ---
    sub rsp, 8

    ; --- int double21 = helper.GetDouble(21); ---
    sub rsp, 8
    mov rdi, 21
    call MathHelper_GetDouble
    mov qword [rbp-16], rax

    ; --- print("GetDouble(21) = "); ---
    lea rdi, [rel str_2]
    call _fsh_print_str

    ; --- print(double21); ---
    mov rdi, [rbp-16]
    call _fsh_print_int

    ; --- int triple7 = helper.GetTriple(7); ---
    sub rsp, 8
    mov rdi, 7
    call MathHelper_GetTriple
    mov qword [rbp-24], rax

    ; --- print("GetTriple(7) = "); ---
    lea rdi, [rel str_3]
    call _fsh_print_str

    ; --- print(triple7); ---
    mov rdi, [rbp-24]
    call _fsh_print_int

    ; --- print("=== Test 2: Calculator Methods ==="); ---
    lea rdi, [rel str_4]
    call _fsh_print_str

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- int sum = calc.Add(10, 20); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 20
    call Calculator_Add
    mov qword [rbp-40], rax

    ; --- print("Add(10, 20) = "); ---
    lea rdi, [rel str_5]
    call _fsh_print_str

    ; --- print(sum); ---
    mov rdi, [rbp-40]
    call _fsh_print_int

    ; --- int diff = calc.Subtract(30, 15); ---
    sub rsp, 8
    mov rdi, 30
    mov rsi, 15
    call Calculator_Subtract
    mov qword [rbp-48], rax

    ; --- print("Subtract(30, 15) = "); ---
    lea rdi, [rel str_6]
    call _fsh_print_str

    ; --- print(diff); ---
    mov rdi, [rbp-48]
    call _fsh_print_int

    ; --- int prod = calc.Multiply(5, 6); ---
    sub rsp, 8
    mov rdi, 5
    mov rsi, 6
    call Calculator_Multiply
    mov qword [rbp-56], rax

    ; --- print("Multiply(5, 6) = "); ---
    lea rdi, [rel str_7]
    call _fsh_print_str

    ; --- print(prod); ---
    mov rdi, [rbp-56]
    call _fsh_print_int

    ; --- print("=== Test 3: ToString() for Primitive Types ==="); ---
    lea rdi, [rel str_8]
    call _fsh_print_str

    ; --- int intValue = 42; ---
    sub rsp, 8
    mov qword [rbp-64], 42

    ; --- float floatValue = 3.14f; ---
    sub rsp, 8
    mov eax, [rel float_9]
    mov dword [rbp-72], eax

    ; --- double doubleValue = 2.71828; ---
    sub rsp, 8
    mov rax, [rel double_10]
    mov qword [rbp-80], rax

    ; --- string strValue = "Hello"; ---
    sub rsp, 8
    lea rax, [rel str_11]
    mov [rbp-88], rax

    ; --- print("int value: "); ---
    lea rdi, [rel str_12]
    call _fsh_print_str

    ; --- print(intValue.ToString()); ---
    mov rdi, 42
    call _fsh_print_int

    ; --- print("float value: "); ---
    lea rdi, [rel str_13]
    call _fsh_print_str

    ; --- print(floatValue.ToString()); ---
    lea rdi, [rel float_14]
    call _fsh_print_str

    ; --- print("double value: "); ---
    lea rdi, [rel str_15]
    call _fsh_print_str

    ; --- print(doubleValue.ToString()); ---
    lea rdi, [rel double_16]
    call _fsh_print_str

    ; --- print("string value: "); ---
    lea rdi, [rel str_17]
    call _fsh_print_str

    ; --- print(strValue.ToString()); ---
    lea rdi, [rel str_18]
    call _fsh_print_str

    ; --- print("=== Test 4: Math Functions ==="); ---
    lea rdi, [rel str_19]
    call _fsh_print_str

    ; --- int powResult = pow(2, 3); ---
    sub rsp, 8
    mov rdi, 2
    mov rsi, 3
    call _fsh_pow
    mov qword [rbp-96], rax

    ; --- print("pow(2, 3) = "); ---
    lea rdi, [rel str_20]
    call _fsh_print_str

    ; --- print(powResult); ---
    mov rdi, [rbp-96]
    call _fsh_print_int

    ; --- int maxResult = max(10, 20); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 20
    call _fsh_max
    mov qword [rbp-104], rax

    ; --- print("max(10, 20) = "); ---
    lea rdi, [rel str_21]
    call _fsh_print_str

    ; --- print(maxResult); ---
    mov rdi, [rbp-104]
    call _fsh_print_int

    ; --- int minResult = min(10, 20); ---
    sub rsp, 8
    mov rdi, 10
    mov rsi, 20
    call _fsh_min
    mov qword [rbp-112], rax

    ; --- print("min(10, 20) = "); ---
    lea rdi, [rel str_22]
    call _fsh_print_str

    ; --- print(minResult); ---
    mov rdi, [rbp-112]
    call _fsh_print_int

    ; --- print("=== Test 5: Variables ==="); ---
    lea rdi, [rel str_23]
    call _fsh_print_str

    ; --- int x = 42; ---
    sub rsp, 8
    mov qword [rbp-120], 42

    ; --- int y = 8; ---
    sub rsp, 8
    mov qword [rbp-128], 8

    ; --- print("Variable x = "); ---
    lea rdi, [rel str_24]
    call _fsh_print_str

    ; --- print(x); ---
    mov rdi, 42
    call _fsh_print_int

    ; --- print("Variable y = "); ---
    lea rdi, [rel str_25]
    call _fsh_print_str

    ; --- print(y); ---
    mov rdi, 8
    call _fsh_print_int

    ; --- print("=== Test 6: String Operations ==="); ---
    lea rdi, [rel str_26]
    call _fsh_print_str

    ; --- print("Hello, FluxSharp!"); ---
    lea rdi, [rel str_27]
    call _fsh_print_str

    ; --- print("Program completed successfully!"); ---
    lea rdi, [rel str_28]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main_main
    ret

