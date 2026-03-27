#!/bin/bash
# Final Cleanup - Move redundant files to .archive/

PROJECT_DIR="/run/media/yvan/E6EAD2EBEAD2B6D1/Projet_Dev/FluxSharp"
ARCHIVE_DIR="$PROJECT_DIR/.archive"

echo "🧹 Final cleanup: archiving redundant files..."
echo ""

# Create archive if it doesn't exist
mkdir -p "$ARCHIVE_DIR"

# List of files to keep in root
KEEP_FILES=(
    "main.fsh"
    "Makefile"
    "README.md"
    "SETUP_COMPLETE.md"
    "Cargo.toml"
    "LICENSE"
    ".git"
    ".gitignore"
    ".idea"
)

echo "📋 Checking files in root directory..."
echo ""

# Archive files that are NOT in KEEP_FILES list
for file in "$PROJECT_DIR"/*; do
    filename=$(basename "$file")
    
    # Check if file should be kept
    should_keep=0
    for keep_file in "${KEEP_FILES[@]}"; do
        if [ "$filename" = "$keep_file" ]; then
            should_keep=1
            break
        fi
    done
    
    # Archive if not in keep list and not a directory (except .archive)
    if [ $should_keep -eq 0 ] && [ "$filename" != ".archive" ]; then
        if [ -f "$file" ]; then
            mv "$file" "$ARCHIVE_DIR/" 2>/dev/null && echo "✓ Archived: $filename"
        fi
    fi
done

echo ""
echo "📦 Archiving old documentation from docs/..."

# Old docs to archive
OLD_DOCS=(
    "ASYNC-AWAIT-IMPLEMENTATION.md"
    "ASYNC-AWAIT-REFERENCE.md"
    "ASYNC-AWAIT.md"
    "EXAMPLES.md"
    "FEATURES.md"
    "FUNCTIONS-REFERENCE.md"
    "LANGUAGE_GUIDE.md"
    "QUICKSTART.md"
    "SECURITY.md"
    "README.md"
)

for doc in "${OLD_DOCS[@]}"; do
    if [ -f "$PROJECT_DIR/docs/$doc" ]; then
        mkdir -p "$ARCHIVE_DIR/docs"
        mv "$PROJECT_DIR/docs/$doc" "$ARCHIVE_DIR/docs/" 2>/dev/null && echo "✓ Archived: docs/$doc"
    fi
done

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "📂 Root directory now contains:"
ls -1 "$PROJECT_DIR" | grep -v "^\." | sort
echo ""
echo "📦 Old files archived in: .archive/"
echo ""

