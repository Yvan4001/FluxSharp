; fluxc/runtime/runtime.asm
; Flux# Runtime Library - Simplified double printing
section .text
global _start
extern main
_start:
    call main
    mov rdi, rax
    mov rax, 60
    syscall
; --- _fsh_print_str ---
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
    ; Print newline after string
    mov rax, 1          ; write syscall
    mov rdi, 1          ; stdout
    mov rsi, newline
    mov rdx, 1          ; 1 byte
    syscall
    pop rbp
    ret
; --- _fsh_print_int ---
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
    ; Print newline after integer
    mov rax, 1          ; write syscall
    mov rdi, 1          ; stdout
    mov rsi, newline
    mov rdx, 1          ; 1 byte
    syscall
    pop rbp
    ret
; --- _fsh_print_float ---
; Simplified: affiche 3 décimales pour float
global _fsh_print_float
_fsh_print_float:
    push rbp
    mov rbp, rsp
    sub rsp, 40
    ; Convertir 32-bit float en double
    mov eax, edi
    movd xmm0, eax
    cvtss2sd xmm0, xmm0
    movsd [rsp], xmm0
    ; Utiliser la fonction d'affichage double
    lea rsi, [rel fbuffer]
    mov rdi, [rsp]
    call _simple_double_to_str
    ; Afficher
    lea rdi, [rel fbuffer]
    call _fsh_print_str
    add rsp, 40
    pop rbp
    ret
; --- _fsh_print_double ---
; Print a double value
global _fsh_print_double
_fsh_print_double:
    push rbp
    mov rbp, rsp
    sub rsp, 40
    ; rdi = bits of double
    mov [rsp], rdi
    lea rsi, [rel dbuffer]
    movsd xmm0, [rsp]
    call _simple_double_to_str
    ; Print buffer
    lea rdi, [rel dbuffer]
    call _fsh_print_str
    add rsp, 40
    pop rbp
    ret
; --- _simple_double_to_str ---
; Convert double in xmm0 to string in rsi buffer
; Simplified: Show integer part and 2 decimal places
_simple_double_to_str:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    push rbx
    push r12
    
    mov r12, rsi            ; r12 = buffer pointer
    
    ; Check if negative
    xorpd xmm1, xmm1
    comisd xmm0, xmm1
    jae .double_positive
    
    ; Negative number
    mov byte [r12], '-'
    inc r12
    mov rax, 0x8000000000000000
    mov rbx, rax
    movq rax, xmm0
    xor rax, rbx
    movq xmm0, rax
    
.double_positive:
    ; Simple approach: just show "X.YZ" format
    ; Store original double
    movsd [rsp], xmm0
    
    ; Round to nearest integer
    cvttsd2si rax, xmm0     ; Convert to int64 (integer part)
    mov rbx, rax            ; Save integer part
    
    ; Convert integer to string
    mov rdi, rax
    mov rsi, r12
    call _int_to_simple_str
    mov rcx, rax            ; rcx = number of chars written
    add r12, rcx            ; Advance buffer pointer
    
    ; Add decimal point
    mov byte [r12], '.'
    inc r12
    
    ; Get fractional part
    movsd xmm1, [rsp]
    cvtsi2sd xmm2, rbx      ; Convert integer part back
    subsd xmm1, xmm2        ; Fractional part
    
    ; Multiply by 100 for 2 decimal places
    mov rax, 0x4059000000000000  ; 100.0 in double format
    movq xmm2, rax
    mulsd xmm1, xmm2
    
    ; Convert to int
    cvttsd2si rax, xmm1
    cmp rax, 100
    jl .frac_ok
    mov rax, 99
.frac_ok:
    ; Ensure 2 digits (pad with 0)
    cmp rax, 10
    jge .two_digit_frac
    mov byte [r12], '0'
    inc r12
.two_digit_frac:
    mov rdi, rax
    mov rsi, r12
    call _int_to_simple_str
    add r12, rax
    
    ; Null terminate
    mov byte [r12], 0
    
    pop r12
    pop rbx
    add rsp, 48
    pop rbp
    ret

; Helper: Convert integer in rdi to string in rsi
; Returns number of characters in rax (max 20)
_int_to_simple_str:
    push rbp
    mov rbp, rsp
    push rbx
    push rcx
    push rdx
    
    mov rax, rdi            ; rax = number
    mov rbx, rsi            ; rbx = buffer
    mov rcx, 0              ; digit count
    
    ; Handle zero
    test rax, rax
    jnz .skip_zero
    mov byte [rbx], '0'
    mov rax, 1
    jmp .end_simple_str
    
.skip_zero:
    ; Build number backward
    mov r8, rbx
    add r8, 20              ; Temp buffer end
    mov byte [r8], 0
    
.num_loop:
    xor rdx, rdx
    mov r9, 10
    div r9
    add dl, '0'
    dec r8
    mov [r8], dl
    inc rcx
    test rax, rax
    jnz .num_loop
    
    ; Copy to start of buffer
    mov rsi, rbx
    mov rax, rcx
.copy_loop:
    test rcx, rcx
    jz .end_simple_str
    mov dl, [r8]
    mov [rsi], dl
    inc r8
    inc rsi
    dec rcx
    jmp .copy_loop
    
.end_simple_str:
    pop rdx
    pop rcx
    pop rbx
    pop rbp
    ret
; --- _fsh_sqrt ---
global _fsh_sqrt
_fsh_sqrt:
    mov rax, rdi
    ret
; --- _fsh_abs ---
global _fsh_abs
_fsh_abs:
    mov rax, rdi
    ret
; --- _fsh_floor ---
global _fsh_floor
_fsh_floor:
    mov rax, rdi
    ret
; --- _fsh_ceil ---
global _fsh_ceil
_fsh_ceil:
    mov rax, rdi
    ret
section .bss
    buffer resb 40
    fbuffer resb 64
    dbuffer resb 64
section .data
    newline: db 10            ; '\n' character
    multiplier: dq 1000.0
    million: dq 1000000.0
    hundred: dq 100.0
    const_neg_one: dq -1.0
    const_pi: dq 0x400921fb54442d18
    const_e: dq 0x4005bf0a8b145769
    const_ln2: dq 0x3fe62e42fefa39ef
    const_ln10: dq 0x40026bb1bbb55516
    const_sqrt2: dq 0x3ff6a09e667f3bcc
