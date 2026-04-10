#!/bin/bash
# Quick rebuild and test script

echo "🔧 Rebuilding FluxSharp compiler..."
echo ""

cd "$(dirname "$0")"

# Clean and rebuild
cd flux_compiler/fluxc
echo "📦 Cleaning Cargo cache..."
rm -rf target

echo "🔨 Building Rust compiler with cargo..."
if ! cargo build --release; then
    echo "❌ Compilation failed!"
    exit 1
fi

cd ../..

echo ""
echo "✅ Compiler rebuilt successfully!"
echo ""
echo "🧪 Running comprehensive tests..."
echo ""

chmod +x build.sh
./build.sh --test

