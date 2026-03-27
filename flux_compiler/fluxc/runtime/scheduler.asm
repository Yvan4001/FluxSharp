; FluxSharp Async Task Scheduler (CORRECTED)
; Pure AT&T x86-64 syntax with proper % and ; usage

.section .data
    ; Constants
    TCB_SIZE = 256
    TCB_STATE = 0
    TCB_RIP = 8
    TCB_RSP = 16
    TCB_RBP = 24
    TCB_EVENT = 32
    TCB_RESULT = 40
    TCB_DATA = 48
    
    STATE_READY = 1
    STATE_RUNNING = 2
    STATE_WAITING = 3
    STATE_DONE = 4
    
    ; Global data
    .align 8
    task_queue: .space 2048
    queue_head: .quad 0
    queue_tail: .quad 0
    queue_size: .quad 0
    current_task: .quad 0

.section .text
    .globl init_scheduler
    .globl create_task
    .globl spawn_task
    .globl enqueue_task
    .globl dequeue_task

;============================================================================
; init_scheduler - Initialize scheduler state
;============================================================================
init_scheduler:
    push %rbp
    mov %rsp, %rbp
    
    xor %rax, %rax
    mov %rax, queue_head(%rip)
    mov %rax, queue_tail(%rip)
    mov %rax, queue_size(%rip)
    mov %rax, current_task(%rip)
    
    pop %rbp
    ret

;============================================================================
; create_task(rdi=fn_ptr) -> rax=tcb_ptr
;============================================================================
create_task:
    push %rbp
    mov %rsp, %rbp
    push %rbx
    
    mov %rdi, %rbx
    call alloc_tcb
    
    ; Initialize TCB
    mov $STATE_READY, %cl
    mov %cl, TCB_STATE(%rax)
    mov %rbx, TCB_RIP(%rax)
    
    lea TCB_DATA+192(%rax), %rcx
    mov %rcx, TCB_RSP(%rax)
    mov %rcx, TCB_RBP(%rax)
    
    movl $0, TCB_EVENT(%rax)
    movq $0, TCB_RESULT(%rax)
    
    pop %rbx
    pop %rbp
    ret

;============================================================================
; alloc_tcb - Simple static allocation
;============================================================================
.section .bss
    tcb_pool: .space 65536
    tcb_pool_ptr: .quad 0

.section .text
alloc_tcb:
    push %rbp
    mov %rsp, %rbp
    
    mov tcb_pool_ptr(%rip), %rax
    lea tcb_pool(%rip), %rcx
    add %rcx, %rax
    
    mov tcb_pool_ptr(%rip), %rcx
    add $256, %rcx
    mov %rcx, tcb_pool_ptr(%rip)
    
    pop %rbp
    ret

;============================================================================
; enqueue_task(rdi=tcb_ptr)
;============================================================================
enqueue_task:
    push %rbp
    mov %rsp, %rbp
    push %rbx
    
    mov queue_size(%rip), %rax
    cmp $256, %rax
    jge .enqueue_done
    
    mov queue_tail(%rip), %rcx
    lea task_queue(%rip), %rax
    mov %rdi, (%rax, %rcx, 8)
    
    inc %rcx
    cmp $256, %rcx
    jne .enqueue_skip_wrap
    xor %rcx, %rcx
.enqueue_skip_wrap:
    mov %rcx, queue_tail(%rip)
    
    mov queue_size(%rip), %rax
    inc %rax
    mov %rax, queue_size(%rip)
    
.enqueue_done:
    pop %rbx
    pop %rbp
    ret

;============================================================================
; dequeue_task() -> rax=tcb_ptr or 0
;============================================================================
dequeue_task:
    push %rbp
    mov %rsp, %rbp
    
    mov queue_size(%rip), %rax
    cmp $0, %rax
    je .dequeue_empty
    
    mov queue_head(%rip), %rcx
    lea task_queue(%rip), %rax
    mov (%rax, %rcx, 8), %rax
    
    mov queue_head(%rip), %rcx
    inc %rcx
    cmp $256, %rcx
    jne .dequeue_skip_wrap
    xor %rcx, %rcx
.dequeue_skip_wrap:
    mov %rcx, queue_head(%rip)
    
    mov queue_size(%rip), %rcx
    dec %rcx
    mov %rcx, queue_size(%rip)
    
    pop %rbp
    ret
    
.dequeue_empty:
    xor %rax, %rax
    pop %rbp
    ret

;============================================================================
; spawn_task(rdi=fn_ptr) - Public API
;============================================================================
spawn_task:
    push %rbp
    mov %rsp, %rbp
    
    call create_task
    mov %rax, %rdi
    call enqueue_task
    
    pop %rbp
    ret

