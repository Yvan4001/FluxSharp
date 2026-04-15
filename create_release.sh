#!/bin/bash
################################################################################
# FluxSharp Release Package Creator - Professional Version
# ============================================
# Builds a complete, production-ready release package for distribution.
# This package requires NO external build tools (Rust/Cargo).
#
# Usage:
#   ./create_release.sh [version] [output_dir]
#
# Examples:
#   ./create_release.sh                    # Uses version from Cargo.toml
#   ./create_release.sh 0.2.0              # Specific version
#   ./create_release.sh 0.2.0 ~/releases   # Custom output directory
################################################################################

set -e

# ============================================================================
# Configuration & Setup
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
COMPILER_DIR="$PROJECT_ROOT/flux_compiler"
FLUXC_SOURCE_DIR="$COMPILER_DIR/fluxc"
RUNTIME_DIR="$FLUXC_SOURCE_DIR/runtime"

# Extract version from Cargo.toml or use provided version
VERSION="${1:$(grep '^version' "$COMPILER_DIR/Cargo.toml" 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)".*/\1/')}"
VERSION="${VERSION:-0.1.0}"
OUTPUT_DIR="${2:-.}"
RELEASE_NAME="fluxsharp-v${VERSION}-linux-x64"
RELEASE_PATH="$OUTPUT_DIR/$RELEASE_NAME"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  🚀 FluxSharp Release Package Creator v${VERSION}${BLUE}                  ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_section() {
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ Error: $1${NC}"
    exit 1
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warn() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

step() {
    echo ""
    echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $1"
}

# ============================================================================
# Validation
# ============================================================================

validate_environment() {
    print_section "🔍 Environment Validation"
    
    local errors=0
    
    if ! command -v cargo &>/dev/null; then
        print_warn "cargo not found (needed to build compiler)"
        errors=$((errors + 1))
    else
        print_success "cargo found"
    fi
    
    if ! command -v nasm &>/dev/null; then
        print_warn "nasm not found (needed for runtime)"
        errors=$((errors + 1))
    else
        print_success "nasm found"
    fi
    
    if ! command -v tar &>/dev/null; then
        print_error "tar is required"
    else
        print_success "tar found"
    fi
    
    if ! command -v zip &>/dev/null; then
        print_warn "zip not found (zip archive won't be created)"
    else
        print_success "zip found"
    fi
    
    if [ ! -d "$COMPILER_DIR" ]; then
        print_error "Compiler directory not found: $COMPILER_DIR"
    fi
    
    if [ ! -d "$RUNTIME_DIR" ]; then
        print_error "Runtime directory not found: $RUNTIME_DIR"
    fi
    
    print_success "Environment validation passed"
    
    if [ $errors -gt 0 ]; then
        print_warn "Some optional tools are missing, but continuing..."
    fi
}

# ============================================================================
# Build Process
# ============================================================================

build_compiler() {
    print_section "📦 Step 1: Building Rust Compiler"
    
    step "Building fluxc with cargo release optimization..."
    (cd "$COMPILER_DIR" && cargo build --release 2>&1) || {
        print_error "Failed to build compiler"
    }
    
    FLUXC_BIN="$COMPILER_DIR/target/release/fluxc"
    if [ ! -f "$FLUXC_BIN" ]; then
        print_error "Compiler binary not found"
    fi
    
    FLUXC_SIZE=$(du -h "$FLUXC_BIN" | cut -f1)
    print_success "Compiler built: $FLUXC_SIZE"
}

build_runtime() {
    print_section "🔨 Step 2: Building Runtime Object Files"
    
    if [ ! -f "$RUNTIME_DIR/runtime.asm" ]; then
        print_error "Runtime assembly not found"
    fi
    
    step "Generating runtime_lib.o..."
    nasm -f elf64 "$RUNTIME_DIR/runtime.asm" -o "$RUNTIME_DIR/runtime_lib.o" 2>/dev/null || {
        print_error "Failed to assemble runtime"
    }
    
    if [ ! -f "$RUNTIME_DIR/runtime_lib.o" ]; then
        print_error "runtime_lib.o not created"
    fi
    
    RUNTIME_SIZE=$(du -h "$RUNTIME_DIR/runtime_lib.o" | cut -f1)
    print_success "runtime_lib.o generated: $RUNTIME_SIZE"
}

prepare_release_dir() {
    print_section "📁 Step 3: Preparing Release Directory"
    
    step "Creating directory structure..."
    mkdir -p "$RELEASE_PATH"/{bin,lib,include,examples,docs}
    
    step "Copying compiler binary..."
    cp "$FLUXC_BIN" "$RELEASE_PATH/bin/fluxc"
    chmod +x "$RELEASE_PATH/bin/fluxc"
    print_success "Compiler binary copied"
    
    step "Copying runtime object files..."
    cp "$RUNTIME_DIR/runtime_lib.o" "$RELEASE_PATH/lib/"
    cp "$RUNTIME_DIR/runtime.o" "$RELEASE_PATH/lib/" 2>/dev/null || true
    print_success "Runtime object files copied"
    
    step "Copying examples..."
    for example in examples/*.fsh; do
        if [ -f "$example" ]; then
            cp "$example" "$RELEASE_PATH/examples/" 2>/dev/null || true
        fi
    done
    print_success "Examples copied"
    
    step "Copying documentation..."
    if [ -d "docs" ]; then
        cp -r docs/* "$RELEASE_PATH/docs/" 2>/dev/null || true
        rm -f "$RELEASE_PATH/docs/"*.asm 2>/dev/null || true
    fi
    print_success "Documentation copied"
}

create_release_build_script() {
    print_section "🛠️  Step 4: Creating Release Build Script"
    
    step "Generating build.sh..."
    cp release_package/build.sh "$RELEASE_PATH/build.sh"
    chmod +x "$RELEASE_PATH/build.sh"
    print_success "Release build script ready"
}

create_documentation() {
    print_section "📚 Step 5: Creating Documentation"
    
    step "Copying documentation files..."
    if [ -f "release_package/README.md" ]; then
        cp "release_package/README.md" "$RELEASE_PATH/"
    fi
    if [ -f "release_package/INSTALL.md" ]; then
        cp "release_package/INSTALL.md" "$RELEASE_PATH/"
    fi
    print_success "Documentation files ready"
}

create_archives() {
    print_section "📦 Step 6: Creating Distribution Archives"
    
    cd "$OUTPUT_DIR"
    
    step "Creating tar.gz archive..."
    tar -czf "${RELEASE_NAME}.tar.gz" "$RELEASE_NAME" 2>/dev/null || {
        print_error "Failed to create tar.gz"
    }
    
    TAR_SIZE=$(du -h "${RELEASE_NAME}.tar.gz" | cut -f1)
    print_success "Archive created: $TAR_SIZE"
    
    if command -v zip &>/dev/null; then
        step "Creating zip archive..."
        zip -r -q "${RELEASE_NAME}.zip" "$RELEASE_NAME" 2>/dev/null || true
        
        if [ -f "${RELEASE_NAME}.zip" ]; then
            ZIP_SIZE=$(du -h "${RELEASE_NAME}.zip" | cut -f1)
            print_success "Archive created: $ZIP_SIZE"
        fi
    fi
    
    cd - > /dev/null
}

generate_checksums() {
    print_section "🔐 Step 7: Generating Checksums"
    
    cd "$OUTPUT_DIR"
    
    step "Generating SHA256 checksums..."
    CHECKSUM_FILE="${RELEASE_NAME}-CHECKSUMS.txt"
    > "$CHECKSUM_FILE"
    
    [ -f "${RELEASE_NAME}.tar.gz" ] && sha256sum "${RELEASE_NAME}.tar.gz" >> "$CHECKSUM_FILE"
    [ -f "${RELEASE_NAME}.zip" ] && sha256sum "${RELEASE_NAME}.zip" >> "$CHECKSUM_FILE"
    
    print_success "Checksums generated"
    
    cd - > /dev/null
}

generate_summary() {
    print_section "📊 Release Summary"
    
    echo ""
    echo -e "${GREEN}✅ Release created successfully!${NC}"
    echo ""
    echo -e "${BLUE}Version:${NC} $VERSION"
    echo -e "${BLUE}Location:${NC} $OUTPUT_DIR/$RELEASE_NAME"
    echo -e "${BLUE}Size:${NC} $(du -sh "$RELEASE_PATH" | cut -f1)"
    echo ""
    echo -e "${BLUE}Files:${NC}"
    [ -f "$OUTPUT_DIR/${RELEASE_NAME}.tar.gz" ] && echo "  📦 $(du -h "$OUTPUT_DIR/${RELEASE_NAME}.tar.gz" | cut -f1) - ${RELEASE_NAME}.tar.gz"
    [ -f "$OUTPUT_DIR/${RELEASE_NAME}.zip" ] && echo "  📦 $(du -h "$OUTPUT_DIR/${RELEASE_NAME}.zip" | cut -f1) - ${RELEASE_NAME}.zip"
    [ -f "$OUTPUT_DIR/${RELEASE_NAME}-CHECKSUMS.txt" ] && echo "  📄 ${RELEASE_NAME}-CHECKSUMS.txt"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  tar -xzf ${RELEASE_NAME}.tar.gz"
    echo "  cd ${RELEASE_NAME}"
    echo "  ./build.sh examples/hello.fsh"
    echo ""
}

error_handler() {
    print_error "Build failed at line $1"
}

trap 'error_handler ${LINENO}' ERR

# ============================================================================
# Main
# ============================================================================

main() {
    print_header
    echo -e "${BLUE}Creating release package v${VERSION}...${NC}"
    echo ""
    
    validate_environment
    build_compiler
    build_runtime
    prepare_release_dir
    create_release_build_script
    create_documentation
    create_archives
    generate_checksums
    generate_summary
    
    echo -e "${GREEN}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}🎉 Release package creation completed!${NC}"
    echo -e "${GREEN}════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

main "$@"

echo "✅ Packaging Complete!"
echo ""
echo "📦 Packages created in: /tmp/"
echo ""
echo "Files:"
ls -lh /tmp/${PKG_NAME}* | awk '{print "  " $9 " (" $5 ")"}'
echo ""
echo "🚀 To distribute:"
echo "  cp /tmp/${PKG_NAME}.tar.gz /path/to/release/"
echo "  cp /tmp/${PKG_NAME}-RELEASE.md /path/to/release/"
echo ""
echo "✨ Users can now install with:"
echo "  tar -xzf ${PKG_NAME}.tar.gz"
echo "  cd ${PKG_NAME}"
echo "  ./build.sh examples/hello.fsh"
echo ""

