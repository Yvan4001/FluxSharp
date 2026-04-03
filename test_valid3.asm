; ============================
; Flux# compiled from "test_valid3.fsh"
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

section .text
; === Compiled from "test_valid3.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- int x = 10; ---
    sub rsp, 8
    mov qword [rbp-8], 10

    ; --- int y = abs(0 - 42); ---
    sub rsp, 8
    mov rdi, -42
    call _fsh_abs
    mov qword [rbp-16], rax

    ; --- print(x); ---
    mov rdi, 10
    call _fsh_print_int

    ; --- print(y); ---
    ; ❌ ERROR: Complex expression not supported in argument for print

    mov rsp, rbp
    pop rbp
    ret

