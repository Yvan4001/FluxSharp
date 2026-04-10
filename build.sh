#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$PROJECT_DIR/flux_compiler"          # ← workspace root
FLUXC_BIN="$COMPILER_DIR/target/release/fluxc"    # ← binaire ici
TEST_SCRIPT="$PROJECT_DIR/test.sh"

if [ $# -eq 0 ]; then
    FSH_SOURCE="$PROJECT_DIR/main.fsh"
    RUN_TESTS=false
elif [ "$1" == "--test" ] || [ "$1" == "-t" ]; then
    RUN_TESTS=true
    FSH_SOURCE=""
else
    FSH_SOURCE="$1"
    [[ "$FSH_SOURCE" = /* ]] || FSH_SOURCE="$PROJECT_DIR/$FSH_SOURCE"
    RUN_TESTS=false
fi

EXECUTABLE="$(dirname "$FSH_SOURCE")/program" 2>/dev/null || EXECUTABLE=""

echo "🚀 FluxSharp - Complete Build and Run"
echo "======================================="
if [ "$RUN_TESTS" = true ]; then
    echo "Mode: COMPREHENSIVE TESTING"
    echo "Tests: Bounds Checking, Security, Async/Await, Exceptions"
else
    echo "📄 Input:  $FSH_SOURCE"
    echo "📦 Output: $EXECUTABLE"
fi
echo ""

echo "🔍 Checking dependencies..."
command -v cargo >/dev/null || { echo "❌ cargo not found"; exit 1; }
command -v nasm  >/dev/null || { echo "❌ nasm not found (sudo apt install nasm)"; exit 1; }
command -v ld    >/dev/null || { echo "❌ ld not found (sudo apt install binutils)"; exit 1; }
if [ "$RUN_TESTS" = false ]; then
    [ -f "$FSH_SOURCE" ]        || { echo "❌ Source not found: $FSH_SOURCE"; exit 1; }
fi
echo "✅ All dependencies found"
echo ""

echo "🔨 Step 1: Building Rust compiler..."
(cd "$COMPILER_DIR" && cargo build --release)

[ -f "$FLUXC_BIN" ] || { echo "❌ Binary not found: $FLUXC_BIN"; exit 1; }
echo "✅ Compiler ready: $FLUXC_BIN"
echo ""

# If testing mode, run comprehensive tests
if [ "$RUN_TESTS" = true ]; then
    echo "🧪 Running comprehensive test suite..."
    echo ""
    bash "$TEST_SCRIPT"
    exit $?
fi

echo "📝 Step 2: Compiling FluxSharp → Executable..."
(cd "$PROJECT_DIR" && "$FLUXC_BIN" compile "$FSH_SOURCE" -o "$EXECUTABLE" 2>&1)
if [ $? -ne 0 ]; then
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
echo ""
echo "💡 Tip: Run './build.sh --test' to run comprehensive test suite"
