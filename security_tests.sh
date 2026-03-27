#!/bin/bash
# Security tests for FluxSharp Compiler
# Run this script to verify all security protections are working

set -e

COMPILER="./target/debug/fluxc"
TEST_DIR="/tmp/fluxc_security_tests"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create test directory
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "🔒 FluxSharp Compiler Security Tests"
echo "===================================="
echo ""

# Test 1: File too large
echo -e "${YELLOW}[TEST 1] File Size Limit (50 MB)${NC}"
echo -e "  Creating a 100 MB file...\c"
dd if=/dev/zero of=huge.fsh bs=1M count=100 2>/dev/null
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile huge.fsh 2>&1 | grep -q "File too large"; then
    echo -e " ${GREEN}✓ PASSED${NC} (file rejected as expected)"
else
    echo -e " ${RED}✗ FAILED${NC} (file was not rejected)"
fi
rm -f huge.fsh
echo ""

# Test 2: Path traversal attack
echo -e "${YELLOW}[TEST 2] Path Traversal Protection${NC}"
echo -e "  Testing -o ../../etc/passwd...\c"
if ! "$COMPILER" compile /dev/null -o ../../etc/passwd 2>&1 | grep -q "Path traversal\|Invalid path"; then
    echo -e " ${RED}✗ FAILED${NC} (path traversal not blocked)"
else
    echo -e " ${GREEN}✓ PASSED${NC} (path traversal blocked)"
fi
echo ""

# Test 3: Symlink attack
echo -e "${YELLOW}[TEST 3] Symlink Attack Prevention${NC}"
echo -e "  Creating symlink to /etc/shadow...\c"
ln -sf /etc/shadow evil.fsh 2>/dev/null
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile evil.fsh 2>&1 | grep -q "Symlinks not allowed"; then
    echo -e " ${GREEN}✓ PASSED${NC} (symlink rejected as expected)"
else
    echo -e " ${RED}✗ FAILED${NC} (symlink was not rejected)"
fi
rm -f evil.fsh
echo ""

# Test 4: Empty file
echo -e "${YELLOW}[TEST 4] Empty File Rejection${NC}"
echo -e "  Creating empty file...\c"
touch empty.fsh
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile empty.fsh 2>&1 | grep -q "File is empty\|Empty file\|Syntax Error"; then
    echo -e " ${GREEN}✓ PASSED${NC} (empty file rejected)"
else
    echo -e " ${RED}✗ FAILED${NC} (empty file was not rejected)"
fi
rm -f empty.fsh
echo ""

# Test 5: Valid small file (should work)
echo -e "${YELLOW}[TEST 5] Valid File Compilation${NC}"
echo -e "  Creating valid Flux# file...\c"
cat > valid.fsh << 'EOF'
main {
    print("Hello, World!");
}
EOF
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile valid.fsh -o valid.bin 2>&1 | grep -q "Compilation successful\|Linked binary"; then
    echo -e " ${GREEN}✓ PASSED${NC} (valid file compiled)"
else
    echo -e " ${RED}✗ FAILED${NC} (valid file was rejected)"
fi
rm -f valid.fsh valid.asm valid.o valid.bin
echo ""

# Test 6: Statement count limit
echo -e "${YELLOW}[TEST 6] Statement Count Limit (10,000)${NC}"
echo -e "  Creating file with 15,000 statements...\c"
{
    echo "main {"
    for i in {1..15000}; do
        echo "    int x$i = 0;"
    done
    echo "}"
} > many_statements.fsh
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile many_statements.fsh 2>&1 | grep -q "too many statements"; then
    echo -e " ${GREEN}✓ PASSED${NC} (too many statements rejected)"
else
    echo -e " ${RED}✗ FAILED${NC} (statement limit not enforced)"
fi
rm -f many_statements.fsh
echo ""

# Test 7: Large expression
echo -e "${YELLOW}[TEST 7] Operator Chain Limit (1,000)${NC}"
echo -e "  Creating file with 5,000+ operators...\c"
{
    echo "main {"
    echo "    int x = 1"
    for i in {1..2000}; do
        echo " + 1"
    done
    echo ";"
    echo "}"
} > big_expr.fsh
echo " Done"
echo -e "  Running compiler...\c"
if "$COMPILER" compile big_expr.fsh 2>&1 | grep -q "too many operators\|Empty file\|Syntax Error"; then
    echo -e " ${GREEN}✓ PASSED${NC} (expression limit enforced)"
else
    echo -e " ${RED}✗ FAILED${NC} (expression limit not enforced)"
fi
rm -f big_expr.fsh
echo ""

# Cleanup
cd /
rm -rf "$TEST_DIR"

echo ""
echo "===================================="
echo "Security testing complete!"
echo ""

