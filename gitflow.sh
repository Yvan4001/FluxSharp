#!/bin/bash
################################################################################
# FluxSharp GitFlow Manager
# Manages alpha, beta, and stable branches with automatic testing
################################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}╔═══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  🌳 FluxSharp GitFlow Manager${BLUE}                             ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_section() {
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}$1${NC}"
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

# ============================================================================
# Commands
# ============================================================================

cmd_status() {
    print_header
    print_section "📊 Branch Status"
    
    echo ""
    echo -e "${YELLOW}Local Branches:${NC}"
    git branch -v | sed 's/^/  /'
    
    echo ""
    echo -e "${YELLOW}Remote Branches:${NC}"
    git branch -r | sed 's/^/  /'
    
    echo ""
    print_info "Current branch: $(git branch --show-current)"
}

cmd_start_alpha() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./gitflow.sh alpha <version>"
    fi
    
    print_header
    print_section "🧪 Start Alpha Testing - v${version}"
    
    echo ""
    echo -e "${YELLOW}Creating alpha branch...${NC}"
    
    # Ensure we're on develop
    git checkout -b alpha 2>/dev/null || git checkout alpha
    
    # Merge latest from main
    git merge main --allow-unrelated-histories 2>/dev/null || true
    
    # Update version
    sed -i "s/^version = \".*\"/version = \"${version}-alpha\"/" flux_compiler/Cargo.toml
    echo "alpha=$version" > .toolversions
    
    git add .toolversions flux_compiler/Cargo.toml
    git commit -m "Start alpha testing v${version}" || true
    git push origin alpha || true
    
    print_success "Alpha branch ready for testing: v${version}-alpha"
    echo ""
    echo "Next steps:"
    echo "  git checkout alpha"
    echo "  # Make changes"
    echo "  git commit -m 'Alpha fixes'"
    echo "  git push origin alpha"
}

cmd_start_beta() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./gitflow.sh beta <version>"
    fi
    
    print_header
    print_section "🧪 Start Beta Testing - v${version}"
    
    echo ""
    echo -e "${YELLOW}Creating beta branch...${NC}"
    
    # Ensure we're on alpha
    git checkout -b beta 2>/dev/null || git checkout beta
    
    # Merge from alpha
    git merge alpha --allow-unrelated-histories 2>/dev/null || true
    
    # Update version
    sed -i "s/^version = \".*\"/version = \"${version}-beta\"/" flux_compiler/Cargo.toml
    echo "beta=$version" > .toolversions
    
    git add .toolversions flux_compiler/Cargo.toml
    git commit -m "Start beta testing v${version}" || true
    git push origin beta || true
    
    print_success "Beta branch ready for testing: v${version}-beta"
    echo ""
    echo "Next steps:"
    echo "  git checkout beta"
    echo "  # Make fixes"
    echo "  git commit -m 'Beta fixes'"
    echo "  git push origin beta"
}

cmd_release_stable() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./gitflow.sh release <version>"
    fi
    
    print_header
    print_section "🚀 Release to Stable - v${version}"
    
    echo ""
    echo -e "${YELLOW}Merging beta to main...${NC}"
    
    # Ensure clean working directory
    [ -z "$(git status --porcelain)" ] || print_error "Working directory not clean"
    
    # Checkout main
    git checkout main
    git pull origin main
    
    # Merge from beta
    git merge beta -m "Merge v${version} from beta"
    
    # Update version
    sed -i "s/^version = \".*\"/version = \"${version}\"/" flux_compiler/Cargo.toml
    echo "stable=$version" > .toolversions
    
    git add flux_compiler/Cargo.toml .toolversions
    git commit -m "Release v${version}" || true
    
    # Create tag
    git tag -a "v${version}" -m "Release v${version}"
    
    # Push everything
    git push origin main
    git push origin --tags
    
    print_success "Released to stable: v${version}"
    echo ""
    echo "Next steps:"
    echo "  ./release.sh publish ${version} --push"
}

cmd_hotfix() {
    local version=$1
    
    if [ -z "$version" ]; then
        print_error "Usage: ./gitflow.sh hotfix <version>"
    fi
    
    print_header
    print_section "🔥 Hotfix - v${version}"
    
    echo ""
    echo -e "${YELLOW}Creating hotfix branch...${NC}"
    
    git checkout -b "hotfix/${version}" main
    
    echo "hotfix=$version" > .toolversions
    git add .toolversions
    git commit -m "Start hotfix v${version}"
    git push origin "hotfix/${version}"
    
    print_success "Hotfix branch created: hotfix/${version}"
    echo ""
    echo "Next steps:"
    echo "  git checkout hotfix/${version}"
    echo "  # Make hotfix changes"
    echo "  git commit -m 'Hotfix for v${version}'"
    echo "  git push origin hotfix/${version}"
}

cmd_sync() {
    print_header
    print_section "🔄 Sync Branches"
    
    echo ""
    echo -e "${YELLOW}Syncing alpha ← main${NC}"
    git checkout alpha 2>/dev/null || git checkout -b alpha
    git merge main 2>/dev/null || true
    git push origin alpha || true
    print_success "Alpha synced"
    
    echo ""
    echo -e "${YELLOW}Syncing beta ← alpha${NC}"
    git checkout beta 2>/dev/null || git checkout -b beta
    git merge alpha 2>/dev/null || true
    git push origin beta || true
    print_success "Beta synced"
    
    echo ""
    print_success "All branches synced"
}

cmd_show_workflow() {
    print_header
    print_section "📖 GitFlow Workflow"
    
    cat << 'EOF'

╔════════════════════════════════════════════════════════════╗
║           FluxSharp GitFlow Workflow                       ║
╚════════════════════════════════════════════════════════════╝

BRANCHES:
  main/        Production-ready releases (stable)
  develop/     Integration branch (future features)
  alpha/       Early testing (unstable)
  beta/        Late testing (nearly stable)
  hotfix/*     Emergency fixes from production

WORKFLOW:

  1. Start Alpha Testing
     ./gitflow.sh alpha 1.1.0
     └─ Creates alpha branch with v1.1.0-alpha tag

  2. Test & Fix Alpha
     git checkout alpha
     ... make changes ...
     git commit -m "Alpha fix"
     git push origin alpha

  3. Move to Beta
     ./gitflow.sh beta 1.1.0
     └─ Creates beta branch from alpha with v1.1.0-beta tag

  4. Test & Fix Beta
     git checkout beta
     ... make changes ...
     git commit -m "Beta fix"
     git push origin beta

  5. Release to Stable
     ./gitflow.sh release 1.1.0
     └─ Merges beta to main, creates v1.1.0 tag

  6. Emergency Hotfix (from production)
     ./gitflow.sh hotfix 1.0.1
     └─ Creates hotfix/1.0.1 branch from main

SYNCHRONIZATION:
  ./gitflow.sh sync    ← Keep branches in sync


BRANCH HIERARCHY:
  
  main (stable)
    ↑
    ← merge
    
  beta (late testing)
    ↑
    ← merge
    
  alpha (early testing)
    ↑
    ← merge
    
  develop (new features)

EOF
}

cmd_help() {
    cat << 'EOF'
🌳 FluxSharp GitFlow Manager

COMMANDS:
  status             Show branch status
  alpha <version>    Start alpha testing
  beta <version>     Start beta testing
  release <version>  Release to stable
  hotfix <version>   Start emergency hotfix
  sync               Sync all branches
  workflow           Show workflow diagram
  help               Show this help

EXAMPLES:
  ./gitflow.sh status                # Show current branches
  ./gitflow.sh alpha 1.1.0          # Start alpha on v1.1.0
  ./gitflow.sh beta 1.1.0           # Start beta on v1.1.0
  ./gitflow.sh release 1.1.0        # Release v1.1.0 to stable
  ./gitflow.sh hotfix 1.0.1         # Create hotfix for v1.0.1
  ./gitflow.sh sync                 # Sync all branches
  ./gitflow.sh workflow             # Show workflow

QUICK START:
  1. ./gitflow.sh alpha 1.1.0      # Start new version
  2. git checkout alpha
  3. ... make changes ...
  4. ./gitflow.sh beta 1.1.0       # Move to beta
  5. ... test & fix ...
  6. ./gitflow.sh release 1.1.0    # Release!

EOF
}

# ============================================================================
# Main
# ============================================================================

main() {
    local command=$1
    
    case "${command}" in
        status)
            cmd_status
            ;;
        alpha)
            cmd_start_alpha "$2"
            ;;
        beta)
            cmd_start_beta "$2"
            ;;
        release)
            cmd_release_stable "$2"
            ;;
        hotfix)
            cmd_hotfix "$2"
            ;;
        sync)
            cmd_sync
            ;;
        workflow)
            cmd_show_workflow
            ;;
        help|--help|-h)
            cmd_help
            ;;
        "")
            print_header
            cmd_help
            ;;
        *)
            print_error "Unknown command: $command"
            ;;
    esac
}

main "$@"
