#!/bin/bash
# FluxSharp Complete Build and Run Script

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_ROOT_DIR="$PROJECT_DIR/flux_compiler"
FLUXC_BIN="$COMPILER_ROOT_DIR/target/release/fluxc"
FSH_SOURCE="$PROJECT_DIR/main.fsh"
EXECUTABLE="$PROJECT_DIR/program"

echo "🚀 FluxSharp - Complete Build and Run"
echo "======================================="
echo ""

# Check dependencies
echo "🔍 Checking dependencies..."
command -v cargo >/dev/null || { echo "❌ cargo not found"; exit 1; }
echo "✅ All dependencies found"
echo ""

# Build Rust compiler completely
echo "🔨 Step 1: Building Rust compiler with cargo build..."
echo "   Directory: $COMPILER_ROOT_DIR"
cd "$COMPILER_ROOT_DIR"

echo "   Building runtime and async support..."
cargo build --release 2>&1 | tail -5

cd "$PROJECT_DIR"

# Verify binary exists
if [ ! -f "$FLUXC_BIN" ]; then
    echo "❌ Compiler binary not found at: $FLUXC_BIN"
    exit 1
fi
echo "✅ Compiler ready at: $FLUXC_BIN"
echo ""

# Compile FluxSharp directly to executable
echo "📝 Step 2: Compiling FluxSharp → Executable"
echo "   Input:  $FSH_SOURCE"
echo "   Output: $EXECUTABLE"
if ! "$FLUXC_BIN" compile "$FSH_SOURCE" -o "$EXECUTABLE" 2>&1; then
    echo "❌ Compilation failed"
    exit 1
fi
echo "✅ Executable created"
echo ""

# Execute
echo "🚀 Step 3: Executing Program"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -x "$EXECUTABLE" ]; then
    "$EXECUTABLE" || true
else
    echo "⚠️  Executable not marked as executable, trying anyway..."
    bash "$EXECUTABLE" 2>/dev/null || "$EXECUTABLE" 2>/dev/null || echo "Failed to execute"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ Build complete!"
echo ""

