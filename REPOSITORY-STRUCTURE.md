# 📁 FluxSharp Repository Structure
Complete guide to the FluxSharp repository organization.
---
## 🗂️ Overview
```
FluxSharp/
├── 📄 Documentation Files
├── 📚 Documentation Directory (docs/)
├── 💻 Source Code
├── 🔧 Compiler Code (flux_compiler/)
├── 🎯 Example Programs
└── 🧪 Test Files
```
---
## 📄 Root Documentation Files
### Main Documents (Read First)
| File | Purpose | Audience |
|------|---------|----------|
| **README.md** | Project overview and quick start | Everyone |
| **README-USAGE.md** | Complete user guide with examples | Users & Developers |
| **INDEX.md** | Documentation navigation guide | Everyone |
| **SUMMARY.txt** | Executive summary of work done | Managers & Leads |
### Technical Documents
| File | Purpose | Audience |
|------|---------|----------|
| **CHANGELOG.md** | Version history and improvements | Developers |
| **PROJECT-STATUS.md** | Detailed project status & architecture | Developers |
| **COMPLETION-REPORT.md** | Technical completion report | Team Leads |
| **REPOSITORY-STRUCTURE.md** | This file - repo organization | Everyone |
---
## 📚 Documentation Directory (`docs/`)
```
docs/
├── LANGUAGE_GUIDE.md      # Complete language reference
├── QUICKSTART.md          # Quick start guide
├── EXAMPLES.md            # Advanced code examples
├── SECURITY.md            # Security features & protections
└── README.md              # Documentation overview
```
### Documentation Files Explained
**LANGUAGE_GUIDE.md**
- Complete syntax reference
- All operators and types
- Language constructs
- For: Developers learning the language
**QUICKSTART.md**
- Get up and running in 5 minutes
- Basic examples
- Common tasks
- For: New users
**EXAMPLES.md**
- Advanced code examples
- Real-world use cases
- Best practices
- For: Experienced programmers
**SECURITY.md**
- Security features
- Protections built-in
- Best practices
- For: Security-conscious developers
---
## 💻 Source Code Files
### Example Programs
```
├── main.fsh                       # Main demonstration program
├── math_demo.fsh                  # Mathematical functions demo
├── main.asm                       # Generated assembly (main.fsh)
├── main.o                         # Compiled object file
└── program                        # Executable binary (main.fsh)
```
---
## 🔧 Compiler Code (`flux_compiler/`)
```
flux_compiler/
├── Cargo.toml                     # Rust workspace config
├── Cargo.lock                     # Dependency lock file
│
├── fluxc/
│   ├── Cargo.toml                # Compiler package config
│   │
│   ├── src/
│   │   ├── main.rs               # Main compiler code (~1400 lines)
│   │   │   ├── Lexer/Parser      # PEG parsing logic
│   │   │   ├── Validator         # Type checking & security
│   │   │   ├── Code Generator    # x86_64 assembly output
│   │   │   └── CLI Handler       # Command-line interface
│   │   │
│   │   └── flux_grammar.pest     # PEG grammar definition (~180 lines)
│   │       ├── Tokens            # Lexical elements
│   │       ├── Expressions       # Expression rules
│   │       ├── Statements        # Statement rules
│   │       └── Top-level         # Program structure
│   │
│   ├── runtime/
│   │   ├── runtime.asm           # x86_64 runtime system (~300 lines)
│   │   │   ├── _fsh_print_int    # Integer printing
│   │   │   ├── _fsh_print_str    # String printing
│   │   │   ├── _fsh_print_double # Double printing (NEW in v1.1)
│   │   │   └── Math functions    # sqrt, sin, cos, etc.
│   │   │
│   │   └── runtime.o             # Compiled runtime object
│   │
│   └── target/
│       ├── debug/                # Debug builds
│       └── release/
│           └── fluxc             # Optimized compiler binary
│
└── README.md                      # Compiler documentation
```
### Compiler Components Explained
**main.rs**
- Lexer: Tokenizes source code
- Parser: PEG-based parsing
- Type Checker: Semantic validation
- Code Generator: x86_64 assembly
- Security Checker: Validates constraints
- CLI: Command-line interface
**flux_grammar.pest**
- PEG (Parsing Expression Grammar)
- Defines all language syntax rules
- Tokens, operators, statements
- Used by Pest parser library
**runtime.asm**
- NASM x86_64 assembly
- System calls (write, exit)
- Print functions for all types
- Mathematical functions
- Helper utilities
---
## 🧪 Test Files
```
├── test_simple                    # Simple functionality tests
├── test_methods                   # Method & class tests
├── security_tests.sh              # Security constraint tests
└── program                        # Compiled test binary
```
### Test Descriptions
**test_simple**
- Basic arithmetic
- Variable assignment
- Simple I/O
- Status: ✅ Passing
**test_methods**
- Method calls
- Class functionality
- Object properties
- Status: ✅ Passing
**security_tests.sh**
- Path traversal protection
- Infinite loop detection
- Stack protection
- Symlink validation
- Status: ✅ Passing
---
## 📊 Generated Files (Build Artifacts)
These files are automatically created during compilation:
```
├── main.asm                       # NASM assembly from main.fsh
├── main.o                         # Object file from assembly
├── program                        # Final executable binary
└── math_demo.asm                  # NASM assembly from math_demo.fsh
└── math_demo.o                    # Object file
└── math_demo                      # Final executable
```
**To clean up generated files:**
```bash
rm -f *.asm *.o program math_demo
```
---
## 🚀 Quick Navigation
### For First-Time Users
1. Read → **README.md**
2. Skim → **README-USAGE.md**
3. Run → **main.fsh**
4. Explore → **docs/QUICKSTART.md**
### For Language Learners
1. Study → **docs/LANGUAGE_GUIDE.md**
2. Try → **docs/EXAMPLES.md**
3. Practice → Write your own programs
4. Reference → **docs/SECURITY.md** for safety
### For Developers
1. Review → **PROJECT-STATUS.md**
2. Inspect → **flux_compiler/fluxc/src/main.rs**
3. Check → **flux_compiler/fluxc/src/flux_grammar.pest**
4. Test → Run **security_tests.sh**
### For Maintainers
1. Check → **CHANGELOG.md** for history
2. Review → **COMPLETION-REPORT.md** for status
3. Validate → All test files
4. Update → Documentation as needed
---
## 📈 File Statistics
| Category | Count | Size |
|----------|-------|------|
| Documentation Files | 5 | 35 KB |
| Language Docs | 5 | 50 KB |
| Source Code | 2 | 6 KB |
| Compiler Code | 2 | 80 KB |
| Tests | 3 | 20 KB |
| Generated Files | 6 | ~100 KB |
| **Total** | **23** | **~291 KB** |
---
## 🔄 Development Workflow
### Step 1: Write Code
```
Edit → main.fsh
```
### Step 2: Compile
```
$ ./flux_compiler/target/release/fluxc compile main.fsh -o program
🔍 Reading source: "main.fsh"
📝 Generated ASM: "main.asm"
🔨 Assembled: "main.o"
✅ Linked binary: "program"
```
### Step 3: Test
```
$ ./program
(output)
```
### Step 4: Clean (optional)
```
$ rm -f main.asm main.o program
```
---
## 📋 File Purposes at a Glance
### Essential Files
- **README.md** → Start here!
- **README-USAGE.md** → Complete guide
- **docs/LANGUAGE_GUIDE.md** → Language reference
- **main.fsh** → Working example
### Reference Files
- **INDEX.md** → Find what you need
- **CHANGELOG.md** → What changed
- **PROJECT-STATUS.md** → Current state
### Implementation Files
- **flux_compiler/fluxc/src/main.rs** → Compiler
- **flux_compiler/fluxc/src/flux_grammar.pest** → Syntax
- **flux_compiler/fluxc/runtime/runtime.asm** → Runtime
### Test Files
- **test_simple** → Basic tests
- **test_methods** → Class tests
- **security_tests.sh** → Security tests
---
## ✨ Key Features by Location
| Feature | File | Status |
|---------|------|--------|
| sqrt(), pow() | main.rs (eval_atom) | ✅ Working |
| Float display | runtime.asm | ✅ Fixed v1.1 |
| Type checking | main.rs (validate) | ✅ Complete |
| Security | main.rs (validate_*) | ✅ Complete |
| Assembly output | main.rs (generate_*) | ✅ Working |
| NASM assembly | flux_grammar.pest | ✅ Complete |
| Math functions | main.rs + runtime.asm | ✅ 15+ functions |
---
## 🎯 Version Information
**Current Version:** 1.1.0  
**Last Updated:** March 27, 2026  
**Language:** Flux (Compiled to x86_64)  
**Compiler Language:** Rust  
**Status:** ✅ Stable & Production-Ready
---
## 📞 Getting Help
- **Usage questions** → See **README-USAGE.md**
- **Syntax questions** → See **docs/LANGUAGE_GUIDE.md**
- **Examples needed** → See **docs/EXAMPLES.md**
- **Navigation help** → See **INDEX.md**
- **Project overview** → See **PROJECT-STATUS.md**
---
**Organization Version:** 1.0  
**Last Reviewed:** March 27, 2026
