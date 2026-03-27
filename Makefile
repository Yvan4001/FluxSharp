# FluxSharp - Main Build System
# Compiles FluxSharp code (.fsh) to assembly, then to executable

.PHONY: all clean build run compile-asm compile-binary help

# Compiler settings
FLUXC := ./flux_compiler/fluxc/target/release/fluxc
AS := as
LD := ld

# Files
FSH_SOURCE := main.fsh
ASM_GENERATED := main.asm
ASM_OBJECT := main.o
EXECUTABLE := program

# Targets
all: build run

build: compile-asm compile-binary
	@echo "✓ Build complete"

# Step 1: Compile FluxSharp code to assembly using Rust compiler
compile-asm: $(FSH_SOURCE)
	@if [ ! -f "$(FLUXC)" ]; then \
		echo "Building FluxSharp compiler..."; \
		cd flux_compiler && cargo build --release; \
	fi
	@echo "Compiling: $(FSH_SOURCE) → $(ASM_GENERATED)"
	$(FLUXC) compile $(FSH_SOURCE) -o $(ASM_GENERATED)
	@echo "✓ Assembly generated: $(ASM_GENERATED)"

# Step 2: Compile assembly to object file
compile-binary: $(ASM_GENERATED)
	@echo "Assembling: $(ASM_GENERATED) → $(ASM_OBJECT)"
	$(AS) $(ASM_GENERATED) -o $(ASM_OBJECT)
	@echo "✓ Object file created: $(ASM_OBJECT)"
	@echo "Linking: $(ASM_OBJECT) → $(EXECUTABLE)"
	$(LD) -dynamic-linker /lib64/ld-linux-x86-64.so.2 -lc -o $(EXECUTABLE) $(ASM_OBJECT)
	@echo "✓ Executable created: $(EXECUTABLE)"

run: $(EXECUTABLE)
	@echo "Running: $(EXECUTABLE)"
	./$(EXECUTABLE)
	@echo "✓ Execution complete"

clean:
	@rm -f $(ASM_GENERATED) $(ASM_OBJECT) $(EXECUTABLE)
	@echo "✓ Cleaned build artifacts"

distclean: clean
	@cd flux_compiler && cargo clean
	@echo "✓ Full clean (including compiler)"

help:
	@echo "FluxSharp Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  make              - Compile, assemble, and run"
	@echo "  make build        - Compile .fsh → .asm → binary"
	@echo "  make compile-asm  - Compile .fsh → .asm (using Rust compiler)"
	@echo "  make compile-binary - Assemble .asm → executable"
	@echo "  make run          - Execute the program"
	@echo "  make clean        - Remove generated files"
	@echo "  make distclean    - Clean everything including compiler"
	@echo "  make help         - Show this help"
	@echo ""
	@echo "Workflow:"
	@echo "  main.fsh (FluxSharp source)"
	@echo "       ↓ (fluxc compiler)"
	@echo "  main.asm (x86-64 assembly)"
	@echo "       ↓ (as assembler)"
	@echo "  main.o (object file)"
	@echo "       ↓ (ld linker)"
	@echo "  program (executable)"


