# Getting Started with FluxSharp

## 5 Minutes to Your First Program

### Step 1: Build the Compiler (30 seconds)

```bash
cd flux_compiler
cargo build --release
cd ..
```

**Result:** Compiler ready at `flux_compiler/target/release/fluxc`

---

### Step 2: Create Your Program (1 minute)

Create a file called `hello.fsh`:

```flux
void main() {
    serial_print("Hello, FluxSharp!");
}
```

---

### Step 3: Compile (1 minute)

```bash
./flux_compiler/target/release/fluxc compile hello.fsh -o hello
```

**What happens:**
1. Parses your code
2. Generates assembly (`hello.asm`)
3. Assembles to object code (`hello.o`)
4. Links with runtime
5. Creates executable (`hello`)

---

### Step 4: Run (10 seconds)

```bash
./hello
```

**Output:**
```
Hello, FluxSharp!
```

---

## 🎉 Congratulations!

You just compiled your first FluxSharp program!

---

## Next: Learn More

### Want to do math?
```flux
void main() {
    int x = 10;
    int y = 3;
    int sum = x + y;
    serial_print(sum);  // Output: 13
}
```

### Want to loop?
```flux
void main() {
    int i = 0;
    while (i < 5) {
        serial_print(i);
        i = i + 1;
    }
    // Output: 0 1 2 3 4
}
```

### Want to use functions?
```flux
void greet(string name) {
    serial_print("Hello, ");
    serial_print(name);
}

void main() {
    greet("World");
}
```

---

## Common Commands

```bash
# Compile only (creates .asm and .o)
fluxc compile program.fsh

# Compile with output binary
fluxc compile program.fsh -o program

# Compile and run immediately
fluxc compile program.fsh -o program --run

# Compile multiple files
fluxc compile --all src/ -o program
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Command not found | Run `cargo build --release` first |
| Syntax Error | Check braces {} and semicolons |
| File not found | Make sure file ends with .fsh |
| No output | Check for print/serial_print calls |

---

## Next Steps

1. ✅ **Done:** Run your first program
2. **Next:** Study `main.fsh` for more examples
3. **Then:** Read LANGUAGE_GUIDE.md to learn syntax
4. **Finally:** Write your own programs!

---

See **LANGUAGE_GUIDE.md** for complete reference.

