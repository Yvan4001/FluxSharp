#!/bin/bash
################################################################################
# FluxSharp Build System - Release Version (No Cargo Required)
# Usage:
#   ./build.sh <source.fsh>          # Compile and run
#   ./build.sh <source.fsh> -o <out> # Compile to output binary
#   ./build.sh --help                # Show this help
################################################################################

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FLUXC_BIN="$SCRIPT_DIR/bin/fluxc"
RUNTIME_OBJ="$SCRIPT_DIR/lib/runtime.o"   # ✅ Fix 1 : runtime.o pas runtime_lib.o
VERSION="1.0.2"

print_header()  { echo "🚀 FluxSharp Compiler v$VERSION - Release Edition"; echo "══════════════════════════════════════════════════"; echo ""; }
print_error()   { echo "❌ Error: $1" >&2; exit 1; }
print_info()    { echo "ℹ️  $1"; }
print_success() { echo "✅ $1"; }

show_help() {
    cat << EOF
FluxSharp Compiler - Release Edition v$VERSION

USAGE:
    ./build.sh <source.fsh>              Compile and run
    ./build.sh <source.fsh> -o <output>  Compile to binary
    ./build.sh --help                    Show this help
    ./build.sh --version                 Show version

EXAMPLES:
    ./build.sh hello.fsh
    ./build.sh myprogram.fsh -o myprogram

REQUIREMENTS:
    - nasm (assembler)
    - ld (linker, part of binutils)
EOF
}

# ============================================================================
# Arguments
# ============================================================================
if [ $# -eq 0 ];                              then show_help; exit 0; fi
if [ "$1" == "--help" ] || [ "$1" == "-h" ];  then show_help; exit 0; fi
if [ "$1" == "--version" ] || [ "$1" == "-v" ]; then echo "FluxSharp v$VERSION"; exit 0; fi

SOURCE_FILE="$1"
OUTPUT_FILE=""
RUN_AFTER=true

[[ "$SOURCE_FILE" = /* ]] || SOURCE_FILE="$(pwd)/$SOURCE_FILE"

if [ $# -ge 3 ] && [ "$2" == "-o" ]; then
    OUTPUT_FILE="$3"
    RUN_AFTER=false
else
    OUTPUT_FILE="${SOURCE_FILE%.*}"
fi

# ============================================================================
# Validation
# ============================================================================
print_header
print_info "Checking dependencies..."

command -v nasm >/dev/null 2>&1 || print_error "nasm not found. Install: sudo apt install nasm"
command -v ld   >/dev/null 2>&1 || print_error "ld not found. Install: sudo apt install binutils"
print_success "Dependencies found: nasm, ld"
echo ""

[ -f "$SOURCE_FILE" ] || print_error "Source file not found: $SOURCE_FILE"
print_success "Source file found: $SOURCE_FILE"

[ -f "$FLUXC_BIN" ]   || print_error "Compiler not found: $FLUXC_BIN"
print_success "Compiler found: $FLUXC_BIN"

[ -f "$RUNTIME_OBJ" ] || print_error "Runtime not found: $RUNTIME_OBJ"
print_success "Runtime found: $RUNTIME_OBJ"
echo ""

# ============================================================================
# Step 1 — FluxSharp → ASM
# ============================================================================
echo "📝 Step 1: Compiling FluxSharp → Assembly..."
echo "  Source: $SOURCE_FILE"

OUTPUT_DIR=$(dirname "$OUTPUT_FILE")
OUTPUT_BASE=$(basename "$OUTPUT_FILE")
ASM_FILE="$OUTPUT_DIR/${OUTPUT_BASE}.asm"
OBJ_FILE="$OUTPUT_DIR/${OUTPUT_BASE}.o"

mkdir -p "$OUTPUT_DIR"

# fluxc génère le .asm à côté du .fsh source
# On l'exécute depuis SCRIPT_DIR pour éviter les chemins relatifs cassés
(cd "$SCRIPT_DIR" && "$FLUXC_BIN" compile "$SOURCE_FILE" -o "$OUTPUT_FILE" 2>&1) || true

# Récupérer le .asm généré (fluxc le place à côté du .fsh)
GENERATED_ASM="${SOURCE_FILE%.fsh}.asm"
if [ -f "$GENERATED_ASM" ]; then
    [ "$GENERATED_ASM" != "$ASM_FILE" ] && cp "$GENERATED_ASM" "$ASM_FILE"
    print_success "Generated assembly: $ASM_FILE"
else
    print_error "Assembly file not generated (expected: $GENERATED_ASM)"
fi
echo ""

# ============================================================================
# Step 2 — ASM → .o
# ============================================================================
echo "🔨 Step 2: Assembling → Object File..."
nasm -f elf64 "$ASM_FILE" -o "$OBJ_FILE" || print_error "Assembly failed"
[ -f "$OBJ_FILE" ] || print_error "Object file not created: $OBJ_FILE"
print_success "Object file: $OBJ_FILE"
echo ""

# ============================================================================
# Step 3 — Linking
# ✅ Fix 2 : ld simple sans crt1.o/crti.o/crtn.o
# Le runtime FluxSharp fournit son propre _start, pas besoin des CRT glibc
# ============================================================================
echo "🔗 Step 3: Linking → Executable..."
ld -o "$OUTPUT_FILE" "$OBJ_FILE" "$RUNTIME_OBJ" || print_error "Linking failed"

[ -f "$OUTPUT_FILE" ] || print_error "Executable not created: $OUTPUT_FILE"
chmod +x "$OUTPUT_FILE"
print_success "Executable: $OUTPUT_FILE"
echo ""

# ============================================================================
# Step 4 — Run
# ============================================================================
if [ "$RUN_AFTER" = true ]; then
    echo "🚀 Step 4: Running program"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    "$OUTPUT_FILE"
    EXIT_CODE=$?
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    if [ $EXIT_CODE -eq 0 ]; then
        print_success "Program completed successfully"
    else
        echo "⚠️  Program exited with code: $EXIT_CODE"
    fi
else
    print_success "Build complete! Binary: $OUTPUT_FILE"
    print_info "Run with: $OUTPUT_FILE"
fi

echo ""
