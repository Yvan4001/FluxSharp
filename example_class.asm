; ============================
; Flux# compiled from "../example_class.fsh"
; fluxc v0.1.0
; ============================

section .data
str_0: db "var x + y: 20", 0

section .text
; === Compiled from "../example_class.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- serial_print("var x + y: " + i); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

