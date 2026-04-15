# 🚀 FluxSharp CI/CD & GitFlow - Complete Guide

**FluxSharp** - An open-source, high-performance programming language compiler with professional CI/CD pipeline and automated testing, building, and releasing.

---

## 🎯 Overview

FluxSharp's enterprise-ready release system includes:

✅ **GitHub Actions CI** - Automatic testing on every push  
✅ **Automated Release Workflow** - One-click release creation  
✅ **GitFlow Branching** - alpha → beta → stable workflow  
✅ **Version Management** - Automatic version updates  
✅ **Package Generation** - Automatic archive creation with checksums  

---

## 📊 Branch Strategy

```
main (stable)
  ↓
beta (pre-release testing)
  ↓
alpha (early testing)
  ↓
develop (new features)
```

### Branch Types

| Branch | Purpose | Audience | Version Tag |
|--------|---------|----------|------------|
| `main` | Production releases stable | Users / Open-source community | v1.0.0 |
| `beta` | Late testing | Beta testers | v1.0.0-beta |
| `alpha` | Early testing | Developers | v1.0.0-alpha |
| `develop` | New features | Contributors | - |
| `hotfix/*` | Emergency fixes | Urgent production bugs | - |

---

## 🔄 CI Workflow (Automatic)

Every push to `main`, `develop`, `alpha`, or `beta` triggers:

1. **Build** - Compile Rust compiler
2. **Lint** - Check code formatting with rustfmt
3. **Test** - Run security & logic tests
4. **Release Build** - Test package creation

View CI status: https://github.com/yourname/FluxSharp/actions

---

## 🚀 Release Workflow (Manual Trigger)

### Step 1: Start Alpha Testing

```bash
./gitflow.sh alpha 1.1.0
```

**What happens:**
- Creates `alpha` branch from `main`
- Tags version as v1.1.0-alpha
- Push to GitHub

**For testers:**
```bash
git checkout alpha
# Testing and fixes
git commit -m "Fix alpha issue"
git push origin alpha
```

### Step 2: Start Beta Testing

```bash
./gitflow.sh beta 1.1.0
```

**What happens:**
- Creates `beta` branch from `alpha`
- Tags version as v1.1.0-beta
- Ready for wider testing

**For beta testers:**
```bash
git checkout beta
# More testing and fixes
git commit -m "Fix beta issue"
git push origin beta
```

### Step 3: Release to Stable

```bash
./gitflow.sh release 1.1.0
```

**What happens:**
- Merges `beta` to `main`
- Creates v1.1.0 tag
- **Triggers GitHub Actions release workflow**

**Automatic release workflow will:**
- ✅ Build Rust compiler
- ✅ Generate runtime objects
- ✅ Create .tar.gz and .zip archives
- ✅ Generate SHA256 checksums
- ✅ Create GitHub Release
- ✅ Attach files to release

---

## 🔥 Emergency Hotfix

For urgent production patches:

```bash
./gitflow.sh hotfix 1.0.1
```

**What happens:**
- Creates `hotfix/1.0.1` branch from `main`
- Make your fixes
- When ready: `./gitflow.sh release 1.0.1`

**Example:**
```bash
./gitflow.sh hotfix 1.0.1
git checkout hotfix/1.0.1

# Fix critical bug
vim flux_compiler/src/...
git commit -m "Critical security fix"
git push origin hotfix/1.0.1

# Release when ready
./gitflow.sh release 1.0.1
```

---

## 📦 GitHub Actions Workflows

### `.github/workflows/ci.yml`

**Triggers**: Every push to main, develop, alpha, beta

**Jobs:**
- Build on stable and nightly Rust
- Lint code (rustfmt, clippy)
- Test release package creation

**View:** GitHub → Actions → CI

### `.github/workflows/release.yml`

**Triggers**: Manual dispatch from GitHub (or script)

**Inputs:**
- Version (X.Y.Z format)
- Release type (stable, alpha, beta)

**Automatic Actions:**
- Build compiler
- Generate packages
- Create GitHub Release
- Attach archives

---

## 🎮 Quick Start

### 1. Create Alpha Release

```bash
./gitflow.sh alpha 1.1.0
git checkout alpha
# ... test and fix if needed ...
git push origin alpha
```

### 2. Create Beta Release

```bash
./gitflow.sh beta 1.1.0
git checkout beta
# ... final testing ...
git push origin beta
```

### 3. Release to Stable

```bash
./gitflow.sh release 1.1.0
# GitHub Actions automatically creates release!
```

### 4. Verify Release

Go to: https://github.com/yourname/FluxSharp/releases

Should see:
- v1.1.0 tag
- Archives (.tar.gz, .zip)
- Checksums
- Release notes

---

## 📋 GitFlow Commands

```bash
# Show branch status
./gitflow.sh status

# Start alpha testing
./gitflow.sh alpha 1.1.0

# Start beta testing
./gitflow.sh beta 1.1.0

# Release to stable
./gitflow.sh release 1.1.0

# Emergency hotfix
./gitflow.sh hotfix 1.0.1

# Sync all branches
./gitflow.sh sync

# Show workflow diagram
./gitflow.sh workflow

# Help
./gitflow.sh help
```

---

## 🔧 GitHub Actions Setup

### Enable Actions

1. Go to GitHub repo settings
2. Actions → General
3. Enable "Allow all actions and reusable workflows"

### Configure Release Token

The release workflow uses `GITHUB_TOKEN` (automatic).

No additional setup needed!

---

## 📊 Status Checks

### View CI Status

```
GitHub → Actions → CI
```

Shows:
- ✅ Build passes
- ✅ Lint passes
- ✅ Tests pass

### View Release Status

```
GitHub → Releases → v1.1.0
```

Shows:
- ✅ Archives uploaded
- ✅ Checksums generated
- ✅ Release notes published

---

## 🎯 Typical Release Timeline

### Week 1: Alpha
```bash
Monday: ./gitflow.sh alpha 1.1.0
        # Developers test and fix
Tuesday-Friday: git commit fixes, git push origin alpha
```

### Week 2: Beta
```bash
Monday: ./gitflow.sh beta 1.1.0
        # Beta testers test
Tuesday-Thursday: git commit fixes, git push origin beta
Friday: Final testing complete
```

### Week 3: Stable
```bash
Monday: ./gitflow.sh release 1.1.0
        # GitHub Actions creates release automatically!
Tuesday: Announce release
```

---

## 💡 Tips & Tricks

### Check Branch Status

```bash
./gitflow.sh status
```

Shows:
- Current branch
- Local branches
- Remote branches

### Sync Branches

If branches drift apart:

```bash
./gitflow.sh sync
```

Merges:
- alpha ← main
- beta ← alpha

### View Workflow Diagram

```bash
./gitflow.sh workflow
```

Shows ASCII diagram of branch flow.

---

## 🐛 Troubleshooting

### "Already on alpha"

```bash
# Just pull latest
git pull origin alpha
```

### "Merge conflicts"

```bash
# Manually resolve
git checkout --theirs .
git add -A
git commit -m "Resolve merge"
git push origin branch-name
```

### "CI workflow failed"

```
GitHub → Actions → [workflow] → [job]
```

Check error logs and fix issues.

### "Release didn't create"

```bash
# Check GitHub Actions
GitHub → Actions → release

# Verify version format
./gitflow.sh release 1.1.0  # Correct: X.Y.Z
./gitflow.sh release 1.1    # Wrong: missing patch
```

---

## 🚀 Full Workflow Example

### Day 1: Plan v1.1.0

```bash
# Start alpha
./gitflow.sh alpha 1.1.0
echo "Starting alpha testing for v1.1.0"
```

### Day 2-5: Test & Fix Alpha

```bash
# Developers work
git checkout alpha

# Fix issues
vim flux_compiler/src/...
cargo build --release
./build.sh examples/hello.fsh

# Commit and push
git add -A
git commit -m "Fix alpha issue #123"
git push origin alpha
```

GitHub Actions automatically tests every push!

### Day 8: Move to Beta

```bash
./gitflow.sh beta 1.1.0
echo "Moving to beta testing"
git checkout beta
```

### Day 9-12: Beta Testing

```bash
# Beta testers work
git checkout beta

# Fix remaining issues
git commit -m "Fix beta issue"
git push origin beta
```

### Day 15: Release!

```bash
./gitflow.sh release 1.1.0
# GitHub Actions automatically:
# - Creates v1.1.0 tag
# - Builds compiler
# - Generates packages
# - Creates GitHub Release
# - Uploads archives
# - Generates checksums
```

### Day 16: Announce

Go to: https://github.com/yourname/FluxSharp/releases/tag/v1.1.0

Everything is ready for users! 🎉

---

## 📚 Next Steps

1. **Enable GitHub Actions**
   - Go to repo settings
   - Actions → Enable

2. **Test a release**
   ```bash
   ./gitflow.sh alpha 1.1.0-test
   ./gitflow.sh beta 1.1.0-test
   ```

3. **Create your first real release**
   ```bash
   ./gitflow.sh release 1.1.0
   ```

4. **Verify on GitHub**
   ```
   https://github.com/yourname/FluxSharp/releases
   ```

---

## 🎓 Learn More

- [git-flow cheatsheet](https://danielkummer.github.io/git-flow-cheatsheet/)
- [GitHub Actions docs](https://docs.github.com/actions)
- [Semantic Versioning](https://semver.org/)

---

## 📜 License & Open Source

FluxSharp is distributed as open-source software. All release artifacts, source code, and documentation are freely available to the community.

**To contribute:**
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

**For issues and discussion:**
- Open GitHub Issues
- Participate in Discussions
- Join our community!

---

**Your FluxSharp project is now enterprise-ready and open-source!** 🚀

Questions? Run:
```bash
./gitflow.sh help
./release.sh --help
```
