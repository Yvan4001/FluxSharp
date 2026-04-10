#!/bin/bash
# Diagnostic test script for FluxSharp

export PATH="$HOME/.cargo/bin:$PATH"

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPILER_DIR="$PROJECT_DIR/flux_compiler"
FLUXC_BIN="$COMPILER_DIR/target/release/fluxc"
TEST_DIR="$PROJECT_DIR/test_suite"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}FluxSharp Diagnostic Test${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""

# Check compiler
if [ ! -f "$FLUXC_BIN" ]; then
    echo -e "${RED}❌ Compiler not found at: $FLUXC_BIN${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Compiler found${NC}"
echo ""

# Create test file
mkdir -p "$TEST_DIR"
TEST_FILE="$TEST_DIR/test_bounds.fsh"

cat > "$TEST_FILE" << 'EOF'
public class Main {
    public void main() {
        int[10] arr;
        arr[0] = 42;
        arr[5] = 100;
        arr[9] = 999;
        print("✅ Bounds check valid: PASS");
    }
}
EOF

echo -e "${BLUE}Test file created: $TEST_FILE${NC}"
echo ""
echo -e "${YELLOW}Content of test file:${NC}"
cat "$TEST_FILE"
echo ""
echo -e "${BLUE}───────────────────────────────────────────────────────${NC}"
echo ""

# Try compilation
TEST_PROG="$TEST_DIR/test_prog"
echo -e "${YELLOW}Attempting compilation...${NC}"
echo "Command: $FLUXC_BIN compile \"$TEST_FILE\" -o $TEST_PROG"
echo ""

compile_output=$("$FLUXC_BIN" compile "$TEST_FILE" -o "$TEST_PROG" 2>&1)
compile_exit=$?

echo -e "${YELLOW}Compilation exit code: $compile_exit${NC}"
echo ""

if [ $compile_exit -ne 0 ]; then
    echo -e "${RED}❌ Compilation FAILED${NC}"
    echo ""
    echo -e "${YELLOW}Error output:${NC}"
    echo "$compile_output"
else
    echo -e "${GREEN}✅ Compilation SUCCEEDED${NC}"
    echo ""
    
    # Check if executable was created
    if [ -f "$TEST_PROG" ]; then
        echo -e "${GREEN}✅ Executable created at $TEST_PROG${NC}"
        echo ""
        echo -e "${YELLOW}Attempting to run program...${NC}"
        echo ""
        
        run_output=$(timeout 15s "$TEST_PROG" 2>&1)
        run_exit=$?
        
        echo -e "${YELLOW}Program exit code: $run_exit${NC}"
        echo ""
        echo -e "${YELLOW}Program output:${NC}"
        echo "$run_output"
        echo ""
        
        if echo "$run_output" | grep -q "PASS"; then
            echo -e "${GREEN}✅ Program PASSED (contains 'PASS')${NC}"
        else
            echo -e "${RED}❌ Program output missing 'PASS'${NC}"
        fi
    else
        echo -e "${RED}❌ Executable NOT created at $TEST_PROG${NC}"
    fi
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo "Diagnostic test complete"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"

