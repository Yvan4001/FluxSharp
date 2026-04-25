; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/test_calc.fsh"
; fluxc v0.1.0
; ============================

extern _fsh_print_str
extern _fsh_print_int
extern _fsh_string_length
extern _fsh_abs
extern _fsh_max
extern _fsh_min
extern _fsh_pow
extern _fsh_floor
extern _fsh_ceil
extern _fsh_round
extern _fsh_sqrt

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/test_calc.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int
extern _fsh_string_length

global Main_Add
Main_Add:
    push rbp
    mov rbp, rsp

    ; FluxSharp:3 | return a + b;
    mov rax, rdi
    mov rcx, rsi
    add rax, rcx
    mov rsp, rbp
    pop rbp
    ret
global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; FluxSharp:6 | int x = Add(10, 20);
    sub rsp, 8
    mov rdi, 10
    mov rsi, 20
    call Main_Add
    mov qword [rbp-8], rax

    ; FluxSharp:7 | int p = pow(2, 3);
    sub rsp, 8
    mov rdi, 2
    mov rsi, 3
    call _fsh_pow
    mov qword [rbp-16], rax

    ; FluxSharp:8 | print("x is " + x);
    mov rdi, rax
    call _fsh_print_int

    ; FluxSharp:9 | print("p is " + p);
    mov rdi, rax
    call _fsh_print_int

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main_main
    ret

