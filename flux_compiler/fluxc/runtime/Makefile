# FluxSharp Runtime Build System

ARCH := x86_64
OS := Linux
CC := gcc
AS := as
LD := ld
ASFLAGS := -o
LDFLAGS := -r

# Cibles
.PHONY: all clean test

ASM_FILES := scheduler.asm jit.asm
OBJ_FILES := $(ASM_FILES:.asm=.o)
RUNTIME := async_runtime.o

all: $(RUNTIME)

# Assembler chaque fichier .asm
%.o: %.asm
	@echo "Assembling $<..."
	$(AS) $< $(ASFLAGS) $@
	@echo "✓ $@ created"

# Lier tous les objets
$(RUNTIME): $(OBJ_FILES)
	@echo "Linking runtime..."
	$(LD) $(LDFLAGS) -o $@ $(OBJ_FILES)
	@echo "✓ $@ created ($(shell du -h $@ | cut -f1))"
	@echo ""
	@echo "Exported symbols:"
	@nm $@ | grep " T " | awk '{print "  ✓ " $$3}'

# Tester la syntaxe
test: clean all
	@echo ""
	@echo "Testing syntax..."
	@for file in $(ASM_FILES); do \
		echo "  Testing $$file..."; \
		$(AS) $$file -o /tmp/test_$$file.o 2>&1 || exit 1; \
	done
	@echo "✓ All syntax checks passed"

# Nettoyer
clean:
	@rm -f *.o $(RUNTIME)
	@echo "✓ Cleaned"

# Afficher l'aide
help:
	@echo "FluxSharp Runtime Build"
	@echo ""
	@echo "Targets:"
	@echo "  make           - Build runtime"
	@echo "  make test      - Test syntax"
	@echo "  make clean     - Clean build"
	@echo "  make help      - Show this help"

