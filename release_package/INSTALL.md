# FluxSharp Installation Guide

## System Requirements

### Minimum
- Linux x86-64
- 50 MB disk space
- 2 GB RAM
- glibc 2.29+

### Build Tools Required
- `nasm` (assembler)
- `ld` (linker)

## Installation Steps

### 1. Extract Archive

```bash
tar -xzf fluxsharp-v0.1.0-linux-x64.tar.gz
cd fluxsharp-v0.1.0-linux-x64
```

### 2. Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y nasm binutils
```

**Fedora/RHEL:**
```bash
sudo dnf install -y nasm binutils
```

**Arch:**
```bash
sudo pacman -S nasm binutils
```

### 3. Verify Installation

```bash
./build.sh --version
```

Output should show:
```
FluxSharp v0.1.0
```

### 4. Optional: Add to PATH

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:$(pwd)"

# Reload shell
source ~/.bashrc
```

## Quick Test

```bash
# Copy example program
cp examples/hello.fsh .

# Build and run
./build.sh hello.fsh
```

Expected output:
```
🚀 FluxSharp Compiler v0.1.0 - Release Edition
══════════════════════════════════════════════════

ℹ️  Checking dependencies...
✅ Dependencies found: nasm, ld

ℹ️  Source file found: hello.fsh
✅ Compiler found: ./bin/fluxc
✅ Runtime found: ./lib/runtime.o

📝 Step 1: Compiling FluxSharp → Assembly...
  Source: hello.fsh
✅ Generated assembly: hello.asm

🔨 Step 2: Assembling → Object File...
✅ Generated object file: hello.o

🔗 Step 3: Linking → Executable...
✅ Generated executable: hello

🚀 Step 4: Running program
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Hello, FluxSharp!

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Program completed successfully
```

## Uninstalling

```bash
# Simply remove the directory
rm -rf fluxsharp-v0.1.0-linux-x64
```

## Troubleshooting

### "nasm command not found"
```bash
# Install nasm
sudo apt install nasm

# Verify
nasm -version
```

### "ld command not found"
```bash
# Install binutils
sudo apt install binutils

# Verify
ld --version
```

### "Permission denied" on build.sh
```bash
chmod +x build.sh
./build.sh hello.fsh
```

### Binary doesn't execute
Ensure you're on Linux x86-64:
```bash
uname -m  # Should output: x86_64
```

## Platform-Specific Notes

### Running on Kernel/Embedded System

For FluxGridOS kernel integration:

```bash
# Compile program to object file
./bin/fluxc compile user_program.fsh -o program.o

# Link with kernel
ld -r kernel.o program.o -o combined.o
```

### Docker Environment

```dockerfile
FROM ubuntu:22.04
RUN apt update && apt install -y nasm binutils
COPY fluxsharp-v0.1.0 /app
WORKDIR /app
```

## Next Steps

1. **Read the documentation**: `docs/LANGUAGE.md`
2. **Explore examples**: `examples/`
3. **Try the hello world**: `examples/hello.fsh`
4. **Create your program**: `myprogram.fsh`
5. **Compile and run**: `./build.sh myprogram.fsh`

## Getting Help

- 📖 Read: `README.md`
- 🔍 Reference: `docs/LANGUAGE.md`
- 🔒 Security: `docs/SECURITY.md`
- 📚 Examples: `examples/`

