; ============================
; Flux# compiled from "test_float_error.fsh"
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
double_0: dq 0x40091eb851eb851f
double_1: db "3.14", 0

section .text
; === Compiled from "test_float_error.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- float f = 3.14; ---
    sub rsp, 8
    mov rax, [rel double_0]
    mov qword [rbp-8], rax

    ; --- print(f); ---
    lea rdi, [rel double_1]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

