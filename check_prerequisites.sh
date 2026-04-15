#!/bin/bash
################################################################################
# FluxSharp Release Prerequisites Checker
# Verifies all tools needed for building releases are installed
################################################################################

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  🔍 FluxSharp Release Prerequisites Checker${BLUE}                   ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_check() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅${NC} $2"
        return 0
    else
        echo -e "${RED}❌${NC} $2"
        return 1
    fi
}

print_warn() {
    echo -e "${YELLOW}⚠️ ${NC}$1"
}

print_info() {
    echo -e "${BLUE}ℹ️ ${NC}$1"
}

check_tool() {
    local tool=$1
    local install_cmd=$2
    
    if command -v "$tool" &>/dev/null; then
        local version=$("$tool" --version 2>&1 | head -1 || echo "installed")
        print_check 0 "$tool: $version"
        return 0
    else
        print_check 1 "$tool: not found"
        if [ -n "$install_cmd" ]; then
            print_info "Install with: $install_cmd"
        fi
        return 1
    fi
}

check_file() {
    local file=$1
    local description=$2
    
    if [ -f "$file" ]; then
        print_check 0 "$description ($file)"
        return 0
    else
        print_check 1 "$description ($file)"
        return 1
    fi
}

check_directory() {
    local dir=$1
    local description=$2
    
    if [ -d "$dir" ]; then
        print_check 0 "$description ($dir)"
        return 0
    else
        print_check 1 "$description ($dir)"
        return 1
    fi
}

main() {
    print_header
    
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    REQUIRED_ERRORS=0
    OPTIONAL_WARNINGS=0
    
    # ========================================================================
    # REQUIRED TOOLS
    # ========================================================================
    
    echo -e "${BLUE}📋 REQUIRED TOOLS (for building releases)${NC}"
    echo ""
    
    check_tool "cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" || ((REQUIRED_ERRORS++))
    check_tool "rustc" "" || ((REQUIRED_ERRORS++))
    check_tool "nasm" "sudo apt install nasm" || ((REQUIRED_ERRORS++))
    check_tool "ld" "sudo apt install binutils" || ((REQUIRED_ERRORS++))
    check_tool "tar" "" || ((REQUIRED_ERRORS++))
    
    echo ""
    
    # ========================================================================
    # OPTIONAL TOOLS
    # ========================================================================
    
    echo -e "${BLUE}🎁 OPTIONAL TOOLS (for additional features)${NC}"
    echo ""
    
    check_tool "zip" "sudo apt install zip" || ((OPTIONAL_WARNINGS++))
    check_tool "sha256sum" "sudo apt install coreutils" || ((OPTIONAL_WARNINGS++))
    check_tool "gh" "https://github.com/cli/cli/blob/trunk/docs/install.md" || ((OPTIONAL_WARNINGS++))
    
    echo ""
    
    # ========================================================================
    # PROJECT STRUCTURE
    # ========================================================================
    
    echo -e "${BLUE}📁 PROJECT STRUCTURE${NC}"
    echo ""
    
    check_directory "$SCRIPT_DIR/flux_compiler" "Compiler source" || ((REQUIRED_ERRORS++))
    check_directory "$SCRIPT_DIR/flux_compiler/fluxc" "Compiler package" || ((REQUIRED_ERRORS++))
    check_directory "$SCRIPT_DIR/flux_compiler/fluxc/runtime" "Runtime source" || ((REQUIRED_ERRORS++))
    check_directory "$SCRIPT_DIR/release_package" "Release template" || ((REQUIRED_ERRORS++))
    check_directory "$SCRIPT_DIR/docs" "Documentation" || ((REQUIRED_ERRORS++))
    
    echo ""
    
    # ========================================================================
    # KEY FILES
    # ========================================================================
    
    echo -e "${BLUE}📄 KEY FILES${NC}"
    echo ""
    
    check_file "$SCRIPT_DIR/flux_compiler/Cargo.toml" "Cargo workspace" || ((REQUIRED_ERRORS++))
    check_file "$SCRIPT_DIR/flux_compiler/fluxc/Cargo.toml" "Compiler Cargo.toml" || ((REQUIRED_ERRORS++))
    check_file "$SCRIPT_DIR/flux_compiler/fluxc/runtime/runtime.asm" "Runtime assembly" || ((REQUIRED_ERRORS++))
    check_file "$SCRIPT_DIR/create_release.sh" "Release creation script" || ((REQUIRED_ERRORS++))
    check_file "$SCRIPT_DIR/.gitignore" "Git ignore file" || ((REQUIRED_ERRORS++))
    
    echo ""
    
    # ========================================================================
    # DISK SPACE
    # ========================================================================
    
    echo -e "${BLUE}💾 DISK SPACE${NC}"
    echo ""
    
    AVAILABLE=$(df "$SCRIPT_DIR" | tail -1 | awk '{print $4}')
    REQUIRED_MB=$((2000))  # 2 GB should be enough
    AVAILABLE_MB=$((AVAILABLE / 1024))
    
    if [ $AVAILABLE_MB -gt $REQUIRED_MB ]; then
        print_check 0 "Available space: ${AVAILABLE_MB} MB (need ~${REQUIRED_MB} MB)"
    else
        print_check 1 "Available space: ${AVAILABLE_MB} MB (need ~${REQUIRED_MB} MB)"
        ((REQUIRED_ERRORS++))
    fi
    
    echo ""
    
    # ========================================================================
    # SUMMARY
    # ========================================================================
    
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
    
    if [ $REQUIRED_ERRORS -eq 0 ] && [ $OPTIONAL_WARNINGS -eq 0 ]; then
        echo -e "${GREEN}✅ All checks passed! You're ready to build releases!${NC}"
        echo ""
        echo "Next steps:"
        echo "  1. cd $SCRIPT_DIR"
        echo "  2. ./create_release.sh"
        return 0
    elif [ $REQUIRED_ERRORS -eq 0 ]; then
        echo -e "${YELLOW}⚠️  All required tools found, but some optional tools are missing.${NC}"
        echo ""
        echo "You can still build releases, but some features may be limited."
        echo ""
        echo "To fix warnings:"
        echo "  - Install missing tools listed above"
        echo ""
        echo "Next steps:"
        echo "  1. cd $SCRIPT_DIR"
        echo "  2. ./create_release.sh"
        return 0
    else
        echo -e "${RED}❌ Missing required tools or files!${NC}"
        echo ""
        echo "Please install missing tools listed above."
        echo ""
        echo "Quick fix for Debian/Ubuntu:"
        echo "  sudo apt update"
        echo "  sudo apt install nasm binutils"
        echo ""
        echo "Then install Rust:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        return 1
    fi
}

main "$@"
