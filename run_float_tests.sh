#!/bin/bash
# Test script pour les types float et double

echo "================================"
echo "Testing Float and Double Support"
echo "================================"

cd /run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -f test_floats*.bin test_floats*.o test_floats*.asm
rm -f flux_compiler/fluxc/runtime/runtime.o

echo ""
echo "✅ Testing basic float and double syntax..."
./flux_compiler/target/debug/fluxc compile test_floats.fsh -o test_floats.bin
if [ $? -eq 0 ]; then
    echo "✅ Compilation successful!"
    echo "Running program:"
    ./test_floats.bin
    echo ""
else
    echo "❌ Compilation failed!"
    exit 1
fi

echo ""
echo "✅ Testing comprehensive float and double operations..."
./flux_compiler/target/debug/fluxc compile test_floats_complete.fsh -o test_floats_complete.bin
if [ $? -eq 0 ]; then
    echo "✅ Compilation successful!"
    echo "Running program:"
    ./test_floats_complete.bin
    echo ""
else
    echo "❌ Compilation failed!"
    exit 1
fi

echo "================================"
echo "✅ All tests passed!"
echo "================================"

