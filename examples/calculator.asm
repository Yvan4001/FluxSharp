; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/examples/calculator.fsh"
; fluxc v0.1.0
; ============================

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/examples/calculator.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Calculator_Add
Calculator_Add:
    push rbp
    mov rbp, rsp

    ; --- return a + b; ---

    mov rsp, rbp
    pop rbp
    ret

global Calculator_Multiply
Calculator_Multiply:
    push rbp
    mov rbp, rsp

    ; --- return a * b; ---

    mov rsp, rbp
    pop rbp
    ret


; ---  ---
global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- print("Calculator Example"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- calc.value = 10; ---

    ; --- int sum = calc.Add(5, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int product = calc.Multiply(4, 7); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Done"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

