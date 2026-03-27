; FluxSharp Async JIT Compiler (CORRECTED)
; Pure AT&T x86-64 syntax

.section .data
    .align 4096
    jit_buffer: .space 1048576
    jit_offset: .quad 0
    
    .align 8
    function_table: .space 2048
    function_count: .quad 0

.section .text
    .globl init_jit
    .globl compile_call
    .globl register_function
    .globl get_result

;============================================================================
; init_jit - Initialize JIT compiler
;============================================================================
init_jit:
    push %rbp
    mov %rsp, %rbp
    
    xor %rax, %rax
    mov %rax, jit_offset(%rip)
    mov %rax, function_count(%rip)
    
    pop %rbp
    ret

;============================================================================
; compile_call(rdi=name, rsi=arg_count, rdx=args) -> rax=compiled_code
;============================================================================
compile_call:
    push %rbp
    mov %rsp, %rbp
    
    mov jit_offset(%rip), %rax
    lea jit_buffer(%rip, %rax), %rcx
    
    ; Simple: just return the function address for now
    ; In production: generate actual machine code
    mov %rcx, %rax
    
    pop %rbp
    ret

;============================================================================
; register_function(rdi=name, rsi=address)
;============================================================================
register_function:
    push %rbp
    mov %rsp, %rbp
    
    mov function_count(%rip), %rax
    cmp $256, %rax
    jge .register_done
    
    lea function_table(%rip), %rcx
    mov %rsi, (%rcx, %rax, 8)
    
    inc %rax
    mov %rax, function_count(%rip)
    
.register_done:
    pop %rbp
    ret

;============================================================================
; get_result(rdi=tcb_ptr) -> rax=result
;============================================================================
get_result:
    push %rbp
    mov %rsp, %rbp
    
    ; TCB offset for result field = 40
    mov 40(%rdi), %rax
    
    pop %rbp
    ret

