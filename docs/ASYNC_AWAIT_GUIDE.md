# FluxGridOS Async/Await System Documentation

## Overview

FluxGridOS implements a lightweight, optimized async/await system inspired by C# but designed for minimal overhead in kernel-space environments. The system supports:

- **Async Functions**: Functions marked with `async` keyword
- **Await Operations**: Non-blocking waits using `await` keyword
- **Event Loop**: Central task scheduler and event dispatcher
- **Task Pool**: Parallel execution with worker threads
- **Promises**: Promise-like structures for async operations
- **Callbacks**: Event-driven programming model
- **Retry Logic**: Automatic retry with exponential backoff
- **Timeout Handling**: Deadline management for tasks
- **Performance Metrics**: Built-in monitoring and statistics

## Syntax

### Async Functions

```flux
// Declare an async function
public async uint fetch_data(string url) => {
    uint task_id = async_create_task();
    async_schedule_task(task_id);
    return task_id;
}

// Call with await (in async function context)
public async void caller() => {
    uint data_task = await fetch_data("http://example.com");
    long result = async_get_result(data_task);
}
```

### Await Expressions

```flux
// Await a function call
uint task = await async_read_file("/path/to/file");

// Await with direct result retrieval
long result = async_await_task(task);
```

## Core Components

### 1. Task Management

```flux
struct Task {
    uint id;          // Unique task identifier
    uint state;       // Current task state (Pending, Running, etc.)
    long result;      // Task result/return value
    long timeout_ms;  // Timeout in milliseconds
    long created_at;  // Creation timestamp
}
```

**Task States:**
- `Pending`: Waiting to be scheduled
- `Running`: Currently executing
- `Paused`: Temporarily suspended
- `Completed`: Finished successfully
- `Failed`: Execution failed

### 2. Event Loop

The event loop is the heart of the async system:

```flux
public void async_event_loop() => {
    while(true) {
        async_poll_events();      // Check for new events
        // Process completed tasks
        // Handle timeouts
        // Yield to other systems
    }
}
```

### 3. Task Queue

Circular buffer implementation for O(1) enqueue/dequeue:

```flux
struct TaskQueue {
    Task[128] tasks;   // Fixed-size circular buffer
    uint head;         // Write pointer
    uint tail;         // Read pointer
    uint count;        // Current task count
}
```

### 4. Task Pool

For parallel execution (lightweight threading):

```flux
struct TaskPool {
    uint worker_count;
    uint active_workers;
    TaskQueue pending_tasks;
    TaskQueue completed_tasks;
}
```

## Usage Examples

### Example 1: Simple Async Read

```flux
public async void read_file_example() => {
    uint task = async_read_file("/etc/config.ini");
    long result = async_await_task(task);
    
    if(result == 0) {
        serial_print("File read successful");
    }
}
```

### Example 2: Sequential Operations

```flux
public async void sequential_example() => {
    uint task1 = await fetch_data("http://api.example.com/users");
    uint task2 = await fetch_data("http://api.example.com/posts");
    
    long users = async_get_result(task1);
    long posts = async_get_result(task2);
}
```

### Example 3: Parallel Operations

```flux
public void parallel_example() => {
    task_pool_init(4);  // 4 worker threads
    
    uint task1 = task_pool_submit();
    uint task2 = task_pool_submit();
    uint task3 = task_pool_submit();
    
    task_pool_wait_all();  // Wait for completion
}
```

### Example 4: Retry with Exponential Backoff

```flux
public async uint robust_call() => {
    RetryPolicy policy;
    policy.max_retries = 3;
    policy.delay_ms = 500;
    policy.backoff_multiplier = 2;  // 500ms, 1s, 2s
    
    uint task = async_retry(policy);
    return task;
}
```

### Example 5: Timeout Handling

```flux
public async void timeout_example() => {
    uint task = async_network_request();
    uint timed_task = async_with_timeout(task, 5000);  // 5 second timeout
    
    long result = async_get_result(timed_task);
}
```

### Example 6: Event Callbacks

```flux
public void on_data_ready() => {
    serial_print("Data is ready!");
}

public void setup_callbacks() => {
    async_on_event(EventType::IO);  // Register callback
}
```

## Performance Characteristics

### Memory Overhead

- **Per Task**: ~40 bytes (id, state, result, timeout, created_at)
- **Event Loop**: ~1KB (task queue + event loop state)
- **Task Pool**: ~20KB per worker (scalable)

### Time Complexity

- **Task Creation**: O(1)
- **Task Scheduling**: O(1)
- **Task Await**: O(n) where n = task queue size (max 128)
- **Event Polling**: O(k) where k = active events

### Optimization Strategies

1. **Circular Buffer**: O(1) enqueue/dequeue
2. **Lazy Initialization**: Resources allocated on-demand
3. **Minimal Context Switching**: Stack-based task management
4. **Non-blocking I/O**: Async operations don't block CPU

## Integration with Kernel

### Boot Sequence

```flux
public void kernel_entry_from_lib(long magic, long boot_addr) => {
    init_logger();
    init_memory(0);
    init_gdt();
    init_interrupts();
    init_drivers();
    
    async_init();           // Initialize async system
    async_event_loop();     // Start main event loop
}
```

### Interrupt Handling

Async operations can be triggered by:
- **Timer Interrupts**: For timeouts and delays
- **I/O Interrupts**: For disk/network completion
- **GPU Interrupts**: For graphics operations
- **User Input**: For keyboard/mouse events

## Advanced Features

### Promise-Like Operations

```flux
struct Promise {
    uint state;
    long value;
    long error;
}

public void promise_example() => {
    Promise p;
    promise_resolve(p, 42);
}
```

### Performance Metrics

```flux
public void monitor_async() => {
    PerformanceMetrics metrics = async_get_metrics();
    
    // Log metrics
    // metrics.completed_tasks
    // metrics.failed_tasks
    // metrics.total_execution_time
    // metrics.average_task_duration
}
```

### Task Cancellation

```flux
public void cancel_tasks() => {
    uint task = async_create_task();
    async_schedule_task(task);
    
    async_cancel_task(task);        // Cancel single task
    async_cancel_all();              // Cancel all tasks
}
```

## Best Practices

1. **Always initialize async system** before using async functions
2. **Use task timeouts** to prevent deadlocks
3. **Monitor performance metrics** in production
4. **Prefer sequential operations** when order matters
5. **Use task pool** for CPU-bound parallel work
6. **Implement proper error handling** for failed tasks

## Error Handling

```flux
public async void error_handling_example() => {
    uint task = await fetch_data("http://api.example.com");
    
    if(async_is_completed(task)) {
        long result = async_get_result(task);
    } else {
        // Handle timeout or failure
        async_cancel_task(task);
    }
}
```

## Limitations and Future Enhancements

### Current Limitations
- Fixed task queue size (128 tasks)
- No dynamic worker thread creation
- Single event loop (no multi-core scheduling)

### Future Enhancements
- Multi-core event loop distribution
- Dynamic task queue resizing
- Advanced promise chaining
- Async streams and generators
- Better integration with GPU operations

