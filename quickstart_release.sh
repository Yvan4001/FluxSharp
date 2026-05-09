#!/bin/bash

###############################################################################
# FluxSharp - Open-Source CI/CD Quick Start Script
#
# FluxSharp: High-performance, open-source programming language compiler
# This script automates the initial setup for CI/CD and GitFlow branching.
#
# Usage: ./quickstart_release.sh [command]
# License: See LICENSE file in repository
###############################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Config
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VERSION_FILE="${SCRIPT_DIR}/flux_compiler/Cargo.toml"

###############################################################################
# Helper Functions
###############################################################################

print_header() {
    echo -e "\n${PURPLE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${PURPLE}║${NC} $1"
    echo -e "${PURPLE}╚════════════════════════════════════════════════════════════╝${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

get_current_version() {
    if [[ -f "$VERSION_FILE" ]]; then
        grep '^version' "$VERSION_FILE" | head -1 | sed 's/.*"\([^"]*\)".*/\1/'
    else
        echo "1.0.0"
    fi
}

confirm() {
    local prompt="$1"
    local response
    
    read -p "$(echo -e ${CYAN}$prompt${NC} ' [y/N] ')" -n 1 -r response
    echo
    [[ $response =~ ^[Yy]$ ]]
}

###############################################################################
# Main Commands
###############################################################################

cmd_help() {
    cat <<EOF

${BLUE}FluxSharp - Open-Source CI/CD Quick Start${NC}
High-Performance Programming Language Compiler

USAGE:
    ./quickstart_release.sh [command]

COMMANDS:
    help              Show this help message
    setup             Check prerequisites
    verify            Verify all workflows
    init-alpha        Initialize alpha branch (v1.0.0-alpha)
    init-beta         Initialize beta branch (v1.0.0-beta)
    full-setup        Complete setup: Check → Init branches → Show workflow
    status            Show current branch status
    workflow          Show release workflow diagram

EXAMPLES:
    ./quickstart_release.sh setup
    ./quickstart_release.sh full-setup
    ./quickstart_release.sh init-alpha
    ./quickstart_release.sh status

EOF
}

cmd_setup() {
    print_header "🔧 Checking Prerequisites"
    
    # Check Git
    if command -v git &> /dev/null; then
        VERSION=$(git --version)
        print_success "Git installed: $VERSION"
    else
        print_error "Git not found! Install with: apt install git"
        return 1
    fi
    
    # Check Cargo
    if command -v cargo &> /dev/null; then
        VERSION=$(cargo --version)
        print_success "Cargo installed: $VERSION"
    else
        print_error "Cargo not found! Install Rust from https://rustup.rs"
        return 1
    fi
    
    # Check NASM
    if command -v nasm &> /dev/null; then
        VERSION=$(nasm -version 2>&1 | head -1)
        print_success "NASM installed: $VERSION"
    else
        print_warning "NASM not found. Required for assembly compilation."
        echo "  Install with: apt install nasm"
    fi
    
    # Check LD
    if command -v ld &> /dev/null; then
        print_success "LD linker installed"
    else
        print_warning "LD linker not found. Install with: apt install binutils"
    fi
    
    # Check our scripts
    if [[ -f "${SCRIPT_DIR}/gitflow.sh" ]]; then
        print_success "gitflow.sh found"
    else
        print_error "gitflow.sh not found!"
        return 1
    fi
    
    if [[ -f "${SCRIPT_DIR}/release.sh" ]]; then
        print_success "release.sh found"
    else
        print_error "release.sh not found!"
        return 1
    fi
    
    echo
    print_info "All core prerequisites met!"
    print_info "Run './quickstart_release.sh full-setup' to initialize"
}

cmd_verify() {
    print_header "✅ Verifying Workflows"
    
    # Check Git repository
    if git -C "$SCRIPT_DIR" rev-parse --git-dir > /dev/null 2>&1; then
        print_success "Git repository initialized"
    else
        print_error "Not a git repository!"
        return 1
    fi
    
    # Check workflows exist
    if [[ -f "${SCRIPT_DIR}/.github/workflows/release.yml" ]]; then
        print_success "Release workflow exists"
    else
        print_warning "Release workflow not found at .github/workflows/release.yml"
    fi
    
    if [[ -f "${SCRIPT_DIR}/.github/workflows/ci.yml" ]]; then
        print_success "CI workflow exists"
    else
        print_warning "CI workflow not found at .github/workflows/ci.yml"
    fi
    
    # Check branches
    echo
    print_info "Checking branches:"
    
    if git -C "$SCRIPT_DIR" rev-parse --verify develop > /dev/null 2>&1; then
        print_success "develop branch exists"
    else
        print_warning "develop branch doesn't exist yet"
    fi
    
    if git -C "$SCRIPT_DIR" rev-parse --verify alpha > /dev/null 2>&1; then
        print_success "alpha branch exists"
    else
        print_info "alpha branch will be created"
    fi
    
    if git -C "$SCRIPT_DIR" rev-parse --verify beta > /dev/null 2>&1; then
        print_success "beta branch exists"
    else
        print_info "beta branch will be created"
    fi
    
    echo
    print_success "Verification complete!"
}

cmd_init_alpha() {
    print_header "🚀 Creating Alpha Branch"
    
    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: $CURRENT_VERSION"
    
    if confirm "Create alpha branch for v${CURRENT_VERSION}?"; then
        print_info "Running: ./gitflow.sh alpha ${CURRENT_VERSION}"
        
        cd "$SCRIPT_DIR"
        if bash gitflow.sh alpha "${CURRENT_VERSION}"; then
            print_success "Alpha branch created!"
            
            echo
            print_info "Next steps:"
            echo "  1. Test on alpha branch:"
            echo "     git checkout alpha"
            echo "     cargo build --release"
            echo "  2. Push to GitHub:"
            echo "     git push origin alpha"
            echo "  3. When ready, create beta:"
            echo "     ./quickstart_release.sh init-beta"
        else
            print_error "Failed to create alpha branch"
            return 1
        fi
    fi
}

cmd_init_beta() {
    print_header "🚀 Creating Beta Branch"
    
    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: $CURRENT_VERSION"
    
    # Check if alpha exists
    if ! git -C "$SCRIPT_DIR" rev-parse --verify alpha > /dev/null 2>&1; then
        print_error "Alpha branch doesn't exist yet!"
        print_info "Run './quickstart_release.sh init-alpha' first"
        return 1
    fi
    
    if confirm "Create beta branch for v${CURRENT_VERSION}?"; then
        print_info "Running: ./gitflow.sh beta ${CURRENT_VERSION}"
        
        cd "$SCRIPT_DIR"
        if bash gitflow.sh beta "${CURRENT_VERSION}"; then
            print_success "Beta branch created!"
            
            echo
            print_info "Next steps:"
            echo "  1. Test on beta branch:"
            echo "     git checkout beta"
            echo "     cargo build --release"
            echo "  2. Push to GitHub:"
            echo "     git push origin beta"
            echo "  3. When ready, release to stable:"
            echo "     ./gitflow.sh release ${CURRENT_VERSION}"
        else
            print_error "Failed to create beta branch"
            return 1
        fi
    fi
}

cmd_full_setup() {
    print_header "🎯 Full CI/CD Setup"
    
    print_info "This will:"
    echo "  1. Check prerequisites"
    echo "  2. Verify workflows"
    echo "  3. Create alpha and beta branches"
    echo ""
    
    if ! confirm "Continue with full setup?"; then
        print_info "Setup cancelled"
        return 0
    fi
    
    # Step 1: Check prerequisites
    print_header "Step 1/3: Checking Prerequisites"
    if ! cmd_setup; then
        print_error "Setup failed at prerequisite check"
        return 1
    fi
    
    echo
    print_info "Press any key to continue..."
    read -n 1
    
    # Step 2: Verify workflows
    print_header "Step 2/3: Verifying Workflows"
    if ! cmd_verify; then
        print_error "Setup failed at verification"
        return 1
    fi
    
    echo
    print_info "Press any key to continue..."
    read -n 1
    
    # Step 3: Create branches
    print_header "Step 3/3: Creating Release Branches"
    
    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: $CURRENT_VERSION"
    
    if confirm "Create both alpha and beta branches for v${CURRENT_VERSION}?"; then
        cd "$SCRIPT_DIR"
        
        print_info "Creating alpha branch..."
        if bash gitflow.sh alpha "${CURRENT_VERSION}"; then
            print_success "Alpha branch created"
        else
            print_error "Failed to create alpha branch"
            return 1
        fi
        
        echo
        print_info "Creating beta branch..."
        if bash gitflow.sh beta "${CURRENT_VERSION}"; then
            print_success "Beta branch created"
        else
            print_error "Failed to create beta branch"
            return 1
        fi
        
        echo
        print_header "✅ Setup Complete!"
        
        print_info "Your release workflow is ready:"
        echo ""
        echo "  Branch Structure:"
        echo "    main (stable)"
        echo "      └─ beta (pre-release testing)"
        echo "           └─ alpha (early testing)"
        echo ""
        echo "  Next Actions:"
        echo "    1. Push branches to GitHub:"
        echo "       git push origin alpha beta"
        echo ""
        echo "    2. Test your code:"
        echo "       git checkout alpha"
        echo "       cargo build --release"
        echo ""
        echo "    3. Commit and push fixes:"
        echo "       git commit -am 'Fix alpha issue'"
        echo "       git push origin alpha"
        echo ""
        echo "    4. When ready to release:"
        echo "       ./gitflow.sh release ${CURRENT_VERSION}"
        echo ""
        print_success "Happy releasing! 🚀"
        echo ""
        print_info "FluxSharp is open-source. Share your releases with the community!"
    fi
}

cmd_status() {
    print_header "📊 Current Status"
    
    cd "$SCRIPT_DIR"
    
    # Current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    print_info "Current branch: ${CYAN}${CURRENT_BRANCH}${NC}"
    
    # Current version
    CURRENT_VERSION=$(get_current_version)
    print_info "Current version: ${CYAN}${CURRENT_VERSION}${NC}"
    
    echo
    print_info "Local branches:"
    git branch --format='%(refname:short)' | while read branch; do
        if [[ "$branch" == "$CURRENT_BRANCH" ]]; then
            echo "  ${GREEN}* $branch${NC}"
        else
            echo "  $branch"
        fi
    done
    
    echo
    print_info "Tags:"
    git tag --list | tail -10 | while read tag; do
        echo "  $tag"
    done
    
    echo
    print_info "Uncommitted changes:"
    CHANGES=$(git status --porcelain | wc -l)
    if [[ $CHANGES -eq 0 ]]; then
        print_success "Working tree clean"
    else
        print_warning "$CHANGES files with changes"
    fi
}

cmd_workflow() {
    print_header "🔄 Release Workflow"
    
    cat << 'EOF'
┌─────────────────────────────────────────────────────────────────┐
│                   FluxSharp Release Workflow                     │
└─────────────────────────────────────────────────────────────────┘

                      MAIN (Stable)
                          ▲
                          │
                    Tag: v1.0.0
                          │
                   ┌──────┴──────┐
                   │  RELEASE    │
                   │  PROCESS    │
                   └──────┬──────┘
                          │
                        BETA
                          ▲
                          │
                    Tag: v1.0.0-beta
                          │
                   ┌──────┴──────┐
                   │ FINAL TEST  │
                   │  FIXES      │
                   └──────┬──────┘
                          │
                        ALPHA
                          ▲
                          │
                    Tag: v1.0.0-alpha
                          │
                   ┌──────┴──────┐
                   │ EARLY TEST  │
                   │  + FIXES    │
                   └──────┬──────┘
                          │
                      DEVELOP
                    (New Features)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

        COMMANDS                      RESULTS
  ────────────────                    ───────
  ./gitflow.sh alpha 1.0.0            Create alpha branch + tag
                                      Tag: v1.0.0-alpha
  
  ... test and fix ...
  git commit -am "Fix"
  git push origin alpha
                                      GitHub Actions tests
  
  ./gitflow.sh beta 1.0.0             Create beta branch + tag
                                      Tag: v1.0.0-beta
  
  ... test and fix ...
  git commit -am "Fix"
  git push origin beta
                                      GitHub Actions tests
  
  ./gitflow.sh release 1.0.0          Merge beta → main
                                      Create tag: v1.0.0
                                      GitHub Actions builds release
                                      ✅ Packages created
                                      ✅ Checksums generated
                                      ✅ GitHub Release published

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
EOF

    echo
    print_info "For more details, see CI_CD_GITFLOW_GUIDE.md"
}

###############################################################################
# Main
###############################################################################

main() {
    local command="${1:-help}"
    
    case "$command" in
        help)
            cmd_help
            ;;
        setup)
            cmd_setup
            ;;
        verify)
            cmd_verify
            ;;
        init-alpha)
            cmd_init_alpha
            ;;
        init-beta)
            cmd_init_beta
            ;;
        full-setup)
            cmd_full_setup
            ;;
        status)
            cmd_status
            ;;
        workflow)
            cmd_workflow
            ;;
        *)
            print_error "Unknown command: $command"
            echo
            cmd_help
            exit 1
            ;;
    esac
}

# Run main
main "$@"
