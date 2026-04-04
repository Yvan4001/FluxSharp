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
extern _fsh_sqrt

section .data
str_0: db "=== Test 1: Helper Methods ===", 0
str_1: db "GetDouble(21) = ", 0
str_2: db "GetTriple(7) = ", 0
str_3: db "=== Test 2: Calculator Methods ===", 0
str_4: db "Add(10, 20) = ", 0
str_5: db "Subtract(30, 15) = ", 0
str_6: db "Multiply(5, 6) = ", 0
str_7: db "=== Test 3: Variables ===", 0
str_8: db "Variable x = ", 0
str_9: db "Variable y = ", 0
str_10: db "=== Test 4: String Printing ===", 0
str_11: db "Hello, FluxSharp!", 0
str_12: db "Program completed successfully!", 0

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
global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- print("=== Test 1: Helper Methods ==="); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    ; --- MathHelper helper = new MathHelper(); ---
    sub rsp, 8

    ; --- int double21 = helper.GetDouble(21); ---
    sub rsp, 8
    mov rdi, 21
    call MathHelper_GetDouble
    mov qword [rbp-16], rax

    ; --- print("GetDouble(21) = "); ---
    lea rdi, [rel str_1]
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
    lea rdi, [rel str_2]
    call _fsh_print_str

    ; --- print(triple7); ---
    mov rdi, [rbp-24]
    call _fsh_print_int

    ; --- print("=== Test 2: Calculator Methods ==="); ---
    lea rdi, [rel str_3]
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
    lea rdi, [rel str_4]
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
    lea rdi, [rel str_5]
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
    lea rdi, [rel str_6]
    call _fsh_print_str

    ; --- print(prod); ---
    mov rdi, [rbp-56]
    call _fsh_print_int

    ; --- print("=== Test 3: Variables ==="); ---
    lea rdi, [rel str_7]
    call _fsh_print_str

    ; --- int x = 42; ---
    sub rsp, 8
    mov qword [rbp-64], 42

    ; --- int y = 8; ---
    sub rsp, 8
    mov qword [rbp-72], 8

    ; --- print("Variable x = "); ---
    lea rdi, [rel str_8]
    call _fsh_print_str

    ; --- print(x); ---
    mov rdi, 42
    call _fsh_print_int

    ; --- print("Variable y = "); ---
    lea rdi, [rel str_9]
    call _fsh_print_str

    ; --- print(y); ---
    mov rdi, 8
    call _fsh_print_int

    ; --- print("=== Test 4: String Printing ==="); ---
    lea rdi, [rel str_10]
    call _fsh_print_str

    ; --- print("Hello, FluxSharp!"); ---
    lea rdi, [rel str_11]
    call _fsh_print_str

    ; --- print("Program completed successfully!"); ---
    lea rdi, [rel str_12]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main_main
    ret

