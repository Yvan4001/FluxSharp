; ============================
; Flux# compiled from "examples/demo_import.fsh"
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
str_0: db "Result: ", 0

section .text
; === Compiled from "examples/demo_import.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global MathHelper_GetDouble
MathHelper_GetDouble:
    push rbp
    mov rbp, rsp

    ; --- return x * 2; ---

    mov rsp, rbp
    pop rbp
    ret

global MathHelper_GetTriple
MathHelper_GetTriple:
    push rbp
    mov rbp, rsp

    ; --- return x * 3; ---

    mov rsp, rbp
    pop rbp
    ret

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- MathHelper math = new MathHelper(); ---
    sub rsp, 8

    ; --- int value = math.GetDouble(15); ---
    sub rsp, 8
    mov qword [rbp-16], 0 ; Method call stub: math.GetDouble

    ; --- print("Result: "); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    ; --- print(value); ---
    ; ❌ ERROR: Complex expression not supported in argument for print

    mov rsp, rbp
    pop rbp
    ret

