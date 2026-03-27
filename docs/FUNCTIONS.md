# FluxSharp Functions

Complete function definition and calling guide.

## Function Declaration

### Basic Function

```flux
public void PrintMessage() {
    // Function body
}
```

### Function with Parameters

```flux
public void Greet(string name) {
    // Use parameter name
}

public int Add(int a, int b) {
    return a + b;
}
```

### Function with Return Type

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

Accessible from outside:

```flux
public void PublicFunction() {
    // Can be called from anywhere
}
```

### Private Functions

Accessible only in module:

```
private void PrivateFunction() {
    // Only callable within module
}
```

### Static Functions

Class-level functions (can call without instance):

```flux
public static void StaticFunction() {
    // Class-level function
}

StaticFunction();  // Call directly
```

### Async Functions

For asynchronous operations:

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

```flux
public void PrintValue(int value) {
    // Use value
}

PrintValue(42);
```

### Multiple Parameters

```flux
public int Add(int a, int b) {
    return a + b;
}

int result = Add(5, 3);  // 8
```

### Parameter Types

All FluxSharp types supported:

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

```flux
public void DoWork() {
    // No return value
    return;
}
```

### Value Return

```flux
public int Calculate() {
    int result = 10 + 5;
    return result;
}
```

### Early Return

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

```flux
PrintMessage();         // No parameters
int sum = Add(5, 3);   // With parameters
```

### Method Calls

```flux
person.Greet();
obj.Calculate(10);
```

### Function Call Statement

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

