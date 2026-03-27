#!/bin/bash
# FluxSharp Project Cleanup Script
# Removes redundant files and organizes the project structure

set -e

PROJECT_DIR="/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp"
ARCHIVE_DIR="$PROJECT_DIR/.archive"

echo "🧹 FluxSharp Project Cleanup Starting..."
echo ""

# Create archive directory
mkdir -p "$ARCHIVE_DIR"
echo "📁 Created archive directory: .archive/"

# Files to archive (redundant documentation)
ARCHIVE_FILES=(
    "ASYNC-ASSEMBLY-QUICK-REFERENCE.asm"
    "ASYNC-AWAIT-FILES-SUMMARY.md"
    "ASYNC-RUNTIME-CLEAN-FINAL.md"
    "ASYNC-RUNTIME-COMPLETION-REPORT.md"
    "CORRECTION-SYNTAXE-ASSEMBLEUR-RESUME.md"
    "DIRECTORY-STRUCTURE.md"
    "IMPLEMENTATION-VERIFICATION-REPORT.txt"
    "INDEX.md"
    "MATH-FUNCTIONS-STATUS.md"
    "PROJECT-CLEANUP-PLAN.md"
    "PROJECT-INDEX.md"
    "PROJECT-STATUS.md"
    "README-USAGE.md"
    "REPOSITORY-STRUCTURE.md"
    "COMPLETION-REPORT.md"
    "test_async_runtime.sh"
    "validate_async.sh"
)

echo "📦 Archiving redundant files..."
for file in "${ARCHIVE_FILES[@]}"; do
    if [ -f "$PROJECT_DIR/$file" ]; then
        mv "$PROJECT_DIR/$file" "$ARCHIVE_DIR/"
        echo "  ✓ Archived: $file"
    fi
done

echo ""
echo "✅ Cleanup Complete!"
echo ""
echo "Root directory now contains:"
echo "  ✓ Cargo.toml"
echo "  ✓ Makefile"
echo "  ✓ README.md (English)"
echo "  ✓ main.fsh"
echo "  ✓ main.asm"
echo "  ✓ program (executable)"
echo "  ✓ src/"
echo "  ✓ docs/"
echo "  ✓ examples/"
echo "  ✓ .archive/ (old files)"
echo ""

