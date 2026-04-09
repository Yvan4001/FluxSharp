//! FluxSharp Compiler Library
//! 
//! This module provides the core compilation interface for the FluxSharp compiler.

pub struct CompileResult {
    pub success: bool,
    pub assembly: String,
    pub errors: String,
}

impl CompileResult {
    /// Create a new CompileResult
    pub fn new(success: bool, assembly: String, errors: String) -> Self {
        CompileResult { success, assembly, errors }
    }
    
    /// Get success status
    pub fn success(&self) -> bool {
        self.success
    }

    /// Get generated assembly code
    pub fn assembly(&self) -> &str {
        &self.assembly
    }

    /// Get error messages
    pub fn errors(&self) -> &str {
        &self.errors
    }
}

pub fn get_version() -> String {
    "FluxSharp Compiler v0.1.0".to_string()
}

/// Validate FluxSharp syntax
/// 
/// # Arguments
/// * `source_code` - The FluxSharp source code to validate
/// 
/// # Returns
/// Error message if syntax is invalid, empty string if valid
pub fn validate_syntax(source_code: &str) -> String {
    if source_code.is_empty() {
        return "Error: Empty source code".to_string();
    }

    // Basic syntax checks
    let mut errors = Vec::new();

    // Check for balanced braces
    let open_braces = source_code.matches('{').count();
    let close_braces = source_code.matches('}').count();
    if open_braces != close_braces {
        errors.push(format!(
            "Unbalanced braces: {} '{{' but {} '}}'",
            open_braces, close_braces
        ));
    }

    // Check for balanced parentheses
    let open_parens = source_code.matches('(').count();
    let close_parens = source_code.matches(')').count();
    if open_parens != close_parens {
        errors.push(format!(
            "Unbalanced parentheses: {} '(' but {} ')'",
            open_parens, close_parens
        ));
    }

    // Check for Main class
    if !source_code.contains("class Main") {
        errors.push("Main class not found".to_string());
    }

    // Check for main method
    if !source_code.contains("void main()") && !source_code.contains("void main ()") {
        errors.push("main() method not found".to_string());
    }

    errors.join("\n")
}

/// Get list of available FluxSharp keywords and built-in functions
pub fn get_builtins() -> String {
    r#"
Keywords:
- int, float, double, string, bool, void
- class, public, private, async
- if, else, for, while, break, continue
- return, new, using, import

Built-in Functions:
- print(value)
- serial_print(value)
- abs(x)
- max(a, b)
- min(a, b)
- pow(base, exp)
- sqrt(x)
- floor(x)
- ceil(x)
- round(x)

Math Constants:
- PI (3.14159...)
- E (2.71828...)
"#.to_string()
}

pub fn get_example_code() -> String {
    r#"public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Multiply(int a, int b) {
        return a * b;
    }
}

public class Main {
    public void main() {
        Calculator calc = new Calculator();
        
        print("5 + 3 = ");
        print(calc.Add(5, 3));
        
        print("10 * 2 = ");
        print(calc.Multiply(10, 2));
    }
}"#.to_string()
}
