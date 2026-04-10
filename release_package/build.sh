#!/bin/bash
################################################################################
# FluxSharp Build System - Release Version (No Cargo Required)
# This script compiles FluxSharp code to native executables
# 
# Usage:
#   ./build.sh <source.fsh>          # Compile and run
#   ./build.sh <source.fsh> -o <out> # Compile to output binary
#   ./build.sh --help                # Show this help
################################################################################

set -e

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FLUXC_BIN="$SCRIPT_DIR/bin/fluxc"
RUNTIME_OBJ="$SCRIPT_DIR/lib/runtime_lib.o"
VERSION="0.1.0"

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
    echo "🚀 FluxSharp Compiler v$VERSION - Release Edition"
    echo "══════════════════════════════════════════════════"
    echo ""
}

print_error() {
    echo "❌ Error: $1" >&2
    exit 1
}

print_info() {
    echo "ℹ️  $1"
}

print_success() {
    echo "✅ $1"
}

show_help() {
    cat << EOF
FluxSharp Compiler - Release Edition v$VERSION

USAGE:
    ./build.sh <source.fsh>              Compile and run
    ./build.sh <source.fsh> -o <output>  Compile to binary
    ./build.sh --help                    Show this help
    ./build.sh --version                 Show version

EXAMPLES:
    ./build.sh hello.fsh                    # Compiles hello.fsh and runs it
    ./build.sh myprogram.fsh -o myprogram   # Compiles to myprogram binary

REQUIREMENTS:
    - nasm (assembler)
    - ld (linker, part of binutils)

EOF
}

# ============================================================================
# Argument Parsing
# ============================================================================

if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    show_help
    exit 0
fi

if [ "$1" == "--version" ] || [ "$1" == "-v" ]; then
    echo "FluxSharp v$VERSION"
    exit 0
fi

# Source file is the first argument
SOURCE_FILE="$1"
OUTPUT_FILE=""
RUN_AFTER=true

# Convert source file to absolute path
if [[ "$SOURCE_FILE" != /* ]]; then
    SOURCE_FILE="$(pwd)/$SOURCE_FILE"
fi

# Check for -o flag
if [ $# -ge 3 ] && [ "$2" == "-o" ]; then
    OUTPUT_FILE="$3"
    RUN_AFTER=false
else
    # Default output: same name as source, no extension
    OUTPUT_FILE="${SOURCE_FILE%.*}"
fi

# ============================================================================
# Validation
# ============================================================================

print_header

print_info "Checking dependencies..."

# Check required tools
command -v nasm >/dev/null 2>&1 || print_error "nasm not found. Install with: sudo apt install nasm"
command -v ld >/dev/null 2>&1 || print_error "ld not found. Install with: sudo apt install binutils"
print_success "Dependencies found: nasm, ld"
echo ""

# Check source file
[ -f "$SOURCE_FILE" ] || print_error "Source file not found: $SOURCE_FILE"
print_success "Source file found: $SOURCE_FILE"

# Check compiler binary
[ -f "$FLUXC_BIN" ] || print_error "Compiler binary not found: $FLUXC_BIN"
print_success "Compiler found: $FLUXC_BIN"

# Check runtime object
[ -f "$RUNTIME_OBJ" ] || print_error "Runtime object not found: $RUNTIME_OBJ"
print_success "Runtime found: $RUNTIME_OBJ"
echo ""

# ============================================================================
# Compilation Phase
# ============================================================================

echo "📝 Step 1: Compiling FluxSharp → Assembly..."
echo "  Source: $SOURCE_FILE"

# Determine output paths (in current directory)
OUTPUT_DIR=$(dirname "$OUTPUT_FILE")
OUTPUT_BASE=$(basename "$OUTPUT_FILE")
ASM_FILE="$OUTPUT_DIR/${OUTPUT_BASE}.asm"
OBJ_FILE="$OUTPUT_DIR/${OUTPUT_BASE}.o"

# Create output directory if needed
mkdir -p "$OUTPUT_DIR"

# Compile to Assembly - compiler runs from its own directory to access runtime.o
# It generates files in SCRIPT_DIR, then we copy them to OUTPUT_DIR
(cd "$SCRIPT_DIR" && "$FLUXC_BIN" compile "$SOURCE_FILE" -o "$OUTPUT_FILE" 2>&1) || true

# Copy the generated ASM file to the desired location
if [ -f "$SCRIPT_DIR/${OUTPUT_BASE}.asm" ]; then
    cp "$SCRIPT_DIR/${OUTPUT_BASE}.asm" "$ASM_FILE"
    print_success "Generated assembly: $ASM_FILE"
elif [ -f "$OUTPUT_DIR/${OUTPUT_BASE}.asm" ]; then
    print_success "Generated assembly: $ASM_FILE"
else
    print_error "Assembly file not generated"
fi

echo ""

# ============================================================================
# Assembly Phase
# ============================================================================

echo "🔨 Step 2: Assembling → Object File..."
if ! nasm -f elf64 "$ASM_FILE" -o "$OBJ_FILE" 2>&1; then
    print_error "Assembly failed"
fi

[ -f "$OBJ_FILE" ] || print_error "Object file not created: $OBJ_FILE"
print_success "Generated object file: $OBJ_FILE"
echo ""

# ============================================================================
# Linking Phase
# ============================================================================

echo "🔗 Step 3: Linking → Executable..."
if ! ld -dynamic-linker /lib64/ld-linux-x86-64.so.2 /usr/lib/x86_64-linux-gnu/crt1.o \
        /usr/lib/x86_64-linux-gnu/crti.o /usr/lib/x86_64-linux-gnu/crtn.o \
        -L/usr/lib/x86_64-linux-gnu -L/lib/x86_64-linux-gnu \
        "$OBJ_FILE" "$RUNTIME_OBJ" -lc -o "$OUTPUT_FILE" 2>&1; then
    print_error "Linking failed"
fi

[ -f "$OUTPUT_FILE" ] || print_error "Executable not created: $OUTPUT_FILE"
chmod +x "$OUTPUT_FILE"
print_success "Generated executable: $OUTPUT_FILE"
echo ""

# ============================================================================
# Execution Phase
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
    print_info "Run with: ./$OUTPUT_FILE"
fi

echo ""

