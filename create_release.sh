#!/bin/bash
################################################################################
# FluxSharp Release Packager
# Creates distributable .tar.gz packages for all platforms
################################################################################

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_DIR="$PROJECT_DIR/release_package"
VERSION="1.0.1"
BUILD_DATE=$(date +%Y%m%d)

echo "📦 FluxSharp Release Packager v$VERSION"
echo "════════════════════════════════════════"
echo ""

# ============================================================================
# Verify Release Structure
# ============================================================================

echo "🔍 Verifying release structure..."

required_files=(
    "bin/fluxc"
    "lib/runtime.o"
    "include/core.fsh"
    "build.sh"
    "README.md"
    "INSTALL.md"
    "docs/LANGUAGE.md"
    "docs/SECURITY.md"
    "examples/hello.fsh"
    "examples/calculator.fsh"
    "examples/arrays.fsh"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$RELEASE_DIR/$file" ]; then
        echo "❌ Missing: $RELEASE_DIR/$file"
        exit 1
    fi
    echo "  ✅ $file"
done

echo "✅ All required files present"
echo ""

# ============================================================================
# Create Distribution Packages
# ============================================================================

echo "📦 Creating distribution packages..."
echo ""

# Get system info
SYSTEM=$(uname -s)
ARCH=$(uname -m)
KERNEL=$(uname -r)

# Create package directory
PKG_NAME="fluxsharp-v${VERSION}-linux-x64"
PKG_DIR="/tmp/$PKG_NAME"
rm -rf "$PKG_DIR"
cp -r "$RELEASE_DIR" "$PKG_DIR"

echo "📋 Package Contents:"
tree -L 2 "$PKG_DIR" 2>/dev/null || find "$PKG_DIR" -type f | head -20

echo ""

# ============================================================================
# Create Archives
# ============================================================================

echo "📦 Creating archives..."

cd /tmp

# Create .tar.gz
echo "  Creating .tar.gz..."
tar -czf "${PKG_NAME}.tar.gz" "$PKG_NAME"
tar_size=$(du -h "${PKG_NAME}.tar.gz" | cut -f1)
echo "  ✅ ${PKG_NAME}.tar.gz ($tar_size)"

# Create .zip (if zip available)
if command -v zip >/dev/null 2>&1; then
    echo "  Creating .zip..."
    zip -r -q "${PKG_NAME}.zip" "$PKG_NAME"
    zip_size=$(du -h "${PKG_NAME}.zip" | cut -f1)
    echo "  ✅ ${PKG_NAME}.zip ($zip_size)"
fi

echo ""

# ============================================================================
# Create Release Notes
# ============================================================================

echo "📝 Generating release notes..."

cat > "${PKG_NAME}-RELEASE.md" << 'EOF'
# FluxSharp v0.1.0 Release Notes

## What's Included

This release contains a complete, standalone FluxSharp compiler and runtime with no external dependencies.

### Package Contents

```
fluxsharp-v0.1.0-linux-x64/
├── bin/
│   └── fluxc                    # Pre-compiled compiler binary
├── lib/
│   └── runtime.o                # Pre-compiled runtime library
├── include/
│   └── core.fsh                 # Core library definitions
├── examples/
│   ├── hello.fsh                # Hello world example
│   ├── calculator.fsh           # Calculator class example
│   └── arrays.fsh               # Array operations example
├── docs/
│   ├── LANGUAGE.md              # Complete language reference
│   └── SECURITY.md              # Security features
├── build.sh                     # Build script (no cargo required!)
├── README.md                    # Quick start guide
└── INSTALL.md                   # Installation instructions
```

## Installation

```bash
tar -xzf fluxsharp-v0.1.0-linux-x64.tar.gz
cd fluxsharp-v0.1.0-linux-x64
sudo apt install nasm binutils    # Install dependencies
./build.sh examples/hello.fsh      # Run example
```

## Key Features

✅ **Security First**
- Bounds checking on all array accesses
- Null pointer safety
- Integer overflow detection
- Type safety enforcement

⚡ **High Performance**
- Compiles to native x86-64 code
- Zero-cost abstractions
- No runtime overhead
- Direct hardware access

🛠️ **Easy to Use**
- Simple, familiar syntax
- No external dependencies after build
- Single shell script to compile
- Minimal runtime footprint

## System Requirements

- Linux x86-64
- nasm (assembler)
- binutils (linker)

## No Rust Required

Unlike the development version, this release includes pre-compiled binaries. You do NOT need to install Rust!

## Next Steps

1. Extract the archive
2. Install dependencies (nasm, binutils)
3. Run `./build.sh examples/hello.fsh`
4. Check `docs/LANGUAGE.md` for language features

## For Integration with FluxGridOS

```bash
# Compile to object file
./bin/fluxc compile user_program.fsh -o program.o

# Link with kernel
ld -r kernel.o program.o -o combined.o
```

---

**Release Date**: 2024
**License**: MIT
**Status**: Production Ready ✅
EOF

echo "✅ Release notes generated"
echo ""

# ============================================================================
# Summary
# ============================================================================

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

