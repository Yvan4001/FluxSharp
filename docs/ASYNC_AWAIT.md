# FluxSharp Async/Await

Asynchronous programming with async functions and await expressions.

## Async Functions

### Basic Async Function

```flux
async public void FetchData() {
    string response = await GetURL("http://example.com");
    return;
}
```

### Async Function with Return Type

```flux
async public int ProcessData(string url) {
    string data = await FetchData(url);
    int result = data.length();
    return result;
}
```

### Async Function with Parameters

```flux
async public void Download(string url, int timeout) {
    byte[1024] buffer;
    buffer = await FetchBytes(url);
    return;
}
```

## Await Expression

### Basic Await

```flux
async public void Main() {
    string response = await GetData();
    return;
}
```

### Await in Assignment

```flux
async public void Process() {
    string content = await FetchContent("url");
    int length = content.length();
    return;
}
```

### Multiple Awaits

```flux
async public void FetchAll() {
    string data1 = await GetData("url1");
    string data2 = await GetData("url2");
    string data3 = await GetData("url3");
    return;
}
```

### Await in Conditions

```flux
async public void ConditionalAwait(string url) {
    if (url.length() > 0) {
        string response = await FetchURL(url);
        ProcessResponse(response);
    }
    return;
}
```

## Async Statement

From grammar:

```
await_stmt = { await_keyword ~ ident ~ "(" ~ (expr ~ ("," ~ expr)*)? ~ ")" ~ ";" }
```

Syntax:

```flux
await FunctionName();
await FunctionName(arg1, arg2);
```

Examples:

```flux
await ProcessData();
await FetchURL("http://example.com");
await Download(filename, timeout);
```

## Declaring Async Functions

### Simple Async

```flux
async public void FetchData() {
    await GetURL("url");
    return;
}
```

### Async with Static

```flux
async public static void AsyncTask() {
    await SomeOperation();
    return;
}
```

### Async Private

```
async private void InternalOperation() {
    await PrivateTask();
    return;
}
```

## Using Async Functions

### Awaiting Async Functions

```flux
async public void Main() {
    await FetchData();
    return;
}
```

### Async in Loops

```flux
async public void ProcessAll(string[] urls) {
    for (int i = 0; i < 10; i++) {
        string response = await FetchURL(urls[i]);
        ProcessResponse(response);
    }
    return;
}
```

### Async with Conditionals

```flux
async public void ConditionalFetch(bool should_fetch) {
    if (should_fetch) {
        string data = await GetData();
        ProcessData(data);
    }
    return;
}
```

## Async Examples

### Simple Fetch

```flux
async public void FetchFile(string filename) {
    byte[4096] content;
    content = await ReadFile(filename);
    return;
}
```

### Download Multiple Files

```flux
async public void DownloadAll(string[] urls) {
    for (int i = 0; i < 10; i++) {
        byte[1024] data;
        data = await DownloadURL(urls[i]);
        SaveData(data, i);
    }
    return;
}
```

### Async with Error Handling

```flux
async public void SafeFetch(string url) {
    string response = await GetURL(url);
    if (response.length() > 0) {
        ProcessResponse(response);
    }
    return;
}
```

### Database Operations

```flux
async public int QueryDatabase(string query) {
    string result = await ExecuteQuery(query);
    int count = result.length();
    return count;
}
```

## Async Rules

1. **Can only await inside async functions**
   ```flux
   async public void Valid() {
       await Task();       // OK
   }
   
   public void Invalid() {
       // await Task();    // ERROR - not in async
   }
   ```

2. **Await must be on async operations**
   ```flux
   async public void Example() {
       string data = await GetData();  // OK
       int x = 10;                     // OK (not async)
   }
   ```

3. **Async functions return to caller**
   ```flux
   async public void Task() {
       await Operation();
       return;             // Returns from async
   }
   ```

## Keywords

### async Keyword

Marks function as asynchronous:

```flux
async public void TaskName() {
    // Async body
}
```

### await Keyword

Waits for async operation:

```flux
await AsyncFunction();
string result = await GetData();
```

## Execution Model

- Async functions can be called and awaited
- Await pauses execution until operation completes
- Multiple async operations can run concurrently
- Cooperative multitasking (no preemption)

Example:

```flux
async public void Main() {
    // Async operations here
    await FetchData1();     // Runs first
    await FetchData2();     // Runs after first completes
    await FetchData3();     // Runs after second completes
}
```

## Async Patterns

### Sequential Operations

```flux
async public void Sequential() {
    string data1 = await Fetch1();
    string data2 = await Fetch2();
    string data3 = await Fetch3();
    
    // All complete sequentially
    return;
}
```

### Independent Operations

```flux
async public void Independent() {
    // These could run concurrently if spawned separately
    await Task1();
    await Task2();
    await Task3();
    return;
}
```

### Conditional Async

```flux
async public void Conditional(bool flag) {
    if (flag) {
        await AsyncOperation();
    } else {
        SyncOperation();
    }
    return;
}
```

---

End of Language Documentation

