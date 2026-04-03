/// Bounds Checking Module for FluxSharp
/// Implements automatic array bounds validation at runtime
/// 
/// This module tracks array declarations and generates bounds checking code
/// for all array access operations, preventing buffer overflows.

use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ArrayInfo {
    pub name: String,
    pub element_type: String,
    pub size: usize,
    pub dimensions: Vec<usize>, // For multi-dimensional arrays
}

#[allow(dead_code)]
pub struct BoundsChecker {
    /// Maps array name to array metadata
    arrays: HashMap<String, ArrayInfo>,
}

impl BoundsChecker {
    pub fn new() -> Self {
        BoundsChecker {
            arrays: HashMap::new(),
        }
    }

    /// Register an array declaration
    pub fn register_array(&mut self, name: String, array_type: String, size: usize) {
        let info = ArrayInfo {
            name: name.clone(),
            element_type: array_type,
            size,
            dimensions: vec![size],
        };
        self.arrays.insert(name, info);
    }

    /// Register a multi-dimensional array
    #[allow(dead_code)]
    pub fn register_multi_array(&mut self, name: String, array_type: String, dimensions: Vec<usize>) {
        let total_size = dimensions.iter().product();
        let info = ArrayInfo {
            name: name.clone(),
            element_type: array_type,
            size: total_size,
            dimensions,
        };
        self.arrays.insert(name, info);
    }

    /// Get array info if exists
    pub fn get_array(&self, name: &str) -> Option<&ArrayInfo> {
        self.arrays.get(name)
    }

    /// Generate assembly code for bounds checking
    pub fn generate_bounds_check(
        &self,
        array_name: &str,
        index_expr: &str,
        stack_offset: i64,
    ) -> Result<String, String> {
        if let Some(array_info) = self.get_array(array_name) {
            let mut asm = String::new();
            
            // Load index into RAX
            asm.push_str(&format!(
                "; === BOUNDS CHECK for {}[{}] ===\n",
                array_name, index_expr
            ));
            asm.push_str("    ; Load index value into RAX\n");
            
            // Parse index (could be literal or variable)
            if let Ok(idx) = index_expr.parse::<i64>() {
                asm.push_str(&format!("    mov rax, {}\n", idx));
            } else {
                // It's a variable - load it from stack
                asm.push_str(&format!(
                    "    mov rax, qword [rbp-{}] ; Load {} variable\n",
                    stack_offset, index_expr
                ));
            }
            
            // Check if index < 0
            asm.push_str("    ; Check: index >= 0\n");
            asm.push_str("    cmp rax, 0\n");
            asm.push_str(&format!(
                "    jl .bounds_error_{}\n",
                array_name
            ));
            
            // Check if index < array size
            asm.push_str(&format!(
                "    ; Check: index < {}\n",
                array_info.size
            ));
            asm.push_str(&format!("    cmp rax, {}\n", array_info.size));
            asm.push_str(&format!(
                "    jge .bounds_error_{}\n",
                array_name
            ));
            
            asm.push_str(&format!(
                "    ; ✅ Index within bounds for {}\n",
                array_name
            ));
            
            Ok(asm)
        } else {
            Err(format!("Array '{}' not registered for bounds checking", array_name))
        }
    }

    /// Generate the bounds error handler for a specific array
    #[allow(dead_code)]
    pub fn generate_error_handler(&self, array_name: &str) -> String {
        let mut asm = String::new();
        
        if let Some(array_info) = self.get_array(array_name) {
            asm.push_str(&format!(".bounds_error_{}:\n", array_name));
            asm.push_str(&format!(
                "    ; ❌ Array bounds error: {} (max size: {})\n",
                array_name, array_info.size
            ));
            asm.push_str("    mov rdi, 1       ; Exit code 1\n");
            asm.push_str("    mov rax, 60      ; syscall: exit\n");
            asm.push_str("    syscall\n");
            asm.push_str(&format!("    jmp .bounds_error_{}\n\n", array_name));
        }
        
        asm
    }

    /// Check if array is registered
    pub fn has_array(&self, name: &str) -> bool {
        self.arrays.contains_key(name)
    }

    /// Get all registered arrays
    pub fn list_arrays(&self) -> Vec<(&str, usize)> {
        self.arrays
            .iter()
            .map(|(name, info)| (name.as_str(), info.size))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_array() {
        let mut checker = BoundsChecker::new();
        checker.register_array("numbers".to_string(), "int".to_string(), 10);
        
        assert!(checker.has_array("numbers"));
        let array = checker.get_array("numbers").unwrap();
        assert_eq!(array.size, 10);
        assert_eq!(array.element_type, "int");
    }

    #[test]
    fn test_bounds_check_generation() {
        let mut checker = BoundsChecker::new();
        checker.register_array("arr".to_string(), "int".to_string(), 100);
        
        let check = checker.generate_bounds_check("arr", "5", 8).unwrap();
        assert!(check.contains("cmp rax, 0"));
        assert!(check.contains("cmp rax, 100"));
        assert!(check.contains("bounds_error_arr"));
    }

    #[test]
    fn test_unregistered_array() {
        let checker = BoundsChecker::new();
        let result = checker.generate_bounds_check("unknown", "0", 8);
        assert!(result.is_err());
    }

    #[test]
    fn test_multi_dimensional_array() {
        let mut checker = BoundsChecker::new();
        checker.register_multi_array(
            "matrix".to_string(),
            "int".to_string(),
            vec![10, 20],
        );
        
        assert!(checker.has_array("matrix"));
        let array = checker.get_array("matrix").unwrap();
        assert_eq!(array.size, 200);
    }
}

