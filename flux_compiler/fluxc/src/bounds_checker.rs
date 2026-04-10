/// Bounds Checking Module for FluxSharp
/// Implements automatic array bounds validation at runtime
/// 
/// This module tracks array declarations and generates bounds checking code
/// for all array access operations, preventing buffer overflows.

use std::collections::HashMap;
use anyhow::{Result, bail};
use pest::iterators::Pair;
use crate::exception_handler::{FluxValue, SymbolTable};
use crate::Rule;

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

pub fn eval_expr(pair: Pair<Rule>, variables: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    let expr_str = pair.as_str().trim();
    
    // Try direct literal parsing first
    if expr_str.starts_with('"') && expr_str.ends_with('"') {
        // String literal
        let str_content = &expr_str[1..expr_str.len()-1];
        return Ok(FluxValue::Str(str_content.to_string()));
    }
    
    if expr_str == "true" {
        return Ok(FluxValue::Int(1));
    }
    if expr_str == "false" {
        return Ok(FluxValue::Int(0));
    }
    
    // Try to parse as float
    if expr_str.ends_with('f') {
        let trimmed = &expr_str[..expr_str.len()-1];
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(FluxValue::Float(f));
        }
    }
    
    // Try to parse as int
    if let Ok(n) = expr_str.parse::<i64>() {
        return Ok(FluxValue::Int(n));
    }
    
    // Try to parse as double
    if let Ok(f) = expr_str.parse::<f64>() {
        return Ok(FluxValue::Float(f));
    }
    
    // Check if it's a variable
    if !expr_str.contains('+') && !expr_str.contains('-') && !expr_str.contains('*') && !expr_str.contains('/') {
        if let Some(val) = variables.get(expr_str) {
            return Ok(val.clone());
        }
    }
    
    // If all simple parsing failed, try the hierarchical descent
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| anyhow::anyhow!("Invalid expression"))?;

    match first.as_rule() {
        Rule::postfix => {
            // Descend into postfix: postfix = unary ~ (...)
            let mut postfix_inner = first.into_inner();
            let unary = postfix_inner.next().ok_or_else(|| anyhow::anyhow!("Invalid postfix"))?;
            // We ignore the rest of postfix operators for now (function calls, array access, member access)
            eval_expr(unary, variables)
        }
        Rule::unary => {
            // Descend into unary: unary = unary_op? ~ primary ~ (...)
            let mut unary_inner = first.into_inner();
            // Skip unary_op if present
            let next = unary_inner.next().ok_or_else(|| anyhow::anyhow!("Invalid unary"))?;
            if next.as_rule() == Rule::primary {
                eval_expr(next, variables)
            } else {
                // It's a unary_op, so skip and get the primary
                let primary = unary_inner.next().ok_or_else(|| anyhow::anyhow!("Invalid unary"))?;
                eval_expr(primary, variables)
            }
        }
        Rule::primary => {
            // Primary can contain various literals
            let mut primary_inner = first.into_inner();
            let literal = primary_inner.next().ok_or_else(|| anyhow::anyhow!("Invalid primary"))?;
            eval_expr(literal, variables)
        }
        Rule::int_literal => {
            let num_str = first.as_str();
            Ok(FluxValue::Int(num_str.parse::<i64>().unwrap()))
        }
        Rule::float_literal => {
            let num_str = first.as_str();
            let trimmed = if num_str.ends_with('f') {
                &num_str[..num_str.len()-1]
            } else {
                num_str
            };
            Ok(FluxValue::Float(trimmed.parse::<f64>().unwrap()))
        }
        Rule::double_literal => {
            let num_str = first.as_str();
            Ok(FluxValue::Float(num_str.parse::<f64>().unwrap()))
        }
        Rule::string_literal => {
            // string_literal is atomic, so use as_str() directly
            let str_with_quotes = first.as_str();
            // Remove the surrounding quotes
            let str_val = if str_with_quotes.len() >= 2 && str_with_quotes.starts_with('"') && str_with_quotes.ends_with('"') {
                &str_with_quotes[1..str_with_quotes.len()-1]
            } else {
                str_with_quotes
            };
            Ok(FluxValue::Str(str_val.to_string()))
        }
        Rule::ident => {
            let var_name = first.as_str();
            variables.get(var_name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Variable not found: {}", var_name))
        }
        Rule::bool_literal => {
            let val = first.as_str() == "true";
            Ok(FluxValue::Int(if val { 1 } else { 0 }))
        }
        Rule::char_literal => {
            // For now, treat as integer (ASCII value of the character)
            let char_str = first.as_str();
            let c = char_str.chars().next().unwrap_or('\0');
            Ok(FluxValue::Int(c as i64))
        }
        _ => bail!("Unsupported expression type: {:?}", first.as_rule()),
    }
}

pub fn compile_condition(
    condition: Pair<Rule>,
    label_false: &str,
    text_section: &mut String,
    symbols: &SymbolTable,
    var_offsets: &HashMap<String, i32>,
) -> Result<()> {
    let expr_pair = condition.into_inner().next().unwrap();

    let mut inner_expr = expr_pair.clone().into_inner();
    let left_pair  = inner_expr.next();
    let op_pair    = inner_expr.next();
    let right_pair = inner_expr.next();

    if let (Some(left), Some(op), Some(right)) = (left_pair, op_pair, right_pair) {
        // --- LEFT operand ---
        let left_str = left.as_str().trim();
        if let Some(&offset) = var_offsets.get(left_str) {
            text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
        } else if let Ok(val) = eval_expr(left, &symbols.variables) {
            match val {
                FluxValue::Int(n) => text_section.push_str(&format!("    mov rax, {}\n", n)),
                _ => bail!("Condition operand must be integer"),
            }
        } else {
            bail!("Unknown left operand in condition: {}", left_str);
        }

        // --- RIGHT operand ---
        let right_str = right.as_str().trim();
        if let Some(&offset) = var_offsets.get(right_str) {
            text_section.push_str(&format!("    cmp rax, [rbp-{}]\n", offset));
        } else if let Ok(val) = eval_expr(right, &symbols.variables) {
            match val {
                FluxValue::Int(n) => text_section.push_str(&format!("    cmp rax, {}\n", n)),
                _ => bail!("Condition operand must be integer"),
            }
        } else {
            bail!("Unknown right operand in condition: {}", right_str);
        }

        // --- Jump based on operator ---
        let jump_op = match op.as_str() {
            "==" => "jne",
            "!=" => "je",
            "<"  => "jge",
            ">"  => "jle",
            "<=" => "jg",
            ">=" => "jl",
            _    => bail!("Unsupported operator in condition: {}", op.as_str()),
        };
        text_section.push_str(&format!("    {} {}\n", jump_op, label_false));

    } else {
        // Single boolean / variable condition: if (flag)
        let val_str = expr_pair.as_str().trim();
        if let Some(&offset) = var_offsets.get(val_str) {
            text_section.push_str(&format!("    cmp qword [rbp-{}], 0\n", offset));
            text_section.push_str(&format!("    je {}\n", label_false));
        } else if let Ok(val) = eval_expr(expr_pair, &symbols.variables) {
            match val {
                FluxValue::Int(n) => {
                    text_section.push_str(&format!("    mov rax, {}\n", n));
                    text_section.push_str("    cmp rax, 0\n");
                    text_section.push_str(&format!("    je {}\n", label_false));
                }
                _ => bail!("Condition must evaluate to an integer"),
            }
        } else {
            bail!("Cannot evaluate condition: {}", val_str);
        }
    }

    Ok(())
}

pub fn compile_block_from_if(
    if_stmt: Pair<Rule>,
    content: &str,
    source_lines: &[&str],
    symbols: &mut SymbolTable,
    data_section: &mut String,
    text_section: &mut String,
    unique_id: &mut usize,
    var_offsets: &mut HashMap<String, i32>,
    stack_offset: &mut i32,
    loop_start: Option<String>,
    loop_end: Option<String>,
) -> Result<()> {
    let mut if_inner = if_stmt.into_inner();
    let condition_pair = if_inner.next().unwrap();
    let then_block = if_inner.next().unwrap();
    let else_part = if_inner.next();

    let label_id = *unique_id;
    *unique_id += 1;
    let label_false = format!(".if_false_{}", label_id);
    let label_end = format!(".if_end_{}", label_id);

    compile_condition(condition_pair, &label_false, text_section, symbols, &var_offsets)?;
    compile_block_with_loop_context(
        then_block,
        content,
        source_lines,
        symbols,
        data_section,
        text_section,
        unique_id,
        var_offsets,
        stack_offset,
        false,
        loop_start.clone(),
        loop_end.clone(),
    )?;
    text_section.push_str(&format!("    jmp {}\n", label_end));
    text_section.push_str(&format!("{}:\n", label_false));

    if let Some(else_pair) = else_part {
        let mut else_inner = else_pair.into_inner();
        if let Some(else_block) = else_inner.next() {
            match else_block.as_rule() {
                Rule::block => {
                    compile_block_with_loop_context(
                        else_block,
                        content,
                        source_lines,
                        symbols,
                        data_section,
                        text_section,
                        unique_id,
                        var_offsets,
                        stack_offset,
                        false,
                        loop_start,
                        loop_end,
                    )?;
                }
                Rule::if_stmt => {
                    compile_block_from_if(
                        else_block,
                        content,
                        source_lines,
                        symbols,
                        data_section,
                        text_section,
                        unique_id,
                        var_offsets,
                        stack_offset,
                        loop_start,
                        loop_end,
                    )?;
                }
                _ => {}
            }
        }
    }
    text_section.push_str(&format!("{}:\n", label_end));
    Ok(())
}

fn compile_block_with_loop_context(
    _block: Pair<Rule>,
    _content: &str,
    _source_lines: &[&str],
    _symbols: &mut SymbolTable,
    _data_section: &mut String,
    _text_section: &mut String,
    _unique_id: &mut usize,
    _var_offsets: &mut HashMap<String, i32>,
    _stack_offset: &mut i32,
    _is_function: bool,
    _loop_start: Option<String>,
    _loop_end: Option<String>,
) -> Result<()> {
    // Dummy implementation
    Ok(())
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
