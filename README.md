# Flux# (F-Sharp Systems)

**Flux#** is a high-performance, memory-safe systems programming language designed specifically for the **FluxGridOS** kernel. It aims to provide the elegant syntax and asynchronous patterns of **C#** while enforcing the strict memory safety and zero-cost abstractions of **Rust**.

## 🚀 The Vision: "C# Syntax, Rust Rigor"

Developing a kernel in pure Rust (currently ~50k LOC in FluxGridOS) provides safety but often leads to complex, verbose code. Flux# solves this by offering a "cleaner" alternative for high-level kernel components (drivers, file systems, user-mode apps) without the overhead of a Garbage Collector (GC) or a Virtual Machine (VM).

### Key Features
* **Zero-GC / Zero-Runtime**: Compiles directly to machine code (AOT) via LLVM.
* **Ownership & Borrowing**: Static memory management inspired by Rust, but with a C#-like keyword set.
* **Native Async/Await**: Integrated state-machine generation for non-blocking I/O without a heavy runtime.
* **Kernel-First Design**: Built-in support for physical memory mapping, port I/O, and interrupt handling.

---

## 🛠 Syntax Overview

### Memory Ownership (No GC)
Flux# eliminates the Garbage Collector. It uses a **Move/Borrow** system. Objects are stack-allocated by default or managed via explicit kernel allocators.

```csharp
// Flux# Memory Safety
public void InitializeNetwork(owned PacketBuffer buffer) {
    // 'owned' keyword transfers ownership like Rust's 'move'
    NetworkCard.Send(buffer); 
    
    // ERROR: buffer cannot be accessed here anymore.
    // buffer.Clear(); 
}

```

## Compiler Architecture
The compiler is built in Rust to ensure its own stability and uses LLVM as the backend to target x86_64.

Lexer/Parser: Built with Pest to handle the C#-inspired grammar.

Semantic Analyzer: Implements the "Rigor Layer" (Borrow Checker and Type Inference).

Intermediate Representation: Lowers code to LLVM IR.

OS Integration: Links directly with the existing FluxGridOS bootloader and kernel symbols.

## Roadmap
[ ] Phase 1: Bootstrap: Basic Lexer/Parser for C# syntax.

[ ] Phase 2: The Rigor: Static analysis for ownership and lifetimes.

[ ] Phase 3: CodeGen: LLVM backend for x86_64 Long Mode.

[ ] Phase 4: OS Bridge: FFI support to call existing Rust kernel functions.

[ ] Phase 5: Async Power: Implementing the Task state-machine generator.

## Credits & License
Flux# is part of the FluxGridOS ecosystem.
Developed by Yvan Simon.

Licensed under the FluxGridOS Proprietary License (See LICENSE for details).