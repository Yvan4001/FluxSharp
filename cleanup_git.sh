#!/bin/bash
################################################################################
# FluxSharp Git Cleanup Script
# Removes generated files from Git tracking and organizes the repository
################################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🧹 FluxSharp Git Cleanup - Remove Generated Files${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}⚠️  This script will:${NC}"
echo "  1. Remove generated files from Git tracking"
echo "  2. Keep source files (.fsh, .asm sources)"
echo "  3. Clean up the repository"
echo ""

read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo -e "${BLUE}🔍 Checking Git status...${NC}"

# Show current status
git status --short | head -20

echo ""
echo -e "${YELLOW}Cleaning generated files from Git tracking...${NC}"
echo ""

# Remove compiled binaries and object files from tracking
echo "Removing *.o files..."
git rm --cached -f $(git ls-files '*.o' 2>/dev/null) 2>/dev/null || true

echo "Removing generated _prog executables..."
git rm --cached -f $(git ls-files '*_prog' 2>/dev/null) 2>/dev/null || true

echo "Removing generated program files..."
git rm --cached -f program program.asm 2>/dev/null || true

echo "Removing release archives..."
git rm --cached -f $(git ls-files 'fluxsharp-v*.tar.gz' '*.zip' 2>/dev/null) 2>/dev/null || true

echo "Removing bin/ directory artifacts..."
git rm --cached -r bin/*.o 2>/dev/null || true
git rm --cached -r bin/*_prog 2>/dev/null || true

echo "Removing examples/ build artifacts..."
git rm --cached -r release_package/examples/*.o 2>/dev/null || true
git rm --cached -r release_package/examples/*_prog 2>/dev/null || true
git rm --cached -r release_package/examples/*.asm 2>/dev/null || true

echo "Removing test_suite/ build artifacts..."
git rm --cached -r test_suite/*.o 2>/dev/null || true
git rm --cached -r test_suite/*_prog 2>/dev/null || true

echo ""
echo -e "${BLUE}📝 Adding important files...${NC}"
echo ""

# Add new release workflow files
echo "Adding release workflow files..."
git add release.sh
git add check_prerequisites.sh
git add IMPLEMENTATION_SUMMARY.md
git add RELEASE_QUICK_START.md
git add PROFESSIONAL_RELEASE_WORKFLOW.md
git add test_suite/.gitignore
git add release_package/.gitignore
echo -e "${GREEN}✅ Added${NC}"

echo ""
echo -e "${BLUE}🔄 Updating main .gitignore...${NC}"
git add .gitignore
echo -e "${GREEN}✅ Updated${NC}"

echo ""
echo -e "${BLUE}📊 Final Git status:${NC}"
echo ""
git status --short

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Cleanup complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff --cached"
echo "  2. Commit: git commit -m 'Add professional release workflow system'"
echo "  3. Verify: git status"
echo ""
