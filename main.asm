; ============================
; Flux# compiled from "main.fsh"
; fluxc v0.1.0
; ============================

section .data
str_0: db "========================================", 0
str_1: db "----------------------------------------", 0
str_2: db "10 + 5 = ", 0
str_3: db "20 - 8 = ", 0
str_4: db "7 * 6 = ", 0
str_5: db "20 / 4 = ", 0
str_6: db "Counting from 0 to 4:", 0
str_7: db "0", 0
str_8: db "1", 0
str_9: db "2", 0
str_10: db "3", 0
str_11: db "4", 0
str_12: db "PI constant:", 0
double_13: dq 0x400921fb54442d18
double_14: db "3.141592653589793", 0
str_15: db "E constant:", 0
double_16: dq 0x4005bf0a8b145769
double_17: db "2.718281828459045", 0
str_18: db "sqrt(16):", 0
double_19: dq 0x4010000000000000
double_20: db "4", 0
str_21: db "Power function 2^3:", 0
double_22: dq 0x4020000000000000
double_23: db "8", 0
str_24: db "FluxSharp Language Demo", 0
str_25: db "", 0
str_26: db "Demo 1: Arithmetic Operations with Classes", 0
str_27: db "", 0
str_28: db "Demo 2: Loop Control Flow", 0
str_29: db "", 0
str_30: db "Demo 3: Math Constants", 0
str_31: db "", 0
str_32: db "Demo 3b: Math Functions", 0
str_33: db "", 0
str_34: db "Program Complete!", 0
str_35: db "FluxSharp v1.0 - Ready for Production", 0

section .text
; === Compiled from "main.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int


; --- // ============================================================ ---
global print_separator
print_separator:
    push rbp
    mov rbp, rsp

    ; --- serial_print("========================================"); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- } ---
global print_line
print_line:
    push rbp
    mov rbp, rsp

    ; --- serial_print("----------------------------------------"); ---
    lea rdi, [rel str_1]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global MathDemo_show_addition
MathDemo_show_addition:
    push rbp
    mov rbp, rsp

    ; --- int a = 10; ---
    sub rsp, 8
    mov qword [rbp-8], 10

    ; --- int b = 5; ---
    sub rsp, 8
    mov qword [rbp-16], 5

    ; --- int sum = a + b; ---
    sub rsp, 8
    mov qword [rbp-24], 15

    ; --- serial_print("10 + 5 = "); ---
    lea rdi, [rel str_2]
    call _fsh_print_str

    ; --- serial_print(sum); ---
    mov rdi, 15
    call _fsh_print_int

    mov rsp, rbp
    pop rbp
    ret

global MathDemo_show_subtraction
MathDemo_show_subtraction:
    push rbp
    mov rbp, rsp

    ; --- int a = 20; ---
    sub rsp, 8
    mov qword [rbp-8], 20

    ; --- int b = 8; ---
    sub rsp, 8
    mov qword [rbp-16], 8

    ; --- int diff = a - b; ---
    sub rsp, 8
    mov qword [rbp-24], 12

    ; --- serial_print("20 - 8 = "); ---
    lea rdi, [rel str_3]
    call _fsh_print_str

    ; --- serial_print(diff); ---
    mov rdi, 12
    call _fsh_print_int

    mov rsp, rbp
    pop rbp
    ret

global MathDemo_show_multiplication
MathDemo_show_multiplication:
    push rbp
    mov rbp, rsp

    ; --- int a = 7; ---
    sub rsp, 8
    mov qword [rbp-8], 7

    ; --- int b = 6; ---
    sub rsp, 8
    mov qword [rbp-16], 6

    ; --- int prod = a * b; ---
    sub rsp, 8
    mov qword [rbp-24], 42

    ; --- serial_print("7 * 6 = "); ---
    lea rdi, [rel str_4]
    call _fsh_print_str

    ; --- serial_print(prod); ---
    mov rdi, 42
    call _fsh_print_int

    mov rsp, rbp
    pop rbp
    ret

global MathDemo_show_division
MathDemo_show_division:
    push rbp
    mov rbp, rsp

    ; --- int a = 20; ---
    sub rsp, 8
    mov qword [rbp-8], 20

    ; --- int b = 4; ---
    sub rsp, 8
    mov qword [rbp-16], 4

    ; --- int quot = a / b; ---
    sub rsp, 8
    mov qword [rbp-24], 5

    ; --- serial_print("20 / 4 = "); ---
    lea rdi, [rel str_5]
    call _fsh_print_str

    ; --- serial_print(quot); ---
    mov rdi, 5
    call _fsh_print_int

    mov rsp, rbp
    pop rbp
    ret


; --- // ============================================================ ---
global show_counting
show_counting:
    push rbp
    mov rbp, rsp

    ; --- serial_print("Counting from 0 to 4:"); ---
    lea rdi, [rel str_6]
    call _fsh_print_str

    ; --- serial_print("0"); ---
    lea rdi, [rel str_7]
    call _fsh_print_str

    ; --- serial_print("1"); ---
    lea rdi, [rel str_8]
    call _fsh_print_str

    ; --- serial_print("2"); ---
    lea rdi, [rel str_9]
    call _fsh_print_str

    ; --- serial_print("3"); ---
    lea rdi, [rel str_10]
    call _fsh_print_str

    ; --- serial_print("4"); ---
    lea rdi, [rel str_11]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ============================================================ ---
global show_math_constants
show_math_constants:
    push rbp
    mov rbp, rsp

    ; --- serial_print("PI constant:"); ---
    lea rdi, [rel str_12]
    call _fsh_print_str

    ; --- double pi = PI; ---
    sub rsp, 8
    mov rax, [rel double_13]
    mov qword [rbp-8], rax

    ; --- serial_print(pi); ---
    lea rdi, [rel double_14]
    call _fsh_print_str

    ; --- serial_print("E constant:"); ---
    lea rdi, [rel str_15]
    call _fsh_print_str

    ; --- double e = E; ---
    sub rsp, 8
    mov rax, [rel double_16]
    mov qword [rbp-16], rax

    ; --- serial_print(e); ---
    lea rdi, [rel double_17]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- } ---
global show_sqrt_function
show_sqrt_function:
    push rbp
    mov rbp, rsp

    ; --- serial_print("sqrt(16):"); ---
    lea rdi, [rel str_18]
    call _fsh_print_str

    ; --- double sqrt_result = sqrt(16); ---
    sub rsp, 8
    mov rax, [rel double_19]
    mov qword [rbp-8], rax

    ; --- serial_print(sqrt_result); ---
    lea rdi, [rel double_20]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- } ---
global show_pow_function
show_pow_function:
    push rbp
    mov rbp, rsp

    ; --- serial_print("Power function 2^3:"); ---
    lea rdi, [rel str_21]
    call _fsh_print_str

    ; --- double power_result = pow(2, 3); ---
    sub rsp, 8
    mov rax, [rel double_22]
    mov qword [rbp-8], rax

    ; --- serial_print(power_result); ---
    lea rdi, [rel double_23]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- // ============================================================ ---
global main
main:
    push rbp
    mov rbp, rsp

    ; --- print_separator(); ---
    ; Function call: print_separator
    call print_separator

    ; --- serial_print("FluxSharp Language Demo"); ---
    lea rdi, [rel str_24]
    call _fsh_print_str

    ; --- print_separator(); ---
    ; Function call: print_separator
    call print_separator

    ; --- serial_print(""); ---
    lea rdi, [rel str_25]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- serial_print("Demo 1: Arithmetic Operations with Classes"); ---
    lea rdi, [rel str_26]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- MathDemo demo; ---
    sub rsp, 8

    ; --- demo.show_addition(); ---
    call MathDemo_show_addition

    ; --- demo.show_subtraction(); ---
    call MathDemo_show_subtraction

    ; --- demo.show_multiplication(); ---
    call MathDemo_show_multiplication

    ; --- demo.show_division(); ---
    call MathDemo_show_division

    ; --- serial_print(""); ---
    lea rdi, [rel str_27]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- serial_print("Demo 2: Loop Control Flow"); ---
    lea rdi, [rel str_28]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- show_counting(); ---
    ; Function call: show_counting
    call show_counting

    ; --- serial_print(""); ---
    lea rdi, [rel str_29]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- serial_print("Demo 3: Math Constants"); ---
    lea rdi, [rel str_30]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- show_math_constants(); ---
    ; Function call: show_math_constants
    call show_math_constants

    ; --- serial_print(""); ---
    lea rdi, [rel str_31]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- serial_print("Demo 3b: Math Functions"); ---
    lea rdi, [rel str_32]
    call _fsh_print_str

    ; --- print_line(); ---
    ; Function call: print_line
    call print_line

    ; --- show_sqrt_function(); ---
    ; Function call: show_sqrt_function
    call show_sqrt_function

    ; --- show_pow_function(); ---
    ; Function call: show_pow_function
    call show_pow_function

    ; --- serial_print(""); ---
    lea rdi, [rel str_33]
    call _fsh_print_str

    ; --- print_separator(); ---
    ; Function call: print_separator
    call print_separator

    ; --- serial_print("Program Complete!"); ---
    lea rdi, [rel str_34]
    call _fsh_print_str

    ; --- serial_print("FluxSharp v1.0 - Ready for Production"); ---
    lea rdi, [rel str_35]
    call _fsh_print_str

    ; --- print_separator(); ---
    ; Function call: print_separator
    call print_separator

    mov rsp, rbp
    pop rbp
    ret

