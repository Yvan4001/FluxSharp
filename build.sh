#!/bin/bash
set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$PROJECT_DIR/flux_compiler"          # ← workspace root
FLUXC_BIN="$COMPILER_DIR/target/release/fluxc"    # ← binaire ici

if [ $# -eq 0 ]; then
    FSH_SOURCE="$PROJECT_DIR/main.fsh"
else
    FSH_SOURCE="$1"
    [[ "$FSH_SOURCE" = /* ]] || FSH_SOURCE="$PROJECT_DIR/$FSH_SOURCE"
fi

EXECUTABLE="$(dirname "$FSH_SOURCE")/program"

echo "🚀 FluxSharp - Complete Build and Run"
echo "======================================="
echo "📄 Input:  $FSH_SOURCE"
echo "📦 Output: $EXECUTABLE"
echo ""

echo "🔍 Checking dependencies..."
command -v cargo >/dev/null || { echo "❌ cargo not found"; exit 1; }
command -v nasm  >/dev/null || { echo "❌ nasm not found (sudo apt install nasm)"; exit 1; }
command -v ld    >/dev/null || { echo "❌ ld not found (sudo apt install binutils)"; exit 1; }
[ -f "$FSH_SOURCE" ]        || { echo "❌ Source not found: $FSH_SOURCE"; exit 1; }
echo "✅ All dependencies found"
echo ""

echo "🔨 Step 1: Building Rust compiler..."
(cd "$COMPILER_DIR" && cargo build --release 2>&1 | tail -5)

[ -f "$FLUXC_BIN" ] || { echo "❌ Binary not found: $FLUXC_BIN"; exit 1; }
echo "✅ Compiler ready: $FLUXC_BIN"
echo ""

echo "📝 Step 2: Compiling FluxSharp → Executable..."
if ! "$FLUXC_BIN" compile "$FSH_SOURCE" -o "$EXECUTABLE" 2>&1; then
    echo "❌ Compilation failed"
    exit 1
fi

[ -f "$EXECUTABLE" ] || { echo "❌ Executable not created: $EXECUTABLE"; exit 1; }
echo "✅ Executable ready: $EXECUTABLE"
echo ""

echo "🚀 Step 3: Running program"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
"$EXECUTABLE"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Done!"