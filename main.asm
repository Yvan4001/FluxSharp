; ============================
; Flux# compiled from "main.fsh"
; fluxc v0.1.0
; ============================

section .data
str_0: db "========================================", 0
str_1: db "FluxSharp Language Demo", 0
str_2: db "========================================", 0
str_3: db "", 0
str_4: db "----------------------------------------", 0
str_5: db "10 + 5 = ", 0
str_6: db "20 - 8 = ", 0
str_7: db "7 * 6 = ", 0
str_8: db "20 / 4 = ", 0
str_9: db "Counting 0 to 4:", 0
str_10: db "", 0
str_11: db "Math Constants and Functions:", 0
str_12: db "----------------------------------------", 0
double_13: dq 0x400921fb54442d18
str_14: db "PI = ", 0
double_15: db "3.141592653589793", 0
double_16: dq 0x4005bf0a8b145769
str_17: db "E = ", 0
double_18: db "2.718281828459045", 0
str_19: db "sqrt(16) = ", 0
str_20: db "pow(2, 3) = ", 0
str_21: db "", 0
str_22: db "========================================", 0
str_23: db "Program Complete!", 0
str_24: db "FluxSharp v1.0 - Ready for Production", 0
str_25: db "========================================", 0

section .text
; === Compiled from "main.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int


; --- // ============================================================ ---
global print_header
print_header:
    push rbp
    mov rbp, rsp

    ; --- serial_print("========================================"); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    ; --- serial_print("FluxSharp Language Demo"); ---
    lea rdi, [rel str_1]
    call _fsh_print_str

    ; --- serial_print("========================================"); ---
    lea rdi, [rel str_2]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret


; --- } ---
global print_section
print_section:
    push rbp
    mov rbp, rsp

    ; --- serial_print(""); ---
    lea rdi, [rel str_3]
    call _fsh_print_str

    ; --- serial_print(name); ---
    ; ERROR serial_print arg eval: Undefined variable: 'name'

    ; --- serial_print("----------------------------------------"); ---
    lea rdi, [rel str_4]
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
    lea rdi, [rel str_5]
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
    lea rdi, [rel str_6]
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
    lea rdi, [rel str_7]
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
    lea rdi, [rel str_8]
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

    ; --- serial_print("Counting 0 to 4:"); ---
    lea rdi, [rel str_9]
    call _fsh_print_str

    ; --- int i = 0; ---
    sub rsp, 8
    mov qword [rbp-8], 0

    ; --- while (i < 5) { ---

    mov rsp, rbp
    pop rbp
    ret


; --- // ============================================================ ---
global show_math_constants
show_math_constants:
    push rbp
    mov rbp, rsp

    ; --- serial_print(""); ---
    lea rdi, [rel str_10]
    call _fsh_print_str

    ; --- serial_print("Math Constants and Functions:"); ---
    lea rdi, [rel str_11]
    call _fsh_print_str

    ; --- serial_print("----------------------------------------"); ---
    lea rdi, [rel str_12]
    call _fsh_print_str

    ; --- double pi = PI; ---
    sub rsp, 8
    mov rax, [rel double_13]
    mov qword [rbp-8], rax

    ; --- serial_print("PI = "); ---
    lea rdi, [rel str_14]
    call _fsh_print_str

    ; --- serial_print(pi); ---
    lea rdi, [rel double_15]
    call _fsh_print_str

    ; --- double e = E; ---
    sub rsp, 8
    mov rax, [rel double_16]
    mov qword [rbp-16], rax

    ; --- serial_print("E = "); ---
    lea rdi, [rel str_17]
    call _fsh_print_str

    ; --- serial_print(e); ---
    lea rdi, [rel double_18]
    call _fsh_print_str

    ; --- double sqrt16 = sqrt(16); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'sqrt'

    ; --- serial_print("sqrt(16) = "); ---
    lea rdi, [rel str_19]
    call _fsh_print_str

    ; --- serial_print(sqrt16); ---
    ; ERROR serial_print arg eval: Undefined variable: 'sqrt16'

    ; --- double pow23 = pow(2, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Undefined variable: 'pow'

    ; --- serial_print("pow(2, 3) = "); ---
    lea rdi, [rel str_20]
    call _fsh_print_str

    ; --- serial_print(pow23); ---
    ; ERROR serial_print arg eval: Undefined variable: 'pow23'

    mov rsp, rbp
    pop rbp
    ret


; --- // ============================================================ ---
global main
main:
    push rbp
    mov rbp, rsp

    ; --- print_header(); ---
    ; Function call: print_header
    call print_header

    ; --- print_section("Demo 1: Arithmetic Operations"); ---
    ; Function call: print_section
    call print_section

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

    ; --- print_section("Demo 2: Loops"); ---
    ; Function call: print_section
    call print_section

    ; --- show_counting(); ---
    ; Function call: show_counting
    call show_counting

    ; --- show_math_constants(); ---
    ; Function call: show_math_constants
    call show_math_constants

    ; --- serial_print(""); ---
    lea rdi, [rel str_21]
    call _fsh_print_str

    ; --- serial_print("========================================"); ---
    lea rdi, [rel str_22]
    call _fsh_print_str

    ; --- serial_print("Program Complete!"); ---
    lea rdi, [rel str_23]
    call _fsh_print_str

    ; --- serial_print("FluxSharp v1.0 - Ready for Production"); ---
    lea rdi, [rel str_24]
    call _fsh_print_str

    ; --- serial_print("========================================"); ---
    lea rdi, [rel str_25]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

