; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/test_suite/bounds_check_valid.fsh"
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
extern _fsh_round
extern _fsh_sqrt

section .data
str_0: db "✅ Bounds check valid: PASS", 0

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/test_suite/bounds_check_valid.fsh" by fluxc ===
extern _fsh_print_str
extern _fsh_print_int

global Main_main
Main_main:
    push rbp
    mov rbp, rsp

    ; --- int[10] arr; ---

    ; --- arr[0] = 42; ---

    ; --- arr[5] = 100; ---

    ; --- arr[9] = 999; ---

    ; --- print("✅ Bounds check valid: PASS"); ---
    lea rdi, [rel str_0]
    call _fsh_print_str

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main_main
    ret

