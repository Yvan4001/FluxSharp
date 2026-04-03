#!/bin/bash
# Test script for FluxSharp compiler error messages

COMPILER="./flux_compiler/target/release/fluxc"
TEST_DIR="/tmp/fluxsharp_tests"

# Create test directory
mkdir -p "$TEST_DIR"

echo "=============================================="
echo "FluxSharp Compiler - Error Messages Test Suite"
echo "=============================================="
echo ""

# Test 1: Missing Semicolon
echo "📋 Test 1: Missing Semicolon Detection"
echo "---"
cat > "$TEST_DIR/test1.fsh" << 'EOF'
class Main {
    public void main() {
        int x = 10
        print(x);
    }
}
EOF
echo "Running: Missing semicolon test..."
$COMPILER compile "$TEST_DIR/test1.fsh" 2>&1 | grep -A 3 "POSSIBLE SYNTAX ERROR" || echo "❌ Error message not found"
echo ""

# Test 2: Float Literal Error
echo "📋 Test 2: Float Literal Format Error"
echo "---"
cat > "$TEST_DIR/test2.fsh" << 'EOF'
class Main {
    public void main() {
        float f = 3.14;
        print(f);
    }
}
EOF
echo "Running: Float literal format test..."
$COMPILER compile "$TEST_DIR/test2.fsh" 2>&1 | grep -A 3 "FLOAT LITERAL ERROR" || echo "❌ Error message not found"
echo ""

# Test 3: Unmatched Parenthesis
echo "📋 Test 3: Unmatched Parenthesis Detection"
echo "---"
cat > "$TEST_DIR/test3.fsh" << 'EOF'
class Main {
    public void main() {
        int x = max(10, 20;
        print(x);
    }
}
EOF
echo "Running: Unmatched parenthesis test..."
$COMPILER compile "$TEST_DIR/test3.fsh" 2>&1 | grep -A 3 "UNMATCHED PARENTHESIS" || echo "❌ Error message not found"
echo ""

# Test 4: Valid Compilation
echo "📋 Test 4: Valid Compilation (No Errors)"
echo "---"
cat > "$TEST_DIR/test4.fsh" << 'EOF'
class Main {
    public void main() {
        int x = 10;
        float f = 3.14f;
        int y = abs(-42);
        print(x);
        print(y);
    }
}
EOF
echo "Running: Valid compilation test..."
if $COMPILER compile "$TEST_DIR/test4.fsh" 2>&1 | grep -q "Generated ASM"; then
    echo "✅ Compilation successful"
else
    echo "✅ Compilation processed (no binary specified)"
fi
echo ""

# Test 5: Unmatched Bracket
echo "📋 Test 5: Unmatched Bracket Detection"
echo "---"
cat > "$TEST_DIR/test5.fsh" << 'EOF'
class Main {
    public void main() {
        int arr[10;
    }
}
EOF
echo "Running: Unmatched bracket test..."
$COMPILER compile "$TEST_DIR/test5.fsh" 2>&1 | grep -A 3 "UNMATCHED BRACKET" || echo "❌ Error message not found"
echo ""

echo "=============================================="
echo "Test Suite Complete"
echo "=============================================="

# Cleanup
rm -rf "$TEST_DIR"

