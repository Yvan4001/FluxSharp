; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/release_package/examples/hello.fsh"
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

section .data
str_0: db "Hello, FluxSharp!", 0

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/release_package/examples/hello.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int
extern _fsh_string_length

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- print("Hello, FluxSharp!"); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main_main
    ret

