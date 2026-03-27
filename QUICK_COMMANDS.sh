#!/usr/bin/env bash
# FluxSharp Quick Commands Reference

echo "
╔════════════════════════════════════════════════════════════════════════╗
║                   FluxSharp - Quick Commands                          ║
╚════════════════════════════════════════════════════════════════════════╝

🚀 ONE COMMAND TO BUILD AND RUN EVERYTHING:

    ./build.sh

That's it! This will:
  1. Build the Rust compiler
  2. Compile main.fsh to assembly
  3. Assemble to machine code
  4. Link to executable
  5. Run the program

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📖 DOCUMENTATION:

  Quick Start:     cat docs/QUICKSTART.md
  Syntax:          cat docs/SYNTAX.md
  All Docs:        ls docs/
  This README:     cat README.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✏️  WORKFLOW:

  1. Edit code:    nano main.fsh
  2. Build & run:  ./build.sh
  3. See output
  4. Repeat!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚙️  ALTERNATIVE BUILD OPTIONS:

  Using make:      make quickstart
  Just build:      make build
  Just run:        make run
  Clean:           make clean
  Help:            make help

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📚 DOCUMENTATION FILES:

  docs/QUICKSTART.md         Get started in 5 minutes
  docs/SYNTAX.md             Language syntax
  docs/TYPES.md              Data types
  docs/VARIABLES.md          Variable management
  docs/FUNCTIONS.md          Functions
  docs/CLASSES.md            Classes and OOP
  docs/CONTROL_FLOW.md       If/while/for
  docs/OPERATORS.md          All operators
  docs/ARRAYS.md             Array operations
  docs/ASYNC_AWAIT.md        Async programming
  docs/STDLIB.md             Math, string, I/O

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔍 FILES IN THIS PROJECT:

  build.sh                   Build and run script
  main.fsh                   Your FluxSharp code
  Makefile                   Build system
  README.md                  Project overview
  FINAL_SUMMARY.md           Complete summary
  flux_compiler/             Rust compiler source
  docs/                      All documentation
  examples/                  Example programs

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ You're ready! Run:

    ./build.sh

╔════════════════════════════════════════════════════════════════════════╗
║        FluxSharp v2.0.0 - Modern Programming Language                 ║
║              Production Ready - Documentation Complete                 ║
╚════════════════════════════════════════════════════════════════════════╝
"

