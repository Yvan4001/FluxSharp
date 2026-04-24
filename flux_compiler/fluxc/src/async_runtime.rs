/// Async/Await Module for FluxSharp
/// Implements asynchronous programming model with coroutines and event loop
/// 
/// Features:
/// - Async function declarations
/// - Await expressions
/// - Promise-based execution
/// - Event loop scheduling
/// - Coroutine state management
/// - Async exception handling

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct PromiseId {
    id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PromiseState {
    /// Promise is pending (not yet resolved)
    Pending,
    /// Promise resolved with success
    Resolved,
    /// Promise rejected with error
    Rejected,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Promise {
    pub id: PromiseId,
    pub state: PromiseState,
    pub value: Option<String>, // Result value
    pub error: Option<String>, // Error message
}

impl Promise {
    pub fn new(id: PromiseId) -> Self {
        Promise {
            id,
            state: PromiseState::Pending,
            value: None,
            error: None,
        }
    }

    #[allow(dead_code)]
    pub fn resolve(mut self, value: String) -> Self {
        self.state = PromiseState::Resolved;
        self.value = Some(value);
        self
    }

    #[allow(dead_code)]
    pub fn reject(mut self, error: String) -> Self {
        self.state = PromiseState::Rejected;
        self.error = Some(error);
        self
    }

    #[allow(dead_code)]
    pub fn is_pending(&self) -> bool {
        self.state == PromiseState::Pending
    }

    #[allow(dead_code)]
    pub fn is_resolved(&self) -> bool {
        self.state == PromiseState::Resolved
    }

    #[allow(dead_code)]
    pub fn is_rejected(&self) -> bool {
        self.state == PromiseState::Rejected
    }

    /// Safe access to promise value
    pub fn get_value(&self) -> Result<&str, &str> {
        match self.state {
            PromiseState::Resolved => {
                Ok(self.value.as_deref().unwrap_or(""))
            }
            PromiseState::Rejected => {
                Err(self.error.as_deref().unwrap_or("Unknown error"))
            }
            PromiseState::Pending => {
                Err("Promise still pending")
            }
        }
    }

    /// Generate safe access ASM code to check state before accessing value
    pub fn generate_safe_access(
        var_name: &str,
        result_offset: i32,
        catch_label: &str,
    ) -> String {
        let mut asm = String::new();
        asm.push_str(&format!("    ; Safe access to promise result -> {}\n", var_name));
        // rax must contain the slot index at this point
        asm.push_str("    imul rax, rax, 16\n");                              // slot * 16
        asm.push_str("    lea rbx, [promise_table]\n");
        asm.push_str("    add rbx, rax\n");
        asm.push_str("    mov rcx, qword [rbx]      ; state\n");
        asm.push_str("    cmp rcx, 1\n");
        asm.push_str(&format!("    jne {}\n", catch_label));                  // Not Resolved -> catch
        asm.push_str("    mov rax, qword [rbx + 8]  ; value ptr\n");
        asm.push_str(&format!("    mov qword [rbp-{}], rax  ; -> {}\n", result_offset, var_name));
        asm
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AsyncFunction {
    pub name: String,
    pub return_type: String,
    pub is_async: bool,
    pub parameters: Vec<(String, String)>, // (name, type)
}

impl AsyncFunction {
    #[allow(dead_code)]
    pub fn new(name: String, return_type: String, is_async: bool) -> Self {
        AsyncFunction {
            name,
            return_type,
            is_async,
            parameters: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_parameter(&mut self, name: String, param_type: String) {
        self.parameters.push((name, param_type));
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AwaitExpression {
    pub function_name: String,
    pub promise_id: Option<PromiseId>,
    pub result_var: Option<String>,
}

#[allow(dead_code)]
pub struct AsyncRuntime {
    promises: std::collections::HashMap<u64, Promise>,
    next_promise_id: u64,
    event_queue: VecDeque<u64>, // Queue of pending promise IDs
    async_functions: std::collections::HashMap<String, AsyncFunction>,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        AsyncRuntime {
            promises: std::collections::HashMap::new(),
            next_promise_id: 1,
            event_queue: VecDeque::new(),
            async_functions: std::collections::HashMap::new(),
        }
    }

    /// Register an async function
    pub fn register_async_function(&mut self, func: AsyncFunction) -> Result<(), String> {
        if self.async_functions.contains_key(&func.name) {
            return Err(format!("Async function '{}' already registered", func.name));
        }
        self.async_functions.insert(func.name.clone(), func);
        Ok(())
    }

    /// Create a new promise
    pub fn create_promise(&mut self) -> PromiseId {
        let id = self.next_promise_id;
        self.next_promise_id += 1;

        let promise_id = PromiseId { id };
        self.promises.insert(id, Promise::new(promise_id));
        self.event_queue.push_back(id);

        promise_id
    }

    /// Get a promise
    pub fn get_promise(&self, id: PromiseId) -> Option<&Promise> {
        self.promises.get(&id.id)
    }

    /// Resolve a promise
    pub fn resolve_promise(&mut self, id: PromiseId, value: String) -> Result<(), String> {
        if let Some(promise) = self.promises.get_mut(&id.id) {
            if !promise.is_pending() {
                return Err(format!("Promise {:?} already settled", id));
            }
            promise.state = PromiseState::Resolved;
            promise.value = Some(value);
            Ok(())
        } else {
            Err(format!("Promise {:?} not found", id))
        }
    }

    /// Reject a promise
    #[allow(dead_code)]
    pub fn reject_promise(&mut self, id: PromiseId, error: String) -> Result<(), String> {
        if let Some(promise) = self.promises.get_mut(&id.id) {
            if !promise.is_pending() {
                return Err(format!("Promise {:?} already settled", id));
            }
            promise.state = PromiseState::Rejected;
            promise.error = Some(error);
            Ok(())
        } else {
            Err(format!("Promise {:?} not found", id))
        }
    }

    /// Process next event in queue
    pub fn process_next_event(&mut self) -> Option<PromiseId> {
        self.event_queue.pop_front().map(|id| PromiseId { id })
    }

    /// Generate the promise state table in .bss/.data sections
    /// To be called ONCE in the global ASM header
    pub fn generate_promise_table(&self) -> String {
        let mut asm = String::new();
        asm.push_str("; === Promise State Table ===\n");
        asm.push_str("section .bss\n");
        // Table of MAX_PROMISES entries, each = 16 bytes:
        // [0..7]  = state (0=Pending, 1=Resolved, 2=Rejected)
        // [8..15] = pointer to value/error (string in .data)
        asm.push_str("    promise_table:    resq 2048  ; 128 max promises * 16 bytes\n");
        asm.push_str("    promise_count:    resq 1     ; Number of active promises\n");
        asm.push_str("section .text\n\n");
        asm
    }

    /// Generate async function declaration code
    pub fn generate_async_function(
        &self,
        name: &str,
        return_type: &str,
        is_async: bool,
        promise_slot: u64,
    ) -> Result<String, String> {
        if !is_async {
            return Err("Function is not async".to_string());
        }

        let slot_offset = promise_slot * 16;
        let mut asm = String::new();

        asm.push_str(&format!("; === async fn {} -> {} (slot {}) ===\n", name, return_type, promise_slot));
        asm.push_str(&format!("global {}\n{}:\n", name, name));

        // Standard prologue
        asm.push_str("    push rbp\n");
        asm.push_str("    mov rbp, rsp\n");

        // Mark the promise as Pending in the table
        asm.push_str(&format!(
            "    mov qword [promise_table + {}], 0  ; state = Pending\n",
            slot_offset
        ));
        asm.push_str(&format!(
            "    mov qword [promise_table + {}], 0  ; value = null\n",
            slot_offset + 8
        ));

        // The body of the function will be inserted here by compile_block

        // Epilogue: mark as Resolved + store the result (rax)
        asm.push_str(&format!("\n.{}_resolve:\n", name));
        asm.push_str(&format!(
            "    mov qword [promise_table + {}], 1  ; state = Resolved\n",
            slot_offset
        ));
        asm.push_str(&format!(
            "    mov qword [promise_table + {}], rax  ; value = rax\n",
            slot_offset + 8
        ));
        asm.push_str("    pop rbp\n");
        asm.push_str("    ret\n\n");

        Ok(asm)
    }

    /// Generate await expression code
    pub fn generate_await_expression(
        &self,
        var_name: &str,
        function_name: &str,
        promise_slot: u64,   // Index in promise_table (0..127)
        result_offset: i32,  // rbp offset to store the result
    ) -> Result<String, String> {
        if !self.async_functions.contains_key(function_name) {
            return Err(format!("Async function '{}' not found", function_name));
        }

        let slot_offset = promise_slot * 16; // 16 bytes per entry

        let mut asm = String::new();
        asm.push_str(&format!("; === await {}() -> {} ===\n", function_name, var_name));

        // 1. Save callee-saved registers BEFORE the call
        asm.push_str("    ; Save coroutine state\n");
        asm.push_str("    push rbx\n");
        asm.push_str("    push r12\n");
        asm.push_str("    push r13\n");
        asm.push_str("    push r14\n");
        asm.push_str("    push r15\n");

        // 2. Call the async function — it writes its state in promise_table
        asm.push_str(&format!("    call {}\n", function_name));
        // rax = promise_slot returned by the function

        // 3. Check the state via the TABLE (not via rax as a pointer!)
        asm.push_str(&format!("\n    ; Poll promise_table[{}] — no busy-wait\n", promise_slot));
        asm.push_str(&format!(".await_{}_check:\n", function_name));
        // Read state = promise_table[slot * 16 + 0]
        asm.push_str(&format!(
            "    mov rbx, qword [promise_table + {}]  ; state\n",
            slot_offset
        ));
        asm.push_str("    cmp rbx, 0\n");
        asm.push_str(&format!("    je .await_{}_pending\n", function_name));  // Pending
        asm.push_str("    cmp rbx, 2\n");
        asm.push_str(&format!("    je .await_{}_rejected\n", function_name)); // Rejected
        // -> Resolved: load the value
        asm.push_str(&format!(
            "    mov rax, qword [promise_table + {}]  ; value ptr\n",
            slot_offset + 8
        ));
        asm.push_str(&format!("    mov qword [rbp-{}], rax  ; -> {}\n", result_offset, var_name));
        asm.push_str(&format!("    jmp .await_{}_done\n", function_name));

        // Pending: yield to the event loop (no spin!)
        asm.push_str(&format!(".await_{}_pending:\n", function_name));
        asm.push_str("    ; Yield to event loop — run other tasks\n");
        asm.push_str("    call _fsh_event_loop_tick\n");
        asm.push_str(&format!("    jmp .await_{}_check\n", function_name));

        // Rejected: branch to error handler
        asm.push_str(&format!(".await_{}_rejected:\n", function_name));
        asm.push_str(&format!(
            "    mov rax, qword [promise_table + {}]  ; error ptr\n",
            slot_offset + 8
        ));
        asm.push_str("    mov rdi, rax\n");
        asm.push_str("    call _fsh_print_str  ; Print the error\n");
        asm.push_str("    mov rax, 0          ; Null value if rejected\n");
        asm.push_str(&format!("    mov qword [rbp-{}], rax\n", result_offset));

        asm.push_str(&format!(".await_{}_done:\n", function_name));
        // 4. Restore registers in reverse order
        asm.push_str("    ; Restore coroutine state\n");
        asm.push_str("    pop r15\n");
        asm.push_str("    pop r14\n");
        asm.push_str("    pop r13\n");
        asm.push_str("    pop r12\n");
        asm.push_str("    pop rbx\n\n");

        Ok(asm)
    }

    /// Generate event loop code
    pub fn generate_event_loop(&self) -> String {
        let mut asm = String::new();

        // _fsh_event_loop_tick : called on each await Pending
        // Executes ONE pending task and returns
        asm.push_str("; === Event Loop (cooperative scheduler) ===\n");
        asm.push_str("global _fsh_event_loop_tick\n");
        asm.push_str("_fsh_event_loop_tick:\n");
        asm.push_str("    push rbp\n");
        asm.push_str("    mov rbp, rsp\n");
        // Read queue head
        asm.push_str("    mov rax, qword [event_queue_head]\n");
        asm.push_str("    cmp rax, qword [event_queue_tail]\n");
        asm.push_str("    je .tick_empty\n");          // Empty queue -> return immediately
        // Pop task
        asm.push_str("    mov rcx, qword [event_queue + rax*8]\n");
        asm.push_str("    inc rax\n");
        asm.push_str("    and rax, 127\n");            // Ring buffer mod 128
        asm.push_str("    mov qword [event_queue_head], rax\n");
        // Call task handler
        asm.push_str("    call rcx\n");
        asm.push_str(".tick_empty:\n");
        asm.push_str("    pop rbp\n");
        asm.push_str("    ret\n\n");

        // Queue data (ring buffer of 128 function pointers)
        asm.push_str("section .bss\n");
        asm.push_str("    event_queue:      resq 128  ; 128 task pointers\n");
        asm.push_str("    event_queue_head: resq 1\n");
        asm.push_str("    event_queue_tail: resq 1\n");
        asm.push_str("section .text\n\n");

        asm
    }

    /// Get list of async functions
    pub fn async_functions(&self) -> Vec<&AsyncFunction> {
        self.async_functions.values().collect()
    }

    /// Get all promises
    pub fn promises(&self) -> Vec<&Promise> {
        self.promises.values().collect()
    }

    /// Get pending promises count
    pub fn pending_count(&self) -> usize {
        self.promises
            .values()
            .filter(|p| p.is_pending())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_promise() {
        let mut runtime = AsyncRuntime::new();
        let promise_id = runtime.create_promise();
        
        assert!(runtime.get_promise(promise_id).is_some());
        assert!(runtime.get_promise(promise_id).unwrap().is_pending());
    }

    #[test]
    fn test_resolve_promise() {
        let mut runtime = AsyncRuntime::new();
        let promise_id = runtime.create_promise();
        
        let result = runtime.resolve_promise(promise_id, "success".to_string());
        assert!(result.is_ok());
        
        let promise = runtime.get_promise(promise_id).unwrap();
        assert!(promise.is_resolved());
        assert_eq!(promise.value, Some("success".to_string()));
    }

    #[test]
    fn test_reject_promise() {
        let mut runtime = AsyncRuntime::new();
        let promise_id = runtime.create_promise();
        
        let result = runtime.reject_promise(promise_id, "error".to_string());
        assert!(result.is_ok());
        
        let promise = runtime.get_promise(promise_id).unwrap();
        assert!(promise.is_rejected());
        assert_eq!(promise.error, Some("error".to_string()));
    }

    #[test]
    fn test_register_async_function() {
        let mut runtime = AsyncRuntime::new();
        let func = AsyncFunction::new(
            "FetchData".to_string(),
            "string".to_string(),
            true,
        );
        
        assert!(runtime.register_async_function(func).is_ok());
        assert_eq!(runtime.async_functions().len(), 1);
    }

    #[test]
    fn test_duplicate_async_function() {
        let mut runtime = AsyncRuntime::new();
        let func1 = AsyncFunction::new(
            "FetchData".to_string(),
            "string".to_string(),
            true,
        );
        let func2 = AsyncFunction::new(
            "FetchData".to_string(),
            "int".to_string(),
            true,
        );
        
        runtime.register_async_function(func1).unwrap();
        let result = runtime.register_async_function(func2);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_queue() {
        let mut runtime = AsyncRuntime::new();
        let promise_id1 = runtime.create_promise();
        let promise_id2 = runtime.create_promise();
        
        let first = runtime.process_next_event();
        assert_eq!(first, Some(promise_id1));
        
        let second = runtime.process_next_event();
        assert_eq!(second, Some(promise_id2));
        
        let third = runtime.process_next_event();
        assert_eq!(third, None);
    }
}

