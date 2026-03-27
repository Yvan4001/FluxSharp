// Example: Async/Await program for FluxSharp v2.0+
// Demonstrates concurrent task execution

// Global event counter
int event_counter = 0;

// Event types
int TIMER_EVENT = 1;
int IO_EVENT = 2;
int CUSTOM_EVENT = 3;

// Async task 1: Process with delay
async void task_one() {
    serial_print("Task 1: Starting");
    await delay(100);
    serial_print("Task 1: After delay");
    await delay(50);
    serial_print("Task 1: Done");
}

// Async task 2: Another concurrent task
async void task_two() {
    serial_print("Task 2: Starting");
    await delay(150);
    serial_print("Task 2: Processing");
    await io_operation();
    serial_print("Task 2: Done");
}

// Async task 3: Quick task
async void task_three() {
    serial_print("Task 3: Starting");
    int value = await compute_async();
    serial_print("Task 3: Computed value = ");
    serial_print(value);
    serial_print("Task 3: Done");
}

// Simulate delay operation
async void delay(int milliseconds) {
    // This would normally interact with a timer interrupt
    // Compiler generates: await_operation(current_task, TIMER_EVENT)
    // When timer fires: event_arrived(TIMER_EVENT) wakes this task
    await TIMER_EVENT;
}

// Simulate I/O operation
async void io_operation() {
    serial_print("Task 2: Performing I/O");
    // Wait for I/O completion event
    await IO_EVENT;
    serial_print("Task 2: I/O complete");
}

// Async compute function
async int compute_async() {
    serial_print("Computing...");
    int result = 0;
    
    for (int i = 0; i < 10; i++) {
        result = result + i;
        await 0;  // Yield to let other tasks run
    }
    
    return result;
}

// Main entry point - spawn all tasks
async void main() {
    serial_print("=== Async/Await Demo ===");
    serial_print("Spawning 3 concurrent tasks...");
    
    // Spawn all tasks
    spawn_task(task_one);
    spawn_task(task_two);
    spawn_task(task_three);
    
    serial_print("All tasks spawned. Starting scheduler...");
    
    // Main task continues
    await all_tasks_complete();
    
    serial_print("=== All tasks completed ===");
    serial_print("Demo finished");
}

// Wait for all tasks to complete
async void all_tasks_complete() {
    // Poll until all tasks done
    // In real implementation, would use proper synchronization
    for (int i = 0; i < 100; i++) {
        await TIMER_EVENT;
    }
}

// Helper: Wait for all spawned tasks
async void wait_all() {
    serial_print("Waiting for all tasks...");
    await CUSTOM_EVENT;
}

// Simulated timer interrupt handler
void on_timer_tick() {
    event_counter = event_counter + 1;
    event_arrived(TIMER_EVENT);
}

// Simulated I/O completion handler
void on_io_complete() {
    event_arrived(IO_EVENT);
}

// For demonstration of sequential vs async execution:

// Sequential version (v1.0 - blocks while waiting)
void sequential_demo() {
    serial_print("Sequential execution:");
    serial_print("Task 1 start");
    slow_operation(100);  // Blocks for 100ms
    serial_print("Task 1 done");
    
    serial_print("Task 2 start");
    slow_operation(100);  // Blocks for 100ms
    serial_print("Task 2 done");
    
    serial_print("Task 3 start");
    slow_operation(100);  // Blocks for 100ms
    serial_print("Task 3 done");
    // Total: 300ms
}

void slow_operation(int time) {
    // Wastes CPU for 'time' ms
    for (int i = 0; i < time * 1000; i++) {
        // Spin
    }
}

// Async version (v2.0+ - concurrent execution)
async void async_demo() {
    serial_print("Async execution:");
    spawn_task(async_task_1);
    spawn_task(async_task_2);
    spawn_task(async_task_3);
    await all_done();
    // Total: ~100ms (concurrent)
}

async void async_task_1() {
    serial_print("Async Task 1 start");
    await delay(100);
    serial_print("Async Task 1 done");
}

async void async_task_2() {
    serial_print("Async Task 2 start");
    await delay(100);
    serial_print("Async Task 2 done");
}

async void async_task_3() {
    serial_print("Async Task 3 start");
    await delay(100);
    serial_print("Async Task 3 done");
}

async void all_done() {
    // Wait for all tasks
    await CUSTOM_EVENT;
}

// Error handling example with async
async void error_handling_demo() {
    serial_print("Error handling demo");
    
    try {
        int result = await risky_async_operation();
        serial_print("Success: ");
        serial_print(result);
    } catch (error_string) {
        serial_print("Caught error: ");
        serial_print(error_string);
        serial_print("Attempting recovery...");
        await recovery_operation();
        serial_print("Recovery complete");
    }
}

async int risky_async_operation() {
    serial_print("Attempting risky operation");
    await delay(50);
    
    int value = 42;
    if (value > 50) {
        throw "Value too high";
    }
    
    return value;
}

async void recovery_operation() {
    serial_print("Running recovery");
    await delay(100);
    serial_print("Recovery done");
}

// Priority example
async void high_priority_task() priority(9) {
    serial_print("High priority task");
}

async void normal_task() priority(5) {
    serial_print("Normal priority task");
}

async void low_priority_task() priority(1) {
    serial_print("Low priority task");
}

