# FluxSharp Functions

Complete function definition and calling guide.

## Function Declaration

### Basic Function

A simple function with no parameters and no return value.

```flux
public void PrintMessage() {
    // Function body
}
```

### Function with Parameters

Functions can accept parameters to receive data from the caller. Parameters must have explicit types.

```flux
public void Greet(string name) {
    // Use parameter name
}

public int Add(int a, int b) {
    return a + b;
}
```

### Function with Return Type

Functions can return a value to the caller using the `return` statement.

```flux
public int GetNumber() {
    return 42;
}

public string GetName() {
    return "Alice";
}

public void DoNothing() {
    return;
}
```

## Function Modifiers

### Public Functions

Public functions can be called from anywhere. They form the public interface of your module.

```flux
public void PublicFunction() {
    // Can be called from anywhere
}
```

### Private Functions

Private functions are only accessible within the current module. Use them for internal helper functions.

```
private void PrivateFunction() {
    // Only callable within module
}
```

### Static Functions

Static functions belong to the class itself, not to instances. Call them using the class name without creating an instance.

```flux
public static void StaticFunction() {
    // Class-level function
}

StaticFunction();  // Call directly
```

### Async Functions

Async functions enable non-blocking operations. They can use `await` to pause and resume execution.

```flux
async public void FetchData() {
    string response = await GetURL("http://example.com");
    return;
}
```

## Function Syntax

### Signature

```
[visibility] [static] [async] return_type function_name(param_list) {
    body
}
```

Examples:
```flux
public void Simple() { }

public static int Calculate(int x, int y) {
    return x + y;
}

async private void Fetch(string url) {
    await GetData(url);
}
```

## Parameters

### Single Parameter

Pass one value to a function. The parameter is available inside the function body.

```flux
public void PrintValue(int value) {
    // Use value
}

PrintValue(42);
```

### Multiple Parameters

Functions can accept multiple parameters. Each parameter must have an explicit type.

```flux
public int Add(int a, int b) {
    return a + b;
}

int result = Add(5, 3);  // 8
```

### Parameter Types

All FluxSharp types are supported as parameters.

```flux
public void Example(
    int number,
    string text,
    double value,
    bool flag,
    int[100] array
) {
    // Use parameters
}
```

## Return Statements

### Void Return

Functions that return nothing use `void` as the return type. The `return;` statement exits the function.

```flux
public void DoWork() {
    // No return value
    return;
}
```

### Value Return

Functions can return a value to the caller. The value must match the declared return type.

```flux
public int Calculate() {
    int result = 10 + 5;
    return result;
}
```

### Early Return

Use `return` to exit the function immediately before the end of the body.

```flux
public int Validate(int value) {
    if (value < 0) {
        return -1;      // Early exit
    }
    return value;
}
```

## Function Calls

### Simple Call

Call a function without parameters.

```flux
PrintMessage();         // No parameters
int sum = Add(5, 3);   // With parameters
```

### Method Calls

Call methods on objects using the dot notation.

```flux
person.Greet();
obj.Calculate(10);
```

### Function Call Statement

A function call can be a statement on its own line (the return value is ignored if there is one).

```flux
PrintMessage();
PrintInt(42);
DoWork();
```

## Function Body

Statements in function:

```flux
public void Process() {
    int x = 10;
    int y = 20;
    int sum = x + y;
    
    if (sum > 15) {
        PrintInt(sum);
    }
    
    return;
}
```

## Entry Point

### Main Function

```flux
public static void Main() {
    // Program entry point
    PrintMessage();
    return;
}
```

Must be:
- Public and static
- Named `Main`
- Return void
- No parameters

## Async Functions

### Async Declaration

```flux
async public void FetchURL(string url) {
    string response = await GetData(url);
    return;
}
```

### Await Expression

```flux
async public void Process() {
    int result = await Calculate(10);
    return;
}
```

### Using Async Functions

```flux
async public void DoWork() {
    // Can only await in async context
    await FetchURL("http://example.com");
    return;
}
```

## Function Examples

### Simple Function

```flux
public void PrintHello() {
    PrintLine("Hello");
    return;
}
```

### Function with Logic

```flux
public int Maximum(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}
```

### Function with Multiple Parameters

```flux
public void PrintSum(int x, int y) {
    int total = x + y;
    PrintInt(total);
    return;
}

PrintSum(5, 3);  // Prints 8
```

### Async Function Example

```flux
async public void DownloadFile(string url) {
    byte[1024] buffer;
    byte[1024] data = await FetchURL(url);
    return;
}
```

## Function Scope

Variables in function are local:

```flux
public void Example() {
    int x = 10;         // Local to function
    {
        int y = 20;     // Local to block
        // Can use both x and y
    }
    // y not available here
    // x still available
}
```

## Function Visibility

- **Public**: Callable from anywhere
- **Private**: Callable only in module
- **No modifier**: Default visibility

```flux
public void PublicFunc() { }
private void PrivateFunc() { }
```

## Recursive Functions

Functions can call themselves:

```flux
public int Factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * Factorial(n - 1);
}
```

## Function Declaration Syntax

From grammar:

```
function = {
    !"class" ~ !"struct" ~
    "public"? ~ "static"? ~ "private"? ~
    async_keyword? ~
    (type_ident | ident) ~ ident ~
    "(" ~ param_list? ~ ")" ~
    function_body
}
```

This means:
- Optional visibility/modifiers
- Optional async keyword
- Required return type
- Required function name
- Optional parameters
- Required body

---

Next: Read [CLASSES.md](CLASSES.md)

