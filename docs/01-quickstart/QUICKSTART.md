# ⚡ Quick Start - 5 Minutes

Get your first FluxSharp program running in 5 minutes!

## Step 1: Build the Compiler (1 minute)

```bash
cd flux_compiler
cargo build --release
cd ..
```

This builds the FluxSharp compiler once. Future builds are fast!

## Step 2: Create Your First Program (1 minute)

Create a file called `hello.fsh`:

```rust
class Main {
    public void main() {
        print("Hello, FluxSharp!");
    }
}
```

That's it! This is a complete program.

## Step 3: Compile and Run (1 minute)

```bash
./build.sh hello.fsh
```

You should see:
```
Hello, FluxSharp!
```

🎉 **Success!** Your first FluxSharp program is running!

## Now Let's Add Variables (1 minute)

Update `hello.fsh`:

```rust
class Main {
    public void main() {
        int age = 25;
        string name = "Alice";
        
        print("Hello, ");
        print(name);
        print("!");
    }
}
```

Run it:
```bash
./build.sh hello.fsh
```

## One More Thing - Math (1 minute)

Update `hello.fsh`:

```rust
class Main {
    public void main() {
        int x = 10;
        int y = 20;
        int sum = x + y;
        
        print(sum);  // Prints: 30
    }
}
```

Run it:
```bash
./build.sh hello.fsh
```

## What You've Learned ✅

- ✅ How to create a FluxSharp program
- ✅ How to compile and run code
- ✅ Basic syntax with classes and main()
- ✅ Using variables
- ✅ Arithmetic operations
- ✅ print() function

## Next Steps

### Learn More:
- [TYPES.md](TYPES.md) - All data types
- [FUNCTIONS.md](FUNCTIONS.md) - Write your own functions
- [CLASSES.md](CLASSES.md) - Full OOP examples
- [CONTROL_FLOW.md](CONTROL_FLOW.md) - Loops and conditions

### Common Patterns:

**Declare variables:**
```rust
int count = 42;
float price = 9.99f;
string message = "Hello";
bool active = true;
```

**Write a function:**
```rust
public int multiply(int a, int b) {
    return a * b;
}
```

**Create a class:**
```rust
class MyClass {
    int value = 0;
    
    public void setValue(int v) {
        value = v;
    }
}
```

**Use a loop:**
```rust
for (int i = 0; i < 5; i = i + 1) {
    print(i);
}
```

## Troubleshooting

**Error: "Missing semicolon"**
→ Add `;` at the end of each statement

**Error: "Float literal"**
→ Use `3.14f` not `3.14` for floats

**Error: "Undefined variable"**
→ Declare variables before using them

**See [ERROR_GUIDE.md](ERROR_GUIDE.md) for all error types.**

## Key Rules to Remember

1. **Semicolons** - Every statement ends with `;`
2. **Float format** - Use `3.14f` not `3.14`
3. **Case sensitive** - `Main` ≠ `main`
4. **Declare first** - Variables before use
5. **Curly braces** - Match all `{` with `}`

## Try These Examples

```bash
# Hello world
./build.sh examples/hello.fsh

# Classes and objects
./build.sh examples/calculator.fsh

# Arrays
./build.sh examples/arrays.fsh
```

## Quick Reference

| Task | Syntax |
|------|--------|
| Print text | `print("Hello");` |
| Declare int | `int x = 10;` |
| Declare float | `float f = 3.14f;` |
| Declare string | `string s = "text";` |
| Add numbers | `int sum = a + b;` |
| Call function | `result = max(10, 20);` |
| Create object | `MyClass obj = new MyClass();` |

## Ready to Learn More?

→ Explore [all documentation](README.md)

→ Try [TYPES.md](TYPES.md) next

→ See [SYNTAX.md](SYNTAX.md) for detailed reference

---

**Happy coding!** 🚀

Having trouble? Check [ERROR_GUIDE.md](ERROR_GUIDE.md)

