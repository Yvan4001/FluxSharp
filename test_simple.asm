; ============================
; Flux# compiled from "test_simple.fsh"
; fluxc v0.1.0
; ============================

section .data
str_0: db "Hello", 0

section .text
; === Compiled from "test_simple.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int


; --- public void Main() { print("Hello"); } ---
global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- public void Main() { print("Hello"); } ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

