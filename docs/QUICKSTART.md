# FluxSharp Quick Start

Get started with FluxSharp in 5 minutes.

## Installation

FluxSharp is already set up in your project. No additional installation needed.

## Your First Program

The `main.fsh` file demonstrates key features:

```flux
// Classes with new keyword
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}

public static void Main() {
    Calculator calc = new Calculator();
    int result = calc.Add(5, 3);
    return;
}
```

## Build and Run

### One Command to Build and Execute Everything

```bash
./build.sh
```

This single command:
1. ✅ Compiles the Rust compiler
2. ✅ Compiles your `main.fsh` code
3. ✅ Creates the executable
4. ✅ Executes `./program` to show results

## Key Features

### Classes with `new`
```flux
public class Person {
    public string name;
    
    public void Greet() {
        print(name);
        return;
    }
}

Person p = new Person();
p.name = "Alice";
p.Greet();
```

### Array Access
```flux
int[10] numbers;
numbers[0] = 10;
numbers[1] = 20;

int first = numbers[0];    // Read array element
int second = numbers[1];
```

### Functions with Parameters
```flux
public int Add(int a, int b) {
    return a + b;
}

int result = Add(5, 3);
```

### Control Flow
```flux
for (int i = 0; i < 10; i++) {
    print("Iteration");
}

if (result > 10) {
    print("Large");
} else {
    print("Small");
}
```

## Editing Your Program

Edit `main.fsh` with your favorite editor:

```bash
nano main.fsh
```

Then rebuild and run:

```bash
./build.sh
```

## Example Patterns

### Constants
```flux
const int MAX_SIZE = 100;
const double PI = 3.14159;
```

### Structs
```flux
public struct Point {
    int x;
    int y;
}
```

### Variables with Types
```flux
int count = 10;
double temperature = 23.5;
string name = "FluxSharp";
bool active = true;
```

### Method Calls
```flux
Calculator calc = new Calculator();
int sum = calc.Add(5, 3);
```

## Workflow

1. **Edit code** - Modify `main.fsh`
2. **Build and run** - Execute `./build.sh`
3. **See output** - Program runs and displays results
4. **Iterate** - Go back to step 1

## Demo Features

The default `main.fsh` demonstrates:
- **Constants** - Global constant values
- **Structs** - Data structures with fields
- **Classes** - Object-oriented programming
- **Objects** - Creating instances with `new`
- **Arrays** - Fixed-size collections
- **Array Access** - Reading and writing elements
- **Functions** - Parameter passing and return values
- **Methods** - Class member functions
- **Control Flow** - for loops, if/else statements

## Next Steps

1. **Run the demo** - `./build.sh`
2. **Read syntax** - Check [SYNTAX.md](SYNTAX.md)
3. **Learn features** - Review other `.md` files in docs/
4. **Experiment** - Modify `main.fsh` and rebuild

## Available Commands

| Command | Does |
|---------|------|
| `./build.sh` | Complete build and run |
| `make quickstart` | Alternative build (same as above) |
| `make build` | Build only (no run) |
| `make clean` | Remove generated files |

---

**You're ready! Run `./build.sh` to get started!** 🚀

