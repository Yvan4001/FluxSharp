; ============================
; Flux# compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh"
; fluxc v0.1.0
; ============================

section .text
; === Compiled from "/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp/main.fsh" by fluxc ===
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


; --- // Simple function ---
global PrintMessage
PrintMessage:
    push rbp
    mov rbp, rsp

    ; --- print(msg); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // Function with array ---
global DemoArrays
DemoArrays:
    push rbp
    mov rbp, rsp

    ; --- int[10] numbers; ---

    ; --- numbers[0] = 10; ---

    ; --- numbers[1] = 20; ---

    ; --- numbers[2] = 30; ---

    ; --- int first = numbers[0]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int second = numbers[1]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int third = numbers[2]; ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Array values:"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // Function with class ---
global DemoClass
DemoClass:
    push rbp
    mov rbp, rsp

    ; --- Calculator calc = new Calculator(); ---
    sub rsp, 8

    ; --- calc.value = 42; ---

    ; --- int sum = calc.Add(5, 3); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- int product = calc.Multiply(4, 7); ---
    sub rsp, 8
    ; ERROR evaluating expr: Unexpected atom: primary

    ; --- print("Calculator demo"); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // Control flow demo ---
global DemoControlFlow
DemoControlFlow:
    push rbp
    mov rbp, rsp

    ; --- for (int i = 0; i < 5; i++) { ---

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret


; --- // Main entry point ---
global Main
Main:
    push rbp
    mov rbp, rsp

    ; --- print("=== FluxSharp Features ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- PrintMessage("Constants and basic types work"); ---
    ; Function call: PrintMessage
    call PrintMessage

    ; --- DemoArrays(); ---
    ; Function call: DemoArrays
    call DemoArrays

    ; --- DemoClass(); ---
    ; Function call: DemoClass
    call DemoClass

    ; --- DemoControlFlow(); ---
    ; Function call: DemoControlFlow
    call DemoControlFlow

    ; --- print("=== Complete ==="); ---
    ; ERROR print arg eval: complex expression not supported

    ; --- return; ---

    mov rsp, rbp
    pop rbp
    ret

global main
main:
    call Main
    ret

