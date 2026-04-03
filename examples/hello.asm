; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/examples/hello.fsh"
; fluxc v0.1.0
; ============================

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/examples/hello.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int


; ---  ---
global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- print("Hello from FluxSharp!"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- print("This is a simple example"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

