# FluxSharp - Main Build System
# Compiles FluxSharp code (.fsh) to assembly, then to executable

.PHONY: all clean build run compile-asm compile-binary quickstart help

# Compiler settings
FLUXC := ./flux_compiler/fluxc/target/release/fluxc
AS := as
LD := ld

# Files
FSH_SOURCE := main.fsh
ASM_GENERATED := main.asm
ASM_OBJECT := main.o
EXECUTABLE := program

# Default target: quickstart (build and run everything)
all: quickstart

# QUICKSTART: Build compiler, compile code, assemble, link, and run in one command
quickstart: build-compiler compile-asm compile-binary run
	@echo ""
	@echo "✅ Quickstart complete!"

# Build Rust compiler if needed
build-compiler:
	@if [ ! -f "$(FLUXC)" ]; then \
		echo "🔨 Building FluxSharp compiler (first time only)..."; \
		cd flux_compiler && cargo build --release && cd ..; \
		echo "✅ Compiler ready"; \
	else \
		echo "✅ Compiler already built"; \
	fi

# Step 1: Compile FluxSharp to assembly
compile-asm: build-compiler
	@echo ""
	@echo "📝 Compiling: $(FSH_SOURCE) → $(ASM_GENERATED)"
	@$(FLUXC) compile $(FSH_SOURCE) -o $(ASM_GENERATED) 2>&1 || { echo "❌ Compilation failed"; exit 1; }
	@echo "✅ Assembly generated"

# Step 2: Assemble and link
compile-binary: $(ASM_GENERATED)
	@echo ""
	@echo "⚙️  Assembling: $(ASM_GENERATED) → $(ASM_OBJECT)"
	@$(AS) $(ASM_GENERATED) -o $(ASM_OBJECT) 2>&1 || { echo "❌ Assembly failed"; exit 1; }
	@echo "✅ Object file created"
	@echo ""
	@echo "🔗 Linking: $(ASM_OBJECT) → $(EXECUTABLE)"
	@$(LD) -dynamic-linker /lib64/ld-linux-x86-64.so.2 -lc -o $(EXECUTABLE) $(ASM_OBJECT) 2>&1 || { echo "❌ Linking failed"; exit 1; }
	@echo "✅ Executable created"

# Build without running
build: compile-asm compile-binary
	@echo ""
	@echo "✅ Build complete: ./$(EXECUTABLE)"

# Run the executable
run: $(EXECUTABLE)
	@echo ""
	@echo "🚀 Running: ./$(EXECUTABLE)"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@./$(EXECUTABLE)
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Clean generated files only
clean:
	@echo "🧹 Cleaning build artifacts..."
	@rm -f $(ASM_GENERATED) $(ASM_OBJECT) $(EXECUTABLE)
	@echo "✅ Cleaned"

# Full clean (including compiler)
distclean: clean
	@echo "🧹 Full clean (including compiler)..."
	@cd flux_compiler && cargo clean
	@echo "✅ Full clean complete"

# Help
help:
	@echo "FluxSharp Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Main Targets:"
	@echo "  make quickstart    - Build everything and run (DEFAULT)"
	@echo "  make build         - Compile, assemble, and link"
	@echo "  make run           - Just run executable"
	@echo "  make clean         - Remove generated files"
	@echo "  make distclean     - Full clean (including compiler)"
	@echo "  make help          - Show this help"
	@echo ""
	@echo "Detailed Targets:"
	@echo "  make build-compiler  - Build Rust compiler only"
	@echo "  make compile-asm     - Generate .asm from .fsh"
	@echo "  make compile-binary  - Assemble and link"
	@echo ""
	@echo "Workflow:"
	@echo "  main.fsh (your code)"
	@echo "       ↓ (fluxc compiler)"
	@echo "  main.asm (x86-64 assembly)"
	@echo "       ↓ (as assembler)"
	@echo "  main.o (object file)"
	@echo "       ↓ (ld linker)"
	@echo "  program (executable)"
	@echo "       ↓ (./program)"
	@echo "  Output"
	@echo ""
	@echo "Example:"
	@echo "  make quickstart    # Compile and run everything"
	@echo "  nano main.fsh      # Edit your code"
	@echo "  make               # Build and run (same as quickstart)"

