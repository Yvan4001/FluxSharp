#!/bin/bash
################################################################################
# FluxSharp Release Workflow Manager
# Professional release workflow with Git tags, semantic versioning, and validation
#
# Usage:
#   ./release.sh init              Initialize versioning system
#   ./release.sh prepare <version> Prepare release (commit version bumps)
#   ./release.sh build <version>   Build release package
#   ./release.sh publish <version> Create Git tag and push
#   ./release.sh complete <version> Full workflow (prepare → build → publish)
#   ./release.sh list              List all releases
#   ./release.sh current           Show current version
#
# Examples:
#   ./release.sh complete 0.2.0
#   ./release.sh prepare 1.0.0
#   ./release.sh build 0.2.0
#   ./release.sh publish 0.2.0 --push
################################################################################

set -e

# ============================================================================
# Configuration
# ============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
COMPILER_DIR="$PROJECT_ROOT/flux_compiler"
RELEASES_DIR="$PROJECT_ROOT/releases"
VERSION_FILE="$PROJECT_ROOT/.toolversions"
CHANGELOG_FILE="$PROJECT_ROOT/CHANGELOG.md"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  📦 FluxSharp Release Workflow Manager${BLUE}                     ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_section() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${MAGENTA}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
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
    echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $1"
}

# ============================================================================
# Version Validation
# ============================================================================

validate_semantic_version() {
    local version=$1
    
    # Check if version matches semantic versioning (X.Y.Z)
    if [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        return 0
    else
        return 1
    fi
}

version_exists() {
    local version=$1
    
    if git rev-parse "v${version}" >/dev/null 2>&1; then
        return 0  # Tag exists
    else
        return 1  # Tag doesn't exist
    fi
}

# ============================================================================
# Version Management
# ============================================================================

get_current_version() {
    if [ -f "$VERSION_FILE" ]; then
        grep "^flux" "$VERSION_FILE" | sed 's/flux=//'
    else
        echo "0.1.0"
    fi
}

set_current_version() {
    local version=$1
    
    if [ ! -f "$VERSION_FILE" ]; then
        echo "# FluxSharp Version Management" > "$VERSION_FILE"
    fi
    
    # Update or add version
    if grep -q "^flux=" "$VERSION_FILE"; then
        sed -i "s/^flux=.*/flux=${version}/" "$VERSION_FILE"
    else
        echo "flux=${version}" >> "$VERSION_FILE"
    fi
}

# ============================================================================
# Git Utilities
# ============================================================================

check_git_clean() {
    if ! git diff --quiet; then
        print_error "Working directory has uncommitted changes. Please commit or stash first."
    fi
}

check_git_repo() {
    if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        print_error "Not inside a Git repository"
    fi
}

# ============================================================================
# Command: Init
# ============================================================================

cmd_init() {
    print_section "🚀 Initialize Release System"
    
    check_git_repo
    
    step "Creating releases directory..."
    mkdir -p "$RELEASES_DIR"
    print_success "Directory created: $RELEASES_DIR"
    
    step "Creating version file..."
    set_current_version "0.1.0"
    print_success "Version file created: $VERSION_FILE"
    
    step "Initializing Git tags..."
    
    # Create initial tag if it doesn't exist
    if ! git rev-parse v0.1.0 >/dev/null 2>&1; then
        git tag -a v0.1.0 -m "Initial release v0.1.0" HEAD
        print_success "Tag v0.1.0 created"
    else
        print_info "Tag v0.1.0 already exists"
    fi
    
    echo ""
    print_success "Release system initialized!"
    echo ""
    echo "Next steps:"
    echo "  ./release.sh current      # Show current version"
    echo "  ./release.sh list         # List all releases"
    echo "  ./release.sh prepare 0.2.0  # Prepare next release"
}

# ============================================================================
# Command: Current
# ============================================================================

cmd_current() {
    print_section "📌 Current Version"
    
    local version=$(get_current_version)
    
    echo ""
    echo -e "${BLUE}Version:${NC}  $version"
    
    if git rev-parse "v${version}" >/dev/null 2>&1; then
        echo -e "${GREEN}Tag:${NC}      v${version} (exists)"
        
        local tag_date=$(git log -1 --format=%ai "v${version}")
        echo -e "${BLUE}Tagged:${NC}    $tag_date"
    else
        echo -e "${YELLOW}Tag:${NC}      v${version} (not yet tagged)"
    fi
    
    echo ""
}

# ============================================================================
# Command: List
# ============================================================================

cmd_list() {
    print_section "📋 All Releases"
    
    check_git_repo
    
    echo ""
    
    # List all version tags
    git tag -l 'v*' --sort=-version:refname --format='%(refname:short)' | while read tag; do
        version=${tag#v}
        
        # Get tag info
        local tag_date=$(git log -1 --format=%ai "$tag" 2>/dev/null || echo "Unknown")
        local tag_msg=$(git tag -l "$tag" --format='%(subject)' 2>/dev/null || echo "")
        
        # Check if release package exists
        local pkg_exists=""
        if [ -d "$RELEASES_DIR/fluxsharp-v${version}-linux-x64" ]; then
            pkg_exists="📦 Package exists"
        fi
        
        echo -e "${GREEN}v${version}${NC}  ${tag_date:0:10}  $pkg_exists"
        if [ -n "$tag_msg" ]; then
            echo "     └─ $tag_msg"
        fi
    done
    
    echo ""
}

# ============================================================================
# Command: Prepare
# ============================================================================

cmd_prepare() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./release.sh prepare <version>"
    fi
    
    if ! validate_semantic_version "$version"; then
        print_error "Invalid version format: $version (expected X.Y.Z)"
    fi
    
    print_section "📋 Prepare Release v${version}"
    
    check_git_repo
    check_git_clean
    
    if version_exists "$version"; then
        print_error "Tag v${version} already exists. Version has been released."
    fi
    
    echo ""
    
    # Update Cargo.toml versions
    step "Updating version numbers..."
    
    # Update compiler version
    if [ -f "$COMPILER_DIR/Cargo.toml" ]; then
        sed -i "s/^version = \"[^\"]*\"/version = \"${version}\"/" "$COMPILER_DIR/Cargo.toml"
        print_success "Updated $COMPILER_DIR/Cargo.toml to v${version}"
    fi
    
    # Update flux_compiler/fluxc/Cargo.toml if it exists
    if [ -f "$COMPILER_DIR/fluxc/Cargo.toml" ]; then
        sed -i "s/^version = \"[^\"]*\"/version = \"${version}\"/" "$COMPILER_DIR/fluxc/Cargo.toml"
        print_success "Updated $COMPILER_DIR/fluxc/Cargo.toml to v${version}"
    fi
    
    # Update version file
    step "Updating version tracking file..."
    set_current_version "$version"
    print_success "Version tracking updated"
    
    # Create release notes if they don't exist
    step "Creating release notes template..."
    local release_notes="$PROJECT_ROOT/RELEASE_NOTES_v${version}.md"
    
    if [ ! -f "$release_notes" ]; then
        cat > "$release_notes" << EOF
# FluxSharp v${version} Release Notes

**Release Date**: $(date +%Y-%m-%d)

## 🎉 What's New

### ✨ Features
- Feature 1
- Feature 2

### 🐛 Bug Fixes
- Fix 1
- Fix 2

### 🔧 Improvements
- Improvement 1
- Improvement 2

## 📋 Changelog

See [CHANGELOG.md](./CHANGELOG.md) for detailed changes.

## 🚀 Installation

\`\`\`bash
tar -xzf fluxsharp-v${version}-linux-x64.tar.gz
cd fluxsharp-v${version}-linux-x64
./build.sh examples/hello.fsh
\`\`\`

## 🔒 Checksums

See \`fluxsharp-v${version}-linux-x64-CHECKSUMS.txt\` for file verification.

---

**Total commits since last release**: $(git rev-list --count v$(get_current_version)..HEAD 2>/dev/null || echo "N/A")

For issues and feedback: https://github.com/yourname/FluxSharp
EOF
        print_success "Release notes template created: $release_notes"
    fi
    
    echo ""
    
    # Stage changes
    step "Staging changes for commit..."
    git add "$COMPILER_DIR/Cargo.toml" "$VERSION_FILE" "$release_notes" 2>/dev/null || true
    
    if git diff --cached --quiet; then
        print_warn "No changes to commit"
    else
        print_success "Changes staged"
    fi
    
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  1. Edit $release_notes to add release notes"
    echo "  2. Review changes: git diff --cached"
    echo "  3. Commit: git commit -m 'Release v${version}'"
    echo "  4. Build: ./release.sh build ${version}"
    echo ""
}

# ============================================================================
# Command: Build
# ============================================================================

cmd_build() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./release.sh build <version>"
    fi
    
    if ! validate_semantic_version "$version"; then
        print_error "Invalid version format: $version (expected X.Y.Z)"
    fi
    
    print_section "🔨 Building Release v${version}"
    
    check_git_repo
    
    echo ""
    
    step "Verifying version in Cargo.toml..."
    if grep -q "version = \"${version}\"" "$COMPILER_DIR/Cargo.toml"; then
        print_success "Version matches in Cargo.toml"
    else
        print_warn "Version mismatch! Cargo.toml doesn't contain v${version}"
        print_info "Current version in Cargo.toml:"
        grep "version =" "$COMPILER_DIR/Cargo.toml" | head -1 || echo "Not found"
    fi
    
    echo ""
    
    # Create release directory
    step "Creating release package..."
    local release_name="fluxsharp-v${version}-linux-x64"
    local release_path="$RELEASES_DIR/$release_name"
    
    mkdir -p "$RELEASES_DIR"
    
    # Call create_release.sh with specific version
    if [ -f "$PROJECT_ROOT/create_release.sh" ]; then
        step "Running package creation script..."
        "$PROJECT_ROOT/create_release.sh" "$version" "$RELEASES_DIR" || {
            print_error "Failed to create release package"
        }
    else
        print_error "create_release.sh not found"
    fi
    
    # Move archives to releases directory if needed
    if [ -f "$release_name.tar.gz" ]; then
        mv "$release_name.tar.gz" "$RELEASES_DIR/" 2>/dev/null || true
        mv "$release_name.zip" "$RELEASES_DIR/" 2>/dev/null || true
        mv "$release_name-CHECKSUMS.txt" "$RELEASES_DIR/" 2>/dev/null || true
    fi
    
    echo ""
    echo -e "${BLUE}Release Package Information:${NC}"
    
    if [ -d "$release_path" ]; then
        local size=$(du -sh "$release_path" | cut -f1)
        echo -e "  Location: $release_path"
        echo -e "  Size:     $size"
    fi
    
    if [ -f "$RELEASES_DIR/$release_name.tar.gz" ]; then
        local tar_size=$(du -h "$RELEASES_DIR/$release_name.tar.gz" | cut -f1)
        echo -e "  Archive:  $RELEASES_DIR/$release_name.tar.gz ($tar_size)"
    fi
    
    echo ""
    echo -e "${GREEN}✅ Release package built successfully!${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo "  ./release.sh publish ${version}        # Create Git tag"
    echo "  ./release.sh publish ${version} --push # Push to GitHub"
    echo ""
}

# ============================================================================
# Command: Publish
# ============================================================================

cmd_publish() {
    local version=$1
    local push_flag=$2
    
    if [ -z "$version" ]; then
        print_error "Usage: ./release.sh publish <version> [--push]"
    fi
    
    if ! validate_semantic_version "$version"; then
        print_error "Invalid version format: $version (expected X.Y.Z)"
    fi
    
    print_section "📤 Publishing Release v${version}"
    
    check_git_repo
    check_git_clean
    
    if version_exists "$version"; then
        print_error "Tag v${version} already exists"
    fi
    
    echo ""
    
    # Verify release package exists
    local release_name="fluxsharp-v${version}-linux-x64"
    if [ ! -d "$RELEASES_DIR/$release_name" ] && [ ! -f "$RELEASES_DIR/$release_name.tar.gz" ]; then
        print_warn "Release package not found at $RELEASES_DIR/$release_name"
    else
        print_success "Release package found"
    fi
    
    echo ""
    
    # Create annotated tag
    step "Creating Git tag v${version}..."
    
    # Read release notes for tag annotation
    local release_notes="$PROJECT_ROOT/RELEASE_NOTES_v${version}.md"
    local tag_message="Release v${version}"
    
    if [ -f "$release_notes" ]; then
        tag_message=$(head -n 5 "$release_notes" | tail -n 1)
    fi
    
    git tag -a "v${version}" -m "$tag_message" HEAD
    print_success "Tag v${version} created"
    
    echo ""
    
    # Push if requested
    if [ "$push_flag" == "--push" ]; then
        step "Pushing tag to remote..."
        
        if ! git push origin "v${version}"; then
            print_warn "Failed to push tag. Make sure you have push access."
        else
            print_success "Tag pushed to remote"
        fi
        
        echo ""
    fi
    
    echo -e "${BLUE}Tag Information:${NC}"
    echo "  Tag:     v${version}"
    echo "  Commit:  $(git rev-parse --short HEAD)"
    echo "  Date:    $(git log -1 --format=%ai HEAD)"
    
    echo ""
    echo -e "${GREEN}✅ Release published!${NC}"
    echo ""
    
    if [ "$push_flag" != "--push" ]; then
        echo -e "${BLUE}Next steps:${NC}"
        echo "  git push origin v${version}             # Push tag to GitHub"
        echo "  gh release create v${version} ..."     # Create GitHub release
        echo ""
    fi
}

# ============================================================================
# Command: Complete
# ============================================================================

cmd_complete() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./release.sh complete <version>"
    fi
    
    if ! validate_semantic_version "$version"; then
        print_error "Invalid version format: $version (expected X.Y.Z)"
    fi
    
    print_header
    echo -e "${MAGENTA}🚀 Complete Release Workflow for v${version}${NC}"
    echo ""
    
    # Step 1: Prepare
    echo -e "${YELLOW}[Step 1/3]${NC} Preparing release..."
    cmd_prepare "$version"
    
    # Ask user to commit
    echo ""
    read -p "Have you edited release notes and committed changes? (y/n) " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Workflow paused. Run when ready: ./release.sh complete $version"
        exit 0
    fi
    
    # Step 2: Build
    echo ""
    echo -e "${YELLOW}[Step 2/3]${NC} Building release package..."
    cmd_build "$version"
    
    # Step 3: Publish
    echo ""
    read -p "Ready to create Git tag? (y/n) " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Skipping publication. Run: ./release.sh publish $version --push"
        exit 0
    fi
    
    echo -e "${YELLOW}[Step 3/3]${NC} Publishing release..."
    cmd_publish "$version" "--push"
    
    echo ""
    echo -e "${GREEN}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}✅ Release v${version} completed successfully!${NC}"
    echo -e "${GREEN}════════════════════════════════════════════════════════════════${NC}"
    echo ""
}

# ============================================================================
# Help
# ============================================================================

show_help() {
    cat << EOF
$(print_header)
📦 FluxSharp Release Workflow Manager

COMMANDS:
    init                Initialize release system
    current             Show current version
    list                List all releases
    prepare <version>   Prepare release (update version numbers)
    build <version>     Build release package
    publish <version>   Create Git tag and optionally push
    complete <version>  Full workflow (prepare → build → publish)

OPTIONS:
    --push              Push to remote when publishing
    --help              Show this help message

EXAMPLES:
    ./release.sh init                      # Initialize once
    ./release.sh current                   # Check current version
    ./release.sh list                      # List all releases
    ./release.sh prepare 0.2.0             # Start preparing v0.2.0
    ./release.sh build 0.2.0               # Build v0.2.0 package
    ./release.sh publish 0.2.0 --push      # Publish and push to GitHub
    ./release.sh complete 1.0.0            # Full workflow for v1.0.0

WORKFLOW:
    1. ./release.sh prepare 0.2.0
       → Update version numbers
       → Create release notes template
       → User edits release notes and commits
    
    2. ./release.sh build 0.2.0
       → Build Rust compiler
       → Generate runtime objects
       → Create package archives
    
    3. ./release.sh publish 0.2.0 --push
       → Create Git tag
       → Optionally push to GitHub

AUTOMATION:
    ./release.sh complete 0.2.0
       → Runs prepare, build, and publish
       → Guides user through each step
       → Requires user confirmation

VERSION FORMAT:
    Semantic Versioning: X.Y.Z
    Example: 0.1.0, 1.0.0, 1.2.3

TAGS:
    Git tags: v<version>
    Example: v0.1.0, v1.0.0

FILES:
    Versions tracked in: $VERSION_FILE
    Releases stored in:  $RELEASES_DIR
    Release notes:       RELEASE_NOTES_v<version>.md

EOF
}

# ============================================================================
# Main
# ============================================================================

main() {
    local command=$1
    
    case "${command}" in
        init)
            cmd_init
            ;;
        current)
            cmd_current
            ;;
        list)
            cmd_list
            ;;
        prepare)
            cmd_prepare "$2"
            ;;
        build)
            cmd_build "$2"
            ;;
        publish)
            cmd_publish "$2" "$3"
            ;;
        complete)
            cmd_complete "$2"
            ;;
        --help|-h|help)
            show_help
            ;;
        "")
            print_header
            echo -e "${YELLOW}No command specified${NC}"
            echo ""
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            ;;
    esac
}

main "$@"
