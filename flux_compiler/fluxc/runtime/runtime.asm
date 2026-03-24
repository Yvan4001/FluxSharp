; fluxc/runtime/runtime.asm
; =============================================
; Flux# Runtime Library — Fonctions internes (Stubs)
; =============================================

section .text

global _start
extern main
_start:
    ; Tentative d'appel main(colle au convention C)
    ; Si main n'est pas défini, le compilateur génère un stub main.
    call main
    mov rdi, rax
    mov rax, 60
    syscall

; --- _fsh_print_str ---
; Affiche une string null-terminated
; Argument : rdi = pointeur vers la string
global _fsh_print_str
_fsh_print_str:
    push rbp
    mov rbp, rsp

    mov rcx, rdi
.find_end:
    cmp byte [rcx], 0
    je .end_len
    inc rcx
    jmp .find_end
.end_len:
    mov rdx, rcx
    sub rdx, rdi

    mov rsi, rdi
    mov rax, 1
    mov rdi, 1
    syscall

    pop rbp
    ret

; --- _fsh_print_int ---
; Affiche un entier signé 64-bit
; Argument : rdi = entier à afficher
global _fsh_print_int
_fsh_print_int:
    push rbp
    mov rbp, rsp

    mov rax, rdi
    mov rsi, buffer + 32
    mov byte [rsi], 0
    mov rcx, 0
    cmp rax, 0
    jge .positive
    mov rbx, 1
    neg rax
    jmp .convert

.positive:
    xor rbx, rbx

.convert:
    mov rdx, 0
    mov r8, 10

.loop_num:
    xor rdx, rdx
    div r8
    add dl, '0'
    dec rsi
    mov [rsi], dl
    inc rcx
    cmp rax, 0
    jne .loop_num

    cmp rbx, 0
    je .write_int
    dec rsi
    mov byte [rsi], '-'
    inc rcx

.write_int:
    mov rdx, rcx
    mov rax, 1
    mov rdi, 1
    syscall

    pop rbp
    ret

section .bss
    buffer resb 40

