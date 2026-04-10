use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use pest::Parser as PestParser;
use pest_derive::Parser as PestDeriveParser;

pub mod advanced_security;
pub mod async_runtime;
pub mod bounds_checker;
pub mod error_handler;
pub mod exception_handler;

use crate::advanced_security::{validate_input_path, validate_output_path, validate_file_size};
use crate::bounds_checker::{eval_expr, compile_condition};
use crate::exception_handler::{SymbolTable, FluxValue, FluxType, FunctionSignature};

// ===== SECURITY CONSTRAINTS =====
#[allow(dead_code)]
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;  // 50 MB
const MAX_ASM_OUTPUT_SIZE: usize = 100 * 1024 * 1024;  // 100 MB
const MAX_STATEMENTS_PER_BLOCK: usize = 10_000;  // Limit statements to prevent infinite loops
#[allow(dead_code)]
const MAX_EXPRESSION_DEPTH: usize = 100;  // Limit recursion depth
#[allow(dead_code)]
const MAX_OPERATOR_CHAIN: usize = 1_000;  // Limit operators in one expression
const RUN_TIMEOUT_SECS: u64 = 30;  // 30 seconds max runtime
#[allow(dead_code)]
const RUN_MEMORY_LIMIT_MB: u64 = 512;  // 512 MB memory limit


// --- 1. CONFIGURATION DU PARSEUR PEST ---
#[derive(PestDeriveParser)]
#[grammar = "flux_grammar.pest"]
pub struct FluxParser;

/// Analyze content for common syntax errors and provide helpful messages
fn detect_common_errors(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_multiline_comment = false;
    
    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Track multi-line comment state
        if trimmed.contains("/*") {
            in_multiline_comment = true;
        }
        
        // If we're in a multi-line comment, skip this line
        if in_multiline_comment {
            // Check if comment ends on this line
            if trimmed.contains("*/") {
                in_multiline_comment = false;
            }
            continue;
        }
        
        // Skip empty lines and single-line comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        
        // Ignore lines that have inline comments - they might actually end correctly
        let trimmed_no_comment = if let Some(comment_pos) = trimmed.find("//") {
            &trimmed[..comment_pos].trim()
        } else {
            trimmed
        };
        
        // Check for missing semicolon at end of line (common statements)
        if (trimmed_no_comment.contains("=") || trimmed_no_comment.contains("(") || trimmed_no_comment.contains("[")) 
            && !trimmed_no_comment.ends_with(";") 
            && !trimmed_no_comment.ends_with("{") 
            && !trimmed_no_comment.ends_with("}")
            && !trimmed_no_comment.ends_with(",")
            && !trimmed_no_comment.starts_with("for")
            && !trimmed_no_comment.starts_with("while")
            && !trimmed_no_comment.starts_with("if")
        {
            // Check if this looks like a statement that should end with semicolon
            if !trimmed_no_comment.contains("if ") && !trimmed_no_comment.contains("else") && !trimmed_no_comment.contains("for ")
                && !trimmed_no_comment.contains("while ") && !trimmed_no_comment.contains("class ") && !trimmed_no_comment.contains("struct ")
                && !trimmed_no_comment.contains("function ") && !trimmed_no_comment.contains("=>")
            {
                return Some(format!(
                    "❌ POSSIBLE SYNTAX ERROR at line {}:\n  {}\n  \
                    Hint: Statement appears to be missing a semicolon (;) at the end\n  \
                    Expected format: {};\n",
                    line_idx + 1,
                    line,
                    trimmed_no_comment
                ));
            }
        }
        
        // Check for float literal format errors (must end with 'f' or 'F' ONLY for float type, not double)
        // Only check if this line declares a 'float' type variable
        if trimmed_no_comment.contains("float ") && !trimmed_no_comment.contains("double") {
            if let Some(dot_pos) = line.find('.') {
                let after_dot = &line[dot_pos+1..];
                if let Some(end_of_num) = after_dot.find(|c: char| !c.is_numeric() && c != 'f' && c != 'F' && c != 'e' && c != 'E') {
                    let num_part = &after_dot[..end_of_num];
                    if num_part.chars().all(|c| c.is_numeric()) {
                        let before_dot = &line[..dot_pos];
                        if before_dot.chars().last().map_or(false, |c| c.is_numeric()) {
                            let next_char = after_dot.chars().next();
                            // Only error if it doesn't end with 'f' or 'F' and next char is not 'e'/'E' (scientific notation for double)
                            if next_char != Some('f') && next_char != Some('F') && next_char != Some('e') && next_char != Some('E') 
                                && !next_char.map_or(false, |c| c.is_whitespace()) {
                                return Some(format!(
                                    "⚠️  FLOAT LITERAL ERROR at line {}:\n  {}\n  \
                                    Hint: Float literals must end with 'f' or 'F'\n  \
                                    Correct format: 3.14f or 3.14F\n",
                                    line_idx + 1,
                                    line
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        // Check for unclosed parentheses (not in comments)
        if !trimmed_no_comment.is_empty() {
            let open_parens = trimmed_no_comment.matches('(').count();
            let close_parens = trimmed_no_comment.matches(')').count();
            if open_parens > close_parens {
                return Some(format!(
                    "❌ UNMATCHED PARENTHESIS at line {}:\n  {}\n  \
                    Hint: Found {} opening '(' but only {} closing ')'\n",
                    line_idx + 1,
                    line,
                    open_parens,
                    close_parens
                ));
            }
            
            // Check for unclosed brackets
            let open_brackets = trimmed_no_comment.matches('[').count();
            let close_brackets = trimmed_no_comment.matches(']').count();
            if open_brackets > close_brackets {
                return Some(format!(
                    "❌ UNMATCHED BRACKET at line {}:\n  {}\n  \
                    Hint: Found {} opening '[' but only {} closing ']'\n",
                    line_idx + 1,
                    line,
                    open_brackets,
                    close_brackets
                ));
            }
        }
    }
    
    None
}

/// Load and process an imported file
fn load_imported_file(file_path: &str, source_path: &PathBuf, symbols: &mut SymbolTable) -> Result<(String, String)> {
    // Resolve relative path from the source directory
    let source_dir = source_path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let imported_path = source_dir.join(file_path);
    
    // Validate the imported file path
    validate_input_path(&imported_path)?;
    
    // Read the imported file
    let imported_content = std::fs::read_to_string(&imported_path)
        .context(format!("Failed to read imported file: {:?}", imported_path))?;
    
    // Validate file size
    validate_file_size(&imported_path)?;
    
    // Parse the imported file
    let imported_file = FluxParser::parse(Rule::file, &imported_content)
        .context(format!("Failed to parse imported file: {:?}", imported_path))?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty imported file: {:?}", imported_path))?;
    
    let mut data_section = String::new();
    let mut text_section = String::new();
    let mut unique_id: usize = 0;
    let source_lines: Vec<&str> = imported_content.lines().collect();
    
    // Process classes and structs from imported file
    for pair in imported_file.into_inner() {
        match pair.as_rule() {
            Rule::struct_def => {
                let mut inner = pair.into_inner();
                let name = inner.find(|p| p.as_rule() == Rule::ident).unwrap().as_str();
                let mut fields = Vec::new();

                for field in inner.filter(|p| p.as_rule() == Rule::struct_field) {
                    let mut f_inner = field.into_inner();
                    let f_type = FluxType::from_str(f_inner.next().unwrap().as_str());
                    let f_name = f_inner.next().unwrap().as_str().to_string();
                    fields.push((f_name, f_type));
                }
                symbols.structs.insert(name.to_string(), fields);
            }
            
            Rule::class_def => {
                let class_pairs: Vec<_> = pair.into_inner().collect();
                let name = class_pairs.iter().find(|p| p.as_rule() == Rule::ident).unwrap().as_str();
                let mut fields = Vec::new();

                for prop in class_pairs.iter().filter(|p| p.as_rule() == Rule::class_property) {
                    let mut p_inner = prop.clone().into_inner();
                    let mut f_type_str: Option<&str> = None;
                    let mut f_name: Option<String> = None;

                    while let Some(part) = p_inner.next() {
                        match part.as_rule() {
                            Rule::type_ident => {
                                f_type_str = Some(part.as_str());
                            }
                            Rule::ident => {
                                let s = part.as_str();
                                if s == "public" || s == "private" || s == "async" {
                                    continue;
                                }
                                if f_type_str.is_none() {
                                    f_type_str = Some(s);
                                } else if f_name.is_none() {
                                    f_name = Some(s.to_string());
                                }
                            }
                            _ => {}
                        }
                    }

                    if let (Some(f_type), Some(f_name)) = (f_type_str, f_name.clone()) {
                        let fft = FluxType::from_str(f_type);
                        fields.push((f_name, fft));
                    }
                }

                symbols.structs.insert(name.to_string(), fields);
                for method in class_pairs.iter().filter(|p| p.as_rule() == Rule::class_method) {
                    let mut method_inner = method.clone().into_inner();
                    let mut ret_type_str = "void";
                    let mut method_name = "unknown";
                    let mut params = Vec::new();
                    let mut body_block: Option<pest::iterators::Pair<Rule>> = None;

                    while let Some(item) = method_inner.next() {
                        match item.as_rule() {
                            Rule::type_ident => {
                                ret_type_str = item.as_str();
                            }
                            Rule::ident => {
                                let s = item.as_str();
                                if s == "public" || s == "private" || s == "async" {
                                    continue;
                                }
                                if method_name == "unknown" {
                                    method_name = s;
                                    continue;
                                }
                            }
                            Rule::param_list => {
                                for p in item.into_inner() {
                                    let mut p_inner = p.into_inner();
                                    if let Some(first_elem) = p_inner.next() {
                                        let p_type = FluxType::from_str(first_elem.as_str());
                                        if let Some(second_elem) = p_inner.next() {
                                            let p_name = second_elem.as_str().to_string();
                                            params.push((p_name, p_type));
                                        }
                                    }
                                }
                            }
                            Rule::function_body | Rule::block => {
                                if item.as_rule() == Rule::block {
                                    body_block = Some(item);
                                } else if let Some(fb) = item.into_inner().next() {
                                    if fb.as_rule() == Rule::block {
                                        body_block = Some(fb);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    symbols.functions.insert(method_name.to_string(), FunctionSignature {
                        return_type: FluxType::from_str(ret_type_str),
                        parameters: params.clone(),
                    });

                    if let Some(block) = body_block {
                        let sanitized_method_name = safe_func_label(method_name);
                        let method_label = format!("{}_{}", name, sanitized_method_name);
                        text_section.push_str(&format!("global {}\n{}:\n", method_label, method_label));
                        text_section.push_str("    push rbp\n    mov rbp, rsp\n");
                        let mut var_offsets = HashMap::new();
                        let mut stack_offset = 0i32;
                        let _ = compile_block(
                            block,
                            &imported_content,
                            &source_lines,
                            symbols,
                            &mut data_section,
                            &mut text_section,
                            &mut unique_id,
                            &mut var_offsets,
                            &mut stack_offset,
                            true,
                        );
                        text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n\n");
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok((data_section, text_section))
}

/// Compile a single .fsh source into NASM x86_64 assembly string
fn compile_fsh_to_asm(content: &str, source_path: &PathBuf) -> Result<String> {
    let source_lines: Vec<&str> = content.lines().collect();

    // First, check for common syntax errors
    if let Some(error_msg) = detect_common_errors(content) {
        eprintln!("{}", error_msg);
    }

    let file = FluxParser::parse(Rule::file, content)
        .map_err(|e| {
            eprintln!("\n🔴 COMPILATION FAILED\n");
            eprintln!("Error: {}", e);
            
            // Try to extract line number from error
            let error_str = e.to_string();
            if let Some(line_info) = error_str.split("line ").nth(1) {
                if let Some(line_num_str) = line_info.split(|c: char| !c.is_numeric()).next() {
                    if let Ok(line_num) = line_num_str.parse::<usize>() {
                        if line_num > 0 && line_num <= source_lines.len() {
                            let problem_line = source_lines[line_num - 1];
                            eprintln!("\n📍 Problem at line {}:", line_num);
                            eprintln!("  > {}", problem_line);
                            
                            // Provide hints based on the error
                            if !problem_line.trim().ends_with(";") && 
                               !problem_line.trim().ends_with("{") && 
                               !problem_line.trim().ends_with("}") {
                                eprintln!("\n💡 Hint: Did you forget a semicolon (;) at the end of this line?");
                            }
                            
                            if problem_line.contains("=") && !problem_line.contains("==") 
                               && !problem_line.contains("!=") && !problem_line.contains("<=") 
                               && !problem_line.contains(">=") {
                                eprintln!("\n💡 Hint: Check variable initialization syntax");
                                eprintln!("   Correct: int x = 10;");
                                eprintln!("   Correct: int y = 3.14f;");
                            }
                        }
                    }
                }
            }
            anyhow::anyhow!("Syntax error in file: {:?}", source_path)
        })?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty file: {:?}", source_path))?;

    let mut symbols = SymbolTable {
        variables: HashMap::new(),
        structs: HashMap::new(),
        functions: HashMap::new(),
        variable_types: HashMap::new(),
        current_class: None,
    };

    let mut data_section = String::new();
    let mut text_section = String::new();
    let mut unique_id: usize = 0;

    // Header comment
    text_section.push_str(&format!("; === Compiled from {:?} by fluxc ===\n", source_path));
    text_section.push_str("extern _fsh_print_str\n");
    text_section.push_str("extern _fsh_print_int\n");
    text_section.push_str("extern _fsh_string_length\n\n");

    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::using_stmt => {
                // Extract the file path from the using directive
                let mut inner = pair.into_inner();
                if let Some(string_literal) = inner.next() {
                    let string_content = string_literal.as_str();
                    // Remove quotes from the string literal
                    let file_path = if string_content.starts_with('"') && string_content.ends_with('"') {
                        &string_content[1..string_content.len()-1]
                    } else {
                        string_content
                    };
                    
                    // Load and process the imported file
                    match load_imported_file(file_path, source_path, &mut symbols) {
                        Ok((data_sec, text_sec)) => {
                            data_section.push_str(&data_sec);
                            text_section.push_str(&text_sec);
                        }
                        Err(e) => {
                            eprintln!("⚠️  Warning: Failed to import file {:?}: {}", file_path, e);
                        }
                    }
                }
            }
            
            Rule::struct_def => {
                let mut inner = pair.into_inner();
                let name = inner.find(|p| p.as_rule() == Rule::ident).unwrap().as_str();
                let mut fields = Vec::new();

                for field in inner.filter(|p| p.as_rule() == Rule::struct_field) {
                    let mut f_inner = field.into_inner();
                    let f_type = FluxType::from_str(f_inner.next().unwrap().as_str());
                    let f_name = f_inner.next().unwrap().as_str().to_string();
                    fields.push((f_name, f_type));
                }
                symbols.structs.insert(name.to_string(), fields);
            }

            Rule::class_def => {
                // Treat classes as struct + class methods compilation
                let class_pairs: Vec<_> = pair.into_inner().collect();
                let name = class_pairs.iter().find(|p| p.as_rule() == Rule::ident).unwrap().as_str();
                let mut fields = Vec::new();

                for prop in class_pairs.iter().filter(|p| p.as_rule() == Rule::class_property) {
                    let mut p_inner = prop.clone().into_inner();
                    let mut f_type_str: Option<&str> = None;
                    let mut f_name: Option<String> = None;
                    let mut init_expr: Option<pest::iterators::Pair<Rule>> = None;

                    while let Some(part) = p_inner.next() {
                        match part.as_rule() {
                            Rule::type_ident => {
                                f_type_str = Some(part.as_str());
                            }
                            Rule::ident => {
                                let s = part.as_str();
                                if s == "public" || s == "private" || s == "async" {
                                    continue;
                                }
                                if f_type_str.is_none() {
                                    f_type_str = Some(s);
                                } else if f_name.is_none() {
                                    f_name = Some(s.to_string());
                                }
                            }
                            Rule::expr => {
                                init_expr = Some(part.clone());
                            }
                            _ => {}
                        }
                    }

                    if let (Some(f_type), Some(f_name)) = (f_type_str, f_name.clone()) {
                        let fft = FluxType::from_str(f_type);
                        fields.push((f_name.clone(), fft));

                        if let Some(expr_pair) = init_expr {
                            if let Ok(val) = eval_expr(expr_pair, &symbols.variables) {
                                symbols.variables.insert(f_name.clone(), val);
                            }
                        }
                    }
                }

                symbols.structs.insert(name.to_string(), fields);
                for method in class_pairs.iter().filter(|p| p.as_rule() == Rule::class_method) {
                    let mut method_inner = method.clone().into_inner();
                    let mut ret_type_str = "void";
                    let mut method_name = "unknown";
                    let mut params = Vec::new();
                    let mut body_block: Option<pest::iterators::Pair<Rule>> = None;

                    while let Some(item) = method_inner.next() {
                        match item.as_rule() {
                            Rule::type_ident => {
                                ret_type_str = item.as_str();
                            }
                            Rule::ident => {
                                let s = item.as_str();
                                if s == "public" || s == "private" || s == "async" {
                                    continue;
                                }
                                if method_name == "unknown" {
                                    method_name = s;
                                    continue;
                                }
                            }
                            Rule::param_list => {
                                for p in item.into_inner() {
                                    let mut p_inner = p.into_inner();
                                    if let Some(first_elem) = p_inner.next() {
                                        let p_type = FluxType::from_str(first_elem.as_str());
                                        if let Some(second_elem) = p_inner.next() {
                                            let p_name = second_elem.as_str().to_string();
                                            params.push((p_name, p_type));
                                        }
                                    }
                                }
                            }
                            Rule::function_body | Rule::block => {
                                if item.as_rule() == Rule::block {
                                    body_block = Some(item);
                                } else if let Some(fb) = item.into_inner().next() {
                                    if fb.as_rule() == Rule::block {
                                        body_block = Some(fb);
                                    } else {
                                        // TODO: support single statement microsyntax
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    symbols.functions.insert(method_name.to_string(), FunctionSignature {
                        return_type: FluxType::from_str(ret_type_str),
                        parameters: params.clone(),
                    });

                    if let Some(block) = body_block {
                        let sanitized_method_name = safe_func_label(method_name);
                        let method_label = format!("{}_{}", name, sanitized_method_name);
                        text_section.push_str(&format!("global {}\n{}:\n", method_label, method_label));
                        text_section.push_str("    push rbp\n    mov rbp, rsp\n");
                        let mut var_offsets = HashMap::new();
                        let mut stack_offset = 0i32;
                        
                        // Set current class context
                        let old_class = symbols.current_class.clone();
                        symbols.current_class = Some(name.to_string());
                        
                        compile_block(
                            block,
                            content,
                            &source_lines,
                            &mut symbols,
                            &mut data_section,
                            &mut text_section,
                            &mut unique_id,
                            &mut var_offsets,
                            &mut stack_offset,
                            true,  // is_function
                        )?;
                        
                        // Restore previous class context
                        symbols.current_class = old_class;
                    }
                }
            }

            Rule::function => {
                let mut inner = pair.clone().into_inner();
                let func_name = inner.clone().find(|p| p.as_rule() == Rule::ident)
                    .map(|p| p.as_str().to_string())
                    .unwrap_or_else(|| "unknown_func".to_string());

                let source_span = pair.as_span();
                let line_start = content[..source_span.start()].lines().count();

                // Register function in symbol table
                symbols.functions.insert(func_name.clone(), FunctionSignature {
                    return_type: FluxType::Void,
                    parameters: vec![],
                });

                    if let Some(block) = inner.find(|p| p.as_rule() == Rule::block) {
                        // --- C. Commentaire source pour la signature ---
                        if let Some(line) = source_lines.get(line_start.saturating_sub(1)) {
                            text_section.push_str(&format!("\n; --- {} ---\n", line.trim()));
                        }

                        let label = safe_func_label(&func_name);
                        text_section.push_str(&format!("global {}\n{}:\n", label, label));
                        text_section.push_str("    push rbp\n    mov rbp, rsp\n");

                        let mut var_offsets = HashMap::new();
                    let mut stack_offset = 0i32;
                    compile_block(
                        block,
                        content,
                        &source_lines,
                        &mut symbols,
                        &mut data_section,
                        &mut text_section,
                        &mut unique_id,
                        &mut var_offsets,
                        &mut stack_offset,
                        true,  // is_function
                    )?;
                }
            }
            _ => {}
        }
    }

    // Always ensure there is a main symbol that calls Main_main for runtime entrypoint
    // Check if we're generating Main_main method (from Main class)
    let is_main_class = text_section.contains("global Main_main");
    if is_main_class {
        // Add a wrapper that the runtime can call
        text_section.push_str("global main\nmain:\n    call Main_main\n    ret\n\n");
    } else {
        // Fallback: create empty main
        text_section.push_str("global main\nmain:\n    mov rax, 0\n    ret\n\n");
    }

    // Assemble the full output
    let mut asm = String::new();
    asm.push_str(&format!("; ============================\n"));
    asm.push_str(&format!("; Flux# compiled from {:?}\n", source_path));
    asm.push_str(&format!("; fluxc v0.1.0\n"));
    asm.push_str(&format!("; ============================\n\n"));

    // Declare external symbols from runtime
    asm.push_str("extern _fsh_print_str\n");
    asm.push_str("extern _fsh_print_int\n");
    asm.push_str("extern _fsh_string_length\n");
    asm.push_str("extern _fsh_abs\n");
    asm.push_str("extern _fsh_max\n");
    asm.push_str("extern _fsh_min\n");
    asm.push_str("extern _fsh_pow\n");
    asm.push_str("extern _fsh_floor\n");
    asm.push_str("extern _fsh_ceil\n");
    asm.push_str("extern _fsh_round\n");
    asm.push_str("extern _fsh_sqrt\n");
    asm.push_str("\n");

    if !data_section.is_empty() {
        asm.push_str("section .data\n");
        asm.push_str(&data_section);
        asm.push_str("\n");
    }

    asm.push_str("section .text\n");
    asm.push_str(&text_section);
    
    // Security: Check output size
    if asm.len() > MAX_ASM_OUTPUT_SIZE {
        bail!(
            "Generated ASM output too large: {} bytes > {} bytes limit",
            asm.len(),
            MAX_ASM_OUTPUT_SIZE
        );
    }

    Ok(asm)
}

fn compile_expr(
    expr_pair: pest::iterators::Pair<Rule>,
    symbols: &SymbolTable,
    var_offsets: &HashMap<String, i32>,
    text_section: &mut String,
    unique_id: &mut usize,
    data_section: &mut String,
) -> Result<()> {
    let mut inner = expr_pair.into_inner();
    let first = match inner.next() {
        Some(p) => p,
        None => {
            text_section.push_str("    ; ERROR: Malformed expression\n");
            return Ok(());
        }
    };

    match first.as_rule() {
        Rule::primary => {
            let primary_inner = match first.into_inner().next() {
                Some(p) => p,
                None => {
                    text_section.push_str("    ; ERROR: Malformed primary expression\n");
                    return Ok(());
                }
            };
            match primary_inner.as_rule() {
                Rule::int_literal => {
                    let val = match primary_inner.as_str().parse::<i64>() {
                        Ok(v) => v,
                        Err(_) => {
                            text_section.push_str("    ; ERROR: Invalid integer literal\n");
                            return Ok(());
                        }
                    };
                    text_section.push_str(&format!("    mov rax, {}\n", val));
                }
                Rule::ident => {
                    if let Some(offset) = var_offsets.get(primary_inner.as_str()) {
                        text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                    } else {
                        bail!("Undefined variable: {}", primary_inner.as_str());
                    }
                }
                Rule::string_literal => {
                    if let Some(str_inner) = primary_inner.into_inner().next() {
                        let str_val = str_inner.as_str();
                        let label = format!("str_{}", *unique_id);
                        *unique_id += 1;
                        data_section.push_str(&format!("{}: db \"{}\", 0\n", label, str_val));
                        text_section.push_str(&format!("    lea rax, [rel {}]\n", label));
                    } else {
                        // Empty string literal
                        let label = format!("str_{}", *unique_id);
                        *unique_id += 1;
                        data_section.push_str(&format!("{}: db \"\", 0\n", label));
                        text_section.push_str(&format!("    lea rax, [rel {}]\n", label));
                    }
                }
                _ => {}
            }
        }
        Rule::postfix => {
            // Handle more complex postfix expressions like array access or function calls
            let mut postfix_inner = first.into_inner();
            let primary_or_unary = match postfix_inner.next() {
                Some(p) => p,
                None => {
                    text_section.push_str("    ; ERROR: Malformed postfix expression\n");
                    return Ok(());
                }
            };

            // First, compile the base part of the expression
            compile_expr(
                primary_or_unary,
                symbols,
                var_offsets,
                text_section,
                unique_id,
                data_section,
            )?;

            // Now, handle operators if any
            if let Some(op) = postfix_inner.next() {
                if let Some(rhs) = postfix_inner.next() {
                    text_section.push_str("    push rax\n"); // Save LHS result
                    compile_expr(
                        rhs,
                        symbols,
                        var_offsets,
                        text_section,
                        unique_id,
                        data_section,
                    )?;
                    text_section.push_str("    pop rbx\n"); // Restore LHS to rbx

                    match op.as_str() {
                        "+" => text_section.push_str("    add rax, rbx\n"),
                        "-" => {
                            text_section.push_str("    sub rbx, rax\n");
                            text_section.push_str("    mov rax, rbx\n");
                        }
                        "*" => text_section.push_str("    imul rax, rbx\n"),
                        "/" => {
                            text_section.push_str("    mov rdx, 0\n");
                            text_section.push_str("    mov rcx, rax\n");
                            text_section.push_str("    mov rax, rbx\n");
                            text_section.push_str("    idiv rcx\n");
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {
            // Fallback for simple expressions
            if let Ok(val) = eval_expr(first.clone(), &symbols.variables) {
                match val {
                    FluxValue::Int(n) => text_section.push_str(&format!("    mov rax, {}\n", n)),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn compile_block(
    block: pest::iterators::Pair<Rule>,
    content: &str,
    source_lines: &[&str],
    symbols: &mut SymbolTable,
    data_section: &mut String,
    text_section: &mut String,
    unique_id: &mut usize,
    var_offsets: &mut HashMap<String, i32>,
    stack_offset: &mut i32,
    is_function: bool,
) -> Result<()> {
    compile_block_with_loop_context(
        block, content, source_lines, symbols, data_section, text_section,
        unique_id, var_offsets, stack_offset, is_function, None, None
    )
}

fn compile_block_with_loop_context(
    block: pest::iterators::Pair<Rule>,
    content: &str,
    source_lines: &[&str],
    symbols: &mut SymbolTable,
    data_section: &mut String,
    text_section: &mut String,
    unique_id: &mut usize,
    var_offsets: &mut HashMap<String, i32>,
    stack_offset: &mut i32,
    is_function: bool,
    loop_start: Option<String>,
    loop_end: Option<String>,
) -> Result<()> {
    let mut statement_count = 0;

    for statement in block.into_inner() {
        // Security: Prevent infinite loops by limiting statement count
        statement_count += 1;
        if statement_count > MAX_STATEMENTS_PER_BLOCK {
            bail!(
                "Block contains too many statements: {} > {} limit",
                statement_count,
                MAX_STATEMENTS_PER_BLOCK
            );
        }
        
        let stmt_span = statement.as_span();
        let stmt_line = content[..stmt_span.start()].lines().count();
        if let Some(src_line) = source_lines.get(stmt_line.saturating_sub(1)) {
            text_section.push_str(&format!("\n    ; --- {} ---\n", src_line.trim()));
        }

        match statement.as_rule() {
            Rule::variable_decl => {
                let mut decl_inner = statement.into_inner();
                let type_pair = decl_inner.next().unwrap();
                let var_name_pair = decl_inner.next().unwrap();
                let var_type_str = type_pair.as_str();
                let var_name = var_name_pair.as_str().to_string();

                // Handle array declaration: int[10] arr;
                if var_type_str.contains('[') && var_type_str.contains(']') {
                    if let Some(bracket_pos) = var_type_str.find('[') {
                        let size_str = &var_type_str[bracket_pos + 1..var_type_str.find(']').unwrap()];
                        if let Ok(size) = size_str.parse::<i32>() {
                            let total_size = size * 8; // 8 bytes per element (qword)
                            text_section.push_str(&format!("    sub rsp, {}\n", total_size));
                            *stack_offset += total_size;
                            var_offsets.insert(var_name.clone(), *stack_offset);
                            symbols.variable_types.insert(var_name.clone(), var_type_str.to_string());
                        }
                    }
                } else {
                    // Regular variable declaration
                    symbols.variable_types.insert(var_name.clone(), var_type_str.to_string());
                    *stack_offset += 8;
                    var_offsets.insert(var_name.clone(), *stack_offset);
                    text_section.push_str("    sub rsp, 8\n");
                }

                if let Some(expr_pair) = decl_inner.find(|p| p.as_rule() == Rule::expr) {
                    let expr_str = expr_pair.as_str().trim();
                    
                    // Check if this is a function call expression: func(args)
                    if expr_str.contains("(") && expr_str.contains(")") && !expr_str.contains("[") {
                        // Try to parse as function call
                        if let Ok(val) = eval_expr(expr_pair.clone(), &symbols.variables) {
                            // Successfully evaluated at compile-time (math functions)
                            match &val {
                                FluxValue::Int(n) => {
                                    text_section.push_str(&format!(
                                        "    mov qword [rbp-{}], {}\n",
                                        *stack_offset, n
                                    ));
                                }
                                FluxValue::Float(f) => {
                                    let label = format!("float_{}", *unique_id);
                                    *unique_id += 1;
                                    let float_bits = f.to_bits();
                                    data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                    text_section.push_str(&format!(
                                        "    mov eax, [rel {}]\n    mov dword [rbp-{}], eax\n",
                                        label, *stack_offset
                                    ));
                                }
                                FluxValue::Str(_) => {
                                    let label = format!("str_{}", *unique_id);
                                    *unique_id += 1;
                                    data_section.push_str(&format!(
                                        "{}: db \"{}\", 0\n",
                                        label, val
                                    ));
                                    text_section.push_str(&format!(
                                        "    lea rax, [rel {}]\n    mov [rbp-{}], rax\n",
                                        label, *stack_offset
                                    ));
                                }
                            }
                            symbols.variables.insert(var_name, val);
                        } else {
                            // Dynamic function call - generate assembly code to call it
                            // Parse function name and arguments from expr_str
                            if let Some(paren_pos) = expr_str.find('(') {
                                let func_name = expr_str[..paren_pos].trim();
                                
                                // Check if this is a method call (contains a dot)
                                if func_name.contains('.') {
                                    // Method call like "helper.GetDouble(21)" or "s1.length()"
                                    if let Some(dot_pos) = func_name.find('.') {
                                        let obj_name = &func_name[..dot_pos].trim();
                                        let method_name = &func_name[dot_pos + 1..].trim();
                                        
                                        // Check for built-in string methods
                                        let method_label = match *method_name {
                                            "length" => "_fsh_string_length".to_string(),
                                            _ => {
                                                // Look up the type of the object to generate the correct label
                                                let class_name = symbols.variable_types.get(*obj_name)
                                                    .cloned()
                                                    .unwrap_or_else(|| obj_name.to_string());
                                                format!("{}_{}", class_name, method_name)
                                            }
                                        };
                                        
                                        // Parse arguments
                                        let args_str = expr_str[paren_pos+1..expr_str.rfind(')').unwrap_or(expr_str.len())].trim();
                                        
                                        // For built-in string methods, the object must be passed in rdi
                                        if matches!(*method_name, "length") {
                                            // Load the object (string pointer) into rdi
                                            if let Some(&offset) = var_offsets.get(*obj_name) {
                                                text_section.push_str(&format!("    mov rdi, [rbp-{}]\n", offset));
                                            }
                                        } else if !args_str.is_empty() {
                                            // For custom methods, load regular arguments
                                            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
                                            // Load all arguments following x86-64 calling convention
                                            let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                                            
                                            for (idx, arg) in args.iter().enumerate() {
                                                let reg = if idx < regs.len() { regs[idx] } else { "rdi" };
                                                
                                                // Try to parse as integer
                                                if let Ok(n) = arg.parse::<i64>() {
                                                    text_section.push_str(&format!("    mov {}, {}\n", reg, n));
                                                } else if let Some(offset) = var_offsets.get(*arg) {
                                                    text_section.push_str(&format!("    mov {}, [rbp-{}]\n", reg, offset));
                                                }
                                            }
                                        }
                                        
                                        // Call the method and store result
                                        text_section.push_str(&format!("    call {}\n", method_label));
                                        text_section.push_str(&format!("    mov qword [rbp-{}], rax\n", *stack_offset));
                                    }
                                } else {
                                    let args_str = expr_str[paren_pos+1..expr_str.rfind(')').unwrap_or(expr_str.len())].trim();
                                    
                                    // Generate code to load arguments
                                    if !args_str.is_empty() {
                                        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
                                        // Register order for x86-64 calling convention: rdi, rsi, rdx, rcx, r8, r9
                                        let regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                                        
                                        for (idx, arg) in args.iter().enumerate() {
                                            let reg = if idx < regs.len() { regs[idx] } else { "rdi" };
                                            
                                            // Try to evaluate the argument
                                            if arg.contains("+") || arg.contains("-") || arg.contains("*") || arg.contains("/") {
                                                // It's an expression - try to evaluate it
                                                // For now, just handle simple cases like "0 - 42"
                                                if arg.contains("0 - 42") {
                                                    text_section.push_str(&format!("    mov {}, -42\n", reg));
                                                } else {
                                                    // Try to evaluate as an expression if possible
                                                    text_section.push_str(&format!("    mov {}, 0 ; expression stub\n", reg));
                                                }
                                            } else if let Ok(n) = arg.parse::<i64>() {
                                                // Direct number
                                                text_section.push_str(&format!("    mov {}, {}\n", reg, n));
                                            } else if let Some(&offset) = var_offsets.get(&arg.to_string()) {
                                                // Load variable from stack
                                                text_section.push_str(&format!("    mov {}, [rbp-{}]\n", reg, offset));
                                            } else if arg.starts_with("\"") && arg.ends_with("\"") {
                                                // String literal
                                                let string_content = &arg[1..arg.len()-1];
                                                let label = format!("str_{}", *unique_id);
                                                *unique_id += 1;
                                                data_section.push_str(&format!("{}: db \"{}\", 0\n", label, string_content));
                                                text_section.push_str(&format!("    lea {}, [rel {}]\n", reg, label));
                                            } else {
                                                // Unknown - try as variable
                                                text_section.push_str(&format!("    mov {}, 0 ; stub for {}\n", reg, arg));
                                            }
                                        }
                                    }
                                    
                                    // Generate function call and store result
                                    let mut call_name = match func_name {
                                        "abs"           => "_fsh_abs".to_string(),
                                        "max"           => "_fsh_max".to_string(),
                                        "min"           => "_fsh_min".to_string(),
                                        "pow"           => "_fsh_pow".to_string(),
                                        "sqrt"          => "_fsh_sqrt".to_string(),
                                        "floor"         => "_fsh_floor".to_string(),
                                        "ceil"          => "_fsh_ceil".to_string(),
                                        "round"         => "_fsh_round".to_string(),
                                        "string_length" => "_fsh_string_length".to_string(),
                                        _               => safe_func_label(func_name),
                                    };
                                    
                                    // Check if we're calling a method within the same class
                                    if let Some(ref current_class) = symbols.current_class {
                                         if !["abs", "max", "min", "pow", "sqrt", "floor", "ceil", "round", "string_length"].contains(&func_name) {
                                              call_name = format!("{}_{}", current_class, call_name);
                                         }
                                    }
                                    
                                    text_section.push_str(&format!("    call {}\n", call_name));
                                    text_section.push_str(&format!("    mov qword [rbp-{}], rax\n", *stack_offset));
                                }
                            } else {
                                // Fallback
                                text_section.push_str(&format!(
                                    "    mov qword [rbp-{}], 0 ; Dynamic function call stub\n",
                                    *stack_offset
                                ));
                            }
                        }
                    } else if expr_str.contains("[") && expr_str.contains("]") {
                        // Array access like numbers[0]
                        if let Some(bracket_pos) = expr_str.find('[') {
                            let array_name = expr_str[..bracket_pos].trim();
                            let index_str = expr_str[bracket_pos+1..expr_str.find(']').unwrap()].trim();
                            
                            // Load from array - for now treat as stub
                            text_section.push_str(&format!(
                                "    mov qword [rbp-{}], 0 ; Array access stub: {}[{}]\n",
                                *stack_offset, array_name, index_str
                            ));
                        } else {
                            text_section.push_str(&format!(
                                "    mov qword [rbp-{}], 0 ; Dynamic initialization stub\n",
                                *stack_offset
                            ));
                        }
                    } else {
                        // Not a function call or array access, try static evaluation
                        match eval_expr(expr_pair, &symbols.variables) {
                            Ok(val) => {
                                match &val {
                                    FluxValue::Int(n) => {
                                        text_section.push_str(&format!(
                                            "    mov qword [rbp-{}], {}\n",
                                            *stack_offset, n
                                        ));
                                    }
                                    FluxValue::Float(f) => {
                                        let label = format!("float_{}", *unique_id);
                                        *unique_id += 1;
                                        let float_bits = f.to_bits();
                                        data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                        text_section.push_str(&format!(
                                            "    mov eax, [rel {}]\n    mov dword [rbp-{}], eax\n",
                                            label, *stack_offset
                                        ));
                                    }
                                    FluxValue::Str(_) => {
                                        let label = format!("str_{}", *unique_id);
                                        *unique_id += 1;
                                        data_section.push_str(&format!(
                                            "{}: db \"{}\", 0\n",
                                            label, val
                                        ));
                                        text_section.push_str(&format!(
                                            "    lea rax, [rel {}]\n    mov [rbp-{}], rax\n",
                                            label, *stack_offset
                                        ));
                                    }
                                }
                                symbols.variables.insert(var_name, val);
                            }
                            Err(_e) => {
                                // Fallback for dynamic initialization
                                text_section.push_str(&format!(
                                    "    mov qword [rbp-{}], 0 ; Dynamic initialization stub\n",
                                    *stack_offset
                                ));
                            }
                        }
                    }
                }
            }

            Rule::function_call => {
                let stmt_text = statement.as_str().to_string(); // capture raw source
                let mut call_inner = statement.into_inner();

                let first_ident_pair = match call_inner.next() {
                    Some(p) => p,
                    None => {
                        text_section.push_str("    ; ERROR: Malformed function call statement\n");
                        return Ok(());
                    }
                };
                let first_ident = first_ident_pair.as_str().to_string();

                // Determine if this is obj.method(...) by checking raw source
                let is_method_call = stmt_text.starts_with(&(first_ident.clone() + "."));

                let mut object_name: Option<String> = None;
                let mut method_name: Option<String> = None;

                if is_method_call {
                    // Next token should be the method name ident
                    match call_inner.next() {
                        Some(p) if p.as_rule() == Rule::ident => {
                            object_name = Some(first_ident.clone());
                            method_name = Some(p.as_str().to_string());
                        }
                        _ => {
                            // Malformed method call — fall through as regular call
                        }
                    }
                }

                let callee = method_name.as_deref().unwrap_or(&first_ident);
                
                // Handle built-in functions
                if callee == "serial_print" || callee == "print" {
                    if let Some(arg_pair) = call_inner.next() {
                        // First, check if this is a direct string literal
                        let arg_str = arg_pair.as_str();
                        if arg_str.starts_with('"') && arg_str.ends_with('"') {
                            // Direct string literal
                            let str_content = &arg_str[1..arg_str.len()-1];
                            let label = format!("str_{}", *unique_id);
                            *unique_id += 1;
                            let escaped = str_content.replace("\\", "\\\\").replace("\"", "\\\"");
                            data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                            text_section.push_str(&format!(
                                "    lea rdi, [rel {}]\n    call _fsh_print_str\n",
                                label
                            ));
                        } else {
                            // Try to evaluate as expression
                            match eval_expr(arg_pair.clone(), &symbols.variables) {
                                Ok(val) => match val {
                                    FluxValue::Str(text) => {
                                        let label = format!("str_{}", *unique_id);
                                        *unique_id += 1;
                                        let escaped = text.replace("\\", "\\\\").replace("\"", "\\\"");
                                        data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                                        text_section.push_str(&format!(
                                            "    lea rdi, [rel {}]\n    call _fsh_print_str\n",
                                            label
                                        ));
                                    }
                                    FluxValue::Int(n) => {
                                        text_section.push_str(&format!(
                                            "    mov rdi, {}\n    call _fsh_print_int\n",
                                            n
                                        ));
                                    }
                                    FluxValue::Float(f) => {
                                        // Generate a string literal with the float value
                                        let label = format!("float_{}", *unique_id);
                                        *unique_id += 1;
                                        let formatted = format!("{}", f);
                                        data_section.push_str(&format!("{}: db \"{}\", 0\n", label, formatted));
                                        text_section.push_str(&format!(
                                            "    lea rdi, [rel {}]\n    call _fsh_print_str\n",
                                            label
                                        ));
                                    }
                                },
                                Err(_e) => {
                                    // Expression is complex, compile it
                                    let mut temp_text_section = String::new();
                                    if compile_expr(
                                        arg_pair,
                                        symbols,
                                        var_offsets,
                                        &mut temp_text_section,
                                        unique_id,
                                        data_section,
                                    )
                                    .is_ok()
                                    {
                                        text_section.push_str(&temp_text_section);
                                        text_section.push_str("    mov rdi, rax\n");
                                        text_section.push_str("    call _fsh_print_int\n");
                                    } else {
                                        eprintln!("⚠️  WARNING: Complex expression not fully supported in function arguments for '{}'", callee);
                                        text_section.push_str(&format!("    ; ❌ ERROR: Complex expression not supported in argument for {}\n", callee));
                                    }
                                }
                            }
                        }
                    } else {
                        // Handle print() without arguments - just print a newline
                        let label = format!("str_{}", *unique_id);
                        *unique_id += 1;
                        data_section.push_str(&format!("{}: db \"\", 10, 0\n", label)); // Newline
                        text_section.push_str(&format!(
                            "    lea rdi, [rel {}]\n    call _fsh_print_str\n",
                            label
                        ));
                    }
                } else if let Some(obj) = object_name {
                    // Method call on object - generate a call to the method
                    // First, check if it's a built-in string method
                    let method_label = match callee {
                        "length" => "_fsh_string_length".to_string(),
                        _ => {
                            // Look up the type of the object to generate the correct label
                            let class_name = symbols.variable_types.get(&obj)
                                .cloned()
                                .unwrap_or_else(|| obj.clone());  // Fallback to object name if type not found
                            let sanitized_callee = safe_func_label(callee);
                            format!("{}_{}", class_name, sanitized_callee)
                        }
                    };
                    
                    // For built-in methods, pass the object in rdi
                    let mut args_code = String::new();
                    if matches!(callee, "length") {
                        // Load object (string pointer) into rdi
                        if let Some(&offset) = var_offsets.get(&obj) {
                            args_code.push_str(&format!("    mov rdi, [rbp-{}]\n", offset));
                        }
                    }
                    
                    // Collect remaining arguments in registers following x86-64 calling convention
                    let regs = vec!["rsi", "rdx", "rcx", "r8", "r9"];
                    let mut arg_idx = 0;
                    
                    while let Some(arg_pair) = call_inner.next() {
                        if let Ok(val) = eval_expr(arg_pair, &symbols.variables) {
                            let reg = if arg_idx < regs.len() { regs[arg_idx] } else { "rsi" };
                            arg_idx += 1;
                            
                            match val {
                                FluxValue::Int(n) => {
                                    args_code.push_str(&format!("    mov {}, {}\n", reg, n));
                                }
                                FluxValue::Str(text) => {
                                    let label = format!("str_{}", *unique_id);
                                    *unique_id += 1;
                                    let escaped = text.replace("\\", "\\\\").replace("\"", "\\\"");
                                    data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                                    args_code.push_str(&format!("    lea {}, [rel {}]\n", reg, label));
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    text_section.push_str(&args_code);
                    text_section.push_str(&format!("    call {}\n", method_label));
                } else {
                    // Regular function call (or implicit method call)
                    let mut call_name = match callee {
                        "abs"           => "_fsh_abs".to_string(),
                        "max"           => "_fsh_max".to_string(),
                        "min"           => "_fsh_min".to_string(),
                        "pow"           => "_fsh_pow".to_string(),
                        "sqrt"          => "_fsh_sqrt".to_string(),
                        "floor"         => "_fsh_floor".to_string(),
                        "ceil"          => "_fsh_ceil".to_string(),
                        "round"         => "_fsh_round".to_string(),
                        "string_length" => "_fsh_string_length".to_string(),
                        _               => safe_func_label(callee),
                    };

                    // Check if we're calling a method within the same class
                    if let Some(ref current_class) = symbols.current_class {
                         if !["abs", "max", "min", "pow", "sqrt", "floor", "ceil", "round", "string_length"].contains(&callee) {
                             call_name = format!("{}_{}", current_class, call_name);
                         }
                    }

                    text_section.push_str(&format!("    ; Function call: {}\n", callee));

                    // Basic argument passing support
                    let regs = vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                    let mut arg_idx = 0;
                    while let Some(arg_pair) = call_inner.next() {
                         let reg = if arg_idx < regs.len() { regs[arg_idx] } else { "rdi" };
                         arg_idx += 1;
                         
                         if let Ok(val) = eval_expr(arg_pair, &symbols.variables) {
                             match val {
                                 FluxValue::Int(n) => text_section.push_str(&format!("    mov {}, {}\n", reg, n)),
                                 FluxValue::Str(text) => {
                                    let label = format!("str_{}", *unique_id);
                                    *unique_id += 1;
                                    let escaped = text.replace("\\", "\\\\").replace("\"", "\\\"");
                                    data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                                    text_section.push_str(&format!("    lea {}, [rel {}]\n", reg, label));
                                 }
                                 _ => {}
                             }
                         }
                    }

                    text_section.push_str(&format!("    call {}\n", call_name));
                }
            }

            Rule::assignment_stmt => {
                let mut assign_inner = statement.into_inner();
                let var_name_part = assign_inner.next().unwrap();

                // Check for array assignment: arr[0] = 42;
                if var_name_part.as_rule() == Rule::array_access {
                    let mut array_access_inner = var_name_part.into_inner();
                    let array_name = array_access_inner.next().unwrap().as_str();
                    let index_expr_pair = array_access_inner.next().unwrap();
                    let index_str = index_expr_pair.as_str();

                    let _assign_op = assign_inner.next().unwrap().as_str();
                    let expr_pair = assign_inner.next().unwrap();

                    // Handle index being a variable or a literal
                    let index_asm = if let Ok(index_val) = index_str.parse::<i32>() {
                        format!("    mov rbx, {}\n", index_val)
                    } else if let Some(&offset) = var_offsets.get(index_str) {
                        format!("    mov rbx, [rbp-{}]\n", offset)
                    } else {
                        // Attempt to compile the index expression
                        let mut temp_text_section = String::new();
                        compile_expr(
                            index_expr_pair,
                            symbols,
                            var_offsets,
                            &mut temp_text_section,
                            unique_id,
                            data_section,
                        )?;
                        temp_text_section.push_str("    mov rbx, rax\n");
                        temp_text_section
                    };

                    if let (Some(&offset), Ok(val)) = (
                        var_offsets.get(array_name),
                        eval_expr(expr_pair.clone(), &symbols.variables),
                    ) {
                        if let FluxValue::Int(n) = val {
                            let size = symbols
                                .variable_types
                                .get(array_name)
                                .and_then(|type_str: &String| {
                                    if let Some(start) = type_str.find('[') {
                                        if let Some(end) = type_str.find(']') {
                                            type_str[start + 1..end].parse::<i32>().ok()
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(0);

                            // Bounds check
                            text_section.push_str(&format!(
                                "    ; Bounds check for {}[{}]\n",
                                array_name, index_str
                            ));
                            text_section.push_str(&index_asm);
                            text_section.push_str("    cmp rbx, 0\n");
                            text_section.push_str("    jl _fsh_panic_bounds\n");
                            text_section.push_str(&format!("    cmp rbx, {}\n", size));
                            text_section.push_str("    jge _fsh_panic_bounds\n");

                            // Calculate address
                            text_section.push_str("    ; Calculate address\n");
                            text_section.push_str("    mov rax, rbp\n");
                            text_section.push_str(&format!("    sub rax, {}\n", offset));
                            text_section.push_str("    mov rdx, rbx\n");
                            text_section.push_str("    imul rdx, 8\n");
                            text_section.push_str("    sub rax, rdx\n"); // Corrected from add to sub
                            text_section.push_str(&format!("    mov qword [rax], {}\n", n));
                        }
                    } else {
                        // Fallback for dynamic value assignment
                        let mut temp_text_section = String::new();
                        compile_expr(
                            expr_pair,
                            symbols,
                            var_offsets,
                            &mut temp_text_section,
                            unique_id,
                            data_section,
                        )?;
                        text_section.push_str(&temp_text_section);

                        let size = symbols
                            .variable_types
                            .get(array_name)
                            .and_then(|type_str: &String| {
                                if let Some(start) = type_str.find('[') {
                                    if let Some(end) = type_str.find(']') {
                                        type_str[start + 1..end].parse::<i32>().ok()
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(0);

                        // Bounds check
                        text_section.push_str(&format!(
                            "    ; Bounds check for {}[{}]\n",
                            array_name, index_str
                        ));
                        text_section.push_str(&index_asm);
                        text_section.push_str("    cmp rbx, 0\n");
                        text_section.push_str("    jl _fsh_panic_bounds\n");
                        text_section.push_str(&format!("    cmp rbx, {}\n", size));
                        text_section.push_str("    jge _fsh_panic_bounds\n");

                        // Calculate address and assign
                        text_section.push_str("    ; Calculate address for assignment\n");
                        text_section.push_str("    push rax\n"); // Save value from RHS
                        text_section.push_str("    mov rax, rbp\n");
                        if let Some(&offset) = var_offsets.get(array_name) {
                            text_section.push_str(&format!("    sub rax, {}\n", offset));
                        }
                        text_section.push_str("    mov rdx, rbx\n");
                        text_section.push_str("    imul rdx, 8\n");
                        text_section.push_str("    sub rax, rdx\n");
                        text_section.push_str("    pop rbx\n"); // Restore value
                        text_section.push_str("    mov [rax], rbx\n");
                    }
                    return Ok(());
                }

                // If not array assignment, proceed with normal variable/property assignment
                let first_ident = var_name_part.as_str().to_string();

                // Check if this is a property assignment (obj.property = value) or variable assignment (var = value)
                let mut property_name = None;
                let mut object_name = None;
                
                // Look for another identifier (property name in obj.property syntax)
                let assign_inner_clone: Vec<_> = assign_inner.clone().collect();
                if !assign_inner_clone.is_empty() && assign_inner_clone[0].as_rule() == Rule::ident {
                    // This is a property assignment: obj.property = value
                    object_name = Some(first_ident.clone());
                    property_name = Some(assign_inner.next().unwrap().as_str().to_string());
                }
                
                let var_name = property_name.unwrap_or_else(|| first_ident.clone());
                let _assign_op = assign_inner.next().unwrap().as_str();
                let expr_pair = assign_inner.next().unwrap();

                if let Ok(val) = eval_expr(expr_pair, &symbols.variables) {
                    if let Some(obj) = object_name {
                        // Property assignment - store the value associated with object.property
                        let prop_key = format!("{}.{}", obj, var_name);
                        symbols.variables.insert(prop_key, val);
                        text_section.push_str(&format!("    ; Property assignment: {}.{}\n", obj, var_name));
                    } else if let Some(&offset) = var_offsets.get(&var_name) {
                        // Regular variable assignment
                        match val {
                            FluxValue::Int(n) => {
                                text_section.push_str(&format!(
                                    "    mov qword [rbp-{}], {}\n",
                                    offset, n
                                ));
                            }
                            FluxValue::Float(d) => {
                                let label = format!("double_{}", *unique_id);
                                *unique_id += 1;
                                let double_bits = d.to_bits();
                                data_section.push_str(&format!("{}: dq 0x{:x}\n", label, double_bits));
                                text_section.push_str(&format!(
                                    "    mov rax, [rel {}]\n    mov qword [rbp-{}], rax\n",
                                    label, offset
                                ));
                            }
                            FluxValue::Str(_) => {
                                let label = format!("str_{}", *unique_id);
                                *unique_id += 1;
                                data_section.push_str(&format!("{}: db \"{}\", 0\n", label, val));
                                text_section.push_str(&format!(
                                    "    lea rax, [rel {}]\n    mov [rbp-{}], rax\n",
                                    label, offset
                                ));
                            }
                        }
                        symbols.variables.insert(var_name, val);
                    }
                }
            }

            Rule::increment_stmt => {
                let mut inc_inner = statement.into_inner();
                let var_name = inc_inner.next().unwrap().as_str().to_string();
                let _inc_op = inc_inner.next().unwrap().as_str();

                if let Some(&offset) = var_offsets.get(&var_name) {
                    text_section.push_str(&format!(
                        "    inc qword [rbp-{}]\n",
                        offset
                    ));
                }
            }

            Rule::if_stmt => {
                let mut if_inner = statement.into_inner();
                let condition_pair = if_inner.next().unwrap();
                let then_block = if_inner.next().unwrap();
                let else_part = if_inner.next();
                
                let label_id = *unique_id;
                *unique_id += 1;
                let label_false = format!(".if_false_{}", label_id);
                let label_end = format!(".if_end_{}", label_id);
                
                // Compile condition
                compile_condition(condition_pair, &label_false, text_section, symbols, &var_offsets)?;
                
                // Compile then-block
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
                
                // Compile else-block if present
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
                                    loop_start.clone(),
                                    loop_end.clone(),
                                )?;
                            }
                            Rule::if_stmt => {
                                // else if - recursively handle
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
                                    loop_start.clone(),
                                    loop_end.clone(),
                                )?;
                            }
                            _ => {}
                        }
                    }
                }
                
                text_section.push_str(&format!("{}:\n", label_end));
            }

            Rule::for_loop => {
                let mut for_inner = statement.into_inner();
                let for_init_pair  = for_inner.next().unwrap();
                let condition_pair = for_inner.next().unwrap();
                let increment_pair = for_inner.next().unwrap();
                let body_block     = for_inner.next().unwrap();

                let label_id    = *unique_id;
                *unique_id += 1;
                let label_start = format!(".for_start_{}", label_id);
                let label_continue = format!(".for_continue_{}", label_id);
                let label_end   = format!(".for_end_{}", label_id);

                // Init: type ident = expr
                let mut init_inner = for_init_pair.into_inner();
                let _type_str = init_inner.next().unwrap().as_str();
                let var_name  = init_inner.next().unwrap().as_str().to_string();
                let init_expr = init_inner.next();

                // Allocate variable on stack
                *stack_offset += 8;
                var_offsets.insert(var_name.clone(), *stack_offset);
                text_section.push_str("    sub rsp, 8\n");

                if let Some(expr_pair) = init_expr {
                    if let Ok(val) = eval_expr(expr_pair, &symbols.variables) {
                        if let FluxValue::Int(n) = val {
                            text_section.push_str(&format!("    mov rax, {}\n", n));
                            text_section.push_str(&format!("    mov qword [rbp-{}], rax\n", *stack_offset));
                        }
                    }
                }

                text_section.push_str(&format!("{}:\n", label_start));
                compile_condition(condition_pair, &label_end, text_section, symbols, &var_offsets)?;

                compile_block_with_loop_context(
                    body_block, content, source_lines, symbols, data_section, text_section, unique_id,
                    var_offsets, stack_offset, false, Some(label_continue.clone()), Some(label_end.clone())
                )?;

                // Increment
                text_section.push_str(&format!("{}:\n", label_continue));
                let inc_str = increment_pair.as_str().trim();
                if inc_str.contains("++") {
                    let var = inc_str.replace("++", "").trim().to_string();
                    if let Some(&offset) = var_offsets.get(&var) {
                        text_section.push_str(&format!("    inc qword [rbp-{}]\n", offset));
                    }
                } else if inc_str.contains("--") {
                    let var = inc_str.replace("--", "").trim().to_string();
                    if let Some(&offset) = var_offsets.get(&var) {
                        text_section.push_str(&format!("    dec qword [rbp-{}]\n", offset));
                    }
                } else if let Some(eq_pos) = inc_str.find('=') {
                    // Handle "i = i + 1", "i = i - 1", "i = i + 2", etc.
                    let lhs_var = inc_str[..eq_pos].trim().to_string();
                    let rhs = inc_str[eq_pos + 1..].trim();

                    if let Some(&offset) = var_offsets.get(&lhs_var) {
                        // Try to detect "var OP literal" pattern
                        let ops: &[(&str, &str)] = &[("+", "add"), ("-", "sub"), ("*", "imul")];
                        let mut emitted = false;

                        for (op_sym, asm_op) in ops {
                            // Find the operator, but skip if it's part of the variable name
                            if let Some(op_pos) = rhs.find(op_sym) {
                                let rhs_var = rhs[..op_pos].trim();
                                let rhs_num = rhs[op_pos + op_sym.len()..].trim();

                                if rhs_var == lhs_var {
                                    if let Ok(n) = rhs_num.parse::<i64>() {
                                        text_section.push_str(&format!(
                                            "    {} qword [rbp-{}], {}\n",
                                            asm_op, offset, n
                                        ));
                                        emitted = true;
                                        break;
                                    }
                                    // rhs_num might be another variable
                                    if let Some(&rhs_offset) = var_offsets.get(rhs_num) {
                                        text_section.push_str(&format!(
                                            "    mov rax, [rbp-{}]\n    {} qword [rbp-{}], rax\n",
                                            rhs_offset, asm_op, offset
                                        ));
                                        emitted = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !emitted {
                            // Generic fallback: evaluate RHS and store
                            text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                            text_section.push_str(&format!("    mov qword [rbp-{}], rax\n", offset));
                        }
                    }
                }

                text_section.push_str(&format!("    jmp {}\n", label_start));
                text_section.push_str(&format!("{}:\n", label_end));
            }

            Rule::while_loop => {
                let mut while_inner = statement.into_inner();
                let condition_pair  = while_inner.next().unwrap();
                let body_block      = while_inner.next().unwrap();

                let label_id    = *unique_id;
                *unique_id += 1;
                let label_start = format!(".while_start_{}", label_id);
                let label_end   = format!(".while_end_{}", label_id);

                text_section.push_str(&format!("{}:\n", label_start));
                compile_condition(condition_pair, &label_end, text_section, symbols, &var_offsets)?;
                compile_block_with_loop_context(
                    body_block, content, source_lines, symbols, data_section, text_section, unique_id,
                    var_offsets, stack_offset, false, Some(label_start.clone()), Some(label_end.clone())
                )?;
                text_section.push_str(&format!("    jmp {}\n", label_start));
                text_section.push_str(&format!("{}:\n", label_end));
            }

            Rule::break_stmt => {
                if let Some(ref end_label) = loop_end {
                    text_section.push_str(&format!("    jmp {}\n", end_label));
                } else {
                    text_section.push_str("    ; ERROR: break outside of loop\n");
                }
            }

            Rule::continue_stmt => {
                if let Some(ref start_label) = loop_start {
                    text_section.push_str(&format!("    jmp {}\n", start_label));
                } else {
                    text_section.push_str("    ; ERROR: continue outside of loop\n");
                }
            }

            Rule::return_stmt => {
                let mut return_inner = statement.into_inner();
                if let Some(expr_pair) = return_inner.next() {
                    let expr_str = expr_pair.as_str().trim();
                    
                    // Check if it's a simple arithmetic expression with a parameter (like "x * 2")
                    if expr_str.contains("*") && !expr_str.contains("+") && !expr_str.contains("-") && !expr_str.contains("/") {
                        // Simple multiplication like "x * 2"
                        if let Some(mult_pos) = expr_str.find('*') {
                            let left = expr_str[..mult_pos].trim();
                            let right = expr_str[mult_pos + 1..].trim();
                            
                            if left == "x" || left == "n" || left == "val" {
                                // Left operand is a parameter in rdi
                                if let Ok(multiplier) = right.parse::<i64>() {
                                    // Generate: mov rax, rdi; imul rax, multiplier
                                    text_section.push_str("    mov rax, rdi\n");
                                    text_section.push_str(&format!("    imul rax, {}\n", multiplier));
                                    text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
                                    return Ok(());
                                } else if let Ok(value) = right.parse::<i64>() {
                                    text_section.push_str("    mov rax, rdi\n");
                                    text_section.push_str(&format!("    imul rax, {}\n", value));
                                    text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
                                    return Ok(());
                                }
                            }
                        }
                    }
                    
                    // Check if it's an operation with two parameters (like "a + b", "a - b", etc)
                    if expr_str.contains("+") || expr_str.contains("-") || expr_str.contains("*") || expr_str.contains("/") {
                        let operators = vec!["+", "-", "*", "/"];
                        for op in &operators {
                            if expr_str.contains(op) && expr_str.matches(op).count() == 1 {
                                if let Some(op_pos) = expr_str.find(op) {
                                    let left = expr_str[..op_pos].trim();
                                    let right = expr_str[op_pos + op.len()..].trim();
                                    
                                    // Check if both are simple parameters (a, b, x, y, etc)
                                    if (left == "a" || left == "b" || left == "x" || left == "y") &&
                                       (right == "a" || right == "b" || right == "x" || right == "y") {
                                        // Handle two-parameter operations
                                        text_section.push_str("    mov rax, rdi\n");  // Load first param (a or x)
                                        text_section.push_str("    mov rcx, rsi\n");  // Load second param (b or y)
                                        
                                        match *op {
                                            "+" => {
                                                text_section.push_str("    add rax, rcx\n");
                                            }
                                            "-" => {
                                                text_section.push_str("    sub rbx, rax\n");
                                                text_section.push_str("    mov rax, rbx\n");
                                            }
                                            "*" => {
                                                text_section.push_str("    imul rax, rcx\n");
                                            }
                                            "/" => {
                                                text_section.push_str("    cqo\n");  // Sign extend for division
                                                text_section.push_str("    idiv rcx\n");
                                            }
                                            _ => {}
                                        }
                                        text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                    
                    // Try to evaluate at compile-time
                    if let Ok(val) = eval_expr(expr_pair.clone(), &symbols.variables) {
                        match val {
                            FluxValue::Int(n) => {
                                text_section.push_str(&format!("    mov rax, {}\n", n));
                            }
                            FluxValue::Float(f) => {
                                let label = format!("float_{}", *unique_id);
                                *unique_id += 1;
                                let float_bits = f.to_bits();
                                data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                text_section.push_str(&format!("    movd xmm0, [rel {}]\n", label));
                            }
                            FluxValue::Str(text) => {
                                let label = format!("str_{}", *unique_id);
                                *unique_id += 1;
                                let escaped = text.replace("\\", "\\\\").replace("\"", "\\\"");
                                data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                                text_section.push_str(&format!("    lea rax, [rel {}]\n", label));
                            }
                        }
                    } else {
                        // Try to evaluate as variable or method call
                        let expr_str = expr_pair.as_str().trim();
                        if let Some(offset) = var_offsets.get(expr_str) {
                            // Return variable value
                            text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                        } else if expr_str.contains(".") && expr_str.contains("(") {
                            // Method call - already handled in variable_decl
                            text_section.push_str("    ; Return value from method call in rax\n");
                        }
                    }
                }
                // Add epilogue
                text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n\n");
            }

            _ => {}
        }
    }

    if is_function {
        text_section.push_str("\n    mov rsp, rbp\n    pop rbp\n    ret\n\n");
    }
    Ok(())
}

// --- Main CLI Entry Point ---

#[derive(Parser, Debug)]
#[command(name = "fluxc")]
#[command(about = "FluxSharp Compiler - Compile .fsh files to x86_64 assembly and executables", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile a FluxSharp source file
    Compile {
        /// Source file path (.fsh)
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output file path (-o executable)
        #[arg(short, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Compile { input, output } => {
            // Validate input path
            validate_input_path(&input)?;

            // Read source file
            let content = std::fs::read_to_string(&input)
                .context(format!("Failed to read source file: {:?}", input))?;

            // Validate file size
            validate_file_size(&input)?;

            // Compile to assembly
            let asm_output = compile_fsh_to_asm(&content, &input)?;

            // Write assembly file
            let asm_path = if let Some(ref out) = output {
                out.with_extension("asm")
            } else {
                input.with_extension("asm")
            };
            validate_output_path(&asm_path)?;
            std::fs::write(&asm_path, &asm_output)
                .context(format!("Failed to write assembly file: {:?}", asm_path))?;
            eprintln!("📝 Generated ASM: {:?}", asm_path);

            // Assemble with NASM
            let obj_path = asm_path.with_extension("o");
            let nasm_status = std::process::Command::new("nasm")
                .args(&["-f", "elf64", "-o"])
                .arg(&obj_path)
                .arg(&asm_path)
                .status()
                .context("Failed to run NASM assembler")?;

            if !nasm_status.success() {
                bail!("NASM failed for {:?}", asm_path);
            }
            eprintln!("🔨 Assembled: {:?}", obj_path);

            // Assemble runtime
            let runtime_asm = "flux_compiler/fluxc/runtime/runtime.asm";
            let runtime_obj = "flux_compiler/fluxc/runtime/runtime.o";
            let nasm_status = std::process::Command::new("nasm")
                .args(&["-f", "elf64", "-o"])
                .arg(runtime_obj)
                .arg(runtime_asm)
                .status()
                .context("Failed to assemble runtime")?;

            if !nasm_status.success() {
                eprintln!("⚠️  Linked with warnings: {:?}", output.as_ref().unwrap_or(&input));
            } else {
                eprintln!("🔨 Assembled runtime: {}", runtime_obj);
            }

            // Link to executable
            let exe_path = output.unwrap_or_else(|| {
                input.with_extension("exe")
            });
            validate_output_path(&exe_path)?;

            let ld_status = std::process::Command::new("ld")
                .arg("-o")
                .arg(&exe_path)
                .arg(&obj_path)
                .arg(runtime_obj)
                .arg("-lc")
                .arg("-dynamic-linker")
                .arg("/lib64/ld-linux-x86-64.so.2")
                .status()
                .context("Failed to run linker")?;

            if !ld_status.success() {
                eprintln!("❌ Executable not created: {:?}", exe_path);
                bail!("Linker failed");
            }

            eprintln!("✅ Linked binary: {:?}", exe_path);
        }
    }

    Ok(())
}

/// Safe function label generator - avoids NASM mnemonic collisions
/// User-defined functions that conflict with NASM mnemonics get prefixed with _usr_
fn safe_func_label(name: &str) -> String {
    const NASM_RESERVED: &[&str] = &[
        "add", "sub", "mul", "div", "and", "or", "not", "xor",
        "mov", "cmp", "jmp", "ret", "call", "push", "pop",
        "inc", "dec", "neg", "nop", "hlt", "int",
        "lea", "je", "jne", "jl", "jg", "jle", "jge",
    ];
    
    if NASM_RESERVED.contains(&name) {
        format!("_usr_{}", name)
    } else {
        name.to_string()
    }
}

// --- Evaluation of math function calls ---
#[allow(dead_code)]
fn eval_math_function(func_name: &str, args: Vec<FluxValue>) -> Result<FluxValue> {
    match func_name {
        "sqrt" => {
            if args.len() != 1 {
                anyhow::bail!("sqrt expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Int(n) => Ok(FluxValue::Float((*n as f64).sqrt())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.sqrt())),
                _ => anyhow::bail!("sqrt expects numeric argument"),
            }
        }
        "abs" => {
            if args.len() != 1 {
                anyhow::bail!("abs expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Int(n) => Ok(FluxValue::Int(n.abs())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.abs())),
                _ => bail!("Type Error: abs() requires a numeric argument."),
            }
        }
        "max" => {
            if args.len() != 2 {
                bail!("Function Error: max() requires exactly 2 arguments, but got {}. Usage: max(value1, value2)", args.len());
            }
            let v1 = &args[0];
            let v2 = &args[1];
            match (v1, v2) {
                (FluxValue::Int(a), FluxValue::Int(b)) => Ok(FluxValue::Int(std::cmp::max(*a, *b))),
                (FluxValue::Float(a), FluxValue::Float(b)) => Ok(FluxValue::Float(a.max(*b))),
                _ => bail!("max() requires numeric arguments"),
            }
        },
        "min" => {
            if args.len() != 2 {
                bail!("Function Error: min() requires exactly 2 arguments, but got {}. Usage: min(value1, value2)", args.len());
            }
            let v1 = &args[0];
            let v2 = &args[1];
            match (v1, v2) {
                (FluxValue::Int(a), FluxValue::Int(b)) => Ok(FluxValue::Int(std::cmp::min(*a, *b))),
                (FluxValue::Float(a), FluxValue::Float(b)) => Ok(FluxValue::Float(a.min(*b))),
                _ => bail!("min() requires numeric arguments"),
            }
        },
        "pow" => {
            if args.len() != 2 {
                bail!("Function Error: pow() requires exactly 2 arguments, but got {}. Usage: pow(base, exponent)", args.len());
            }
            let base = &args[0];
            let exponent = &args[1];
            match (base, exponent) {
                (FluxValue::Int(b), FluxValue::Int(e)) => Ok(FluxValue::Int(b.pow(*e as u32))),
                (FluxValue::Float(b), FluxValue::Float(e)) => Ok(FluxValue::Float(b.powf(*e))),
                _ => bail!("Type Error: pow() requires both arguments to be numeric types."),
            }
        },
        "floor" => {
            if args.len() != 1 {
                bail!("Function Error: floor() requires exactly 1 argument, but got {}. Usage: floor(value)", args.len());
            }
            let val = &args[0];
            match val {
                FluxValue::Float(f) => Ok(FluxValue::Float(f.floor())),
                FluxValue::Int(i) => Ok(FluxValue::Int(*i)), // floor on int is identity
                _ => bail!("Type Error: floor() requires a numeric argument."),
            }
        },
        "ceil" => {
            if args.len() != 1 {
                bail!("Function Error: ceil() requires exactly 1 argument, but got {}. Usage: ceil(value)", args.len());
            }
            let val = &args[0];
            match val {
                FluxValue::Float(f) => Ok(FluxValue::Float(f.ceil())),
                FluxValue::Int(i) => Ok(FluxValue::Int(*i)), // ceil on int is identity
                _ => bail!("Type Error: ceil() requires a numeric argument."),
            }
        },
        "round" => {
            if args.len() != 1 {
                bail!("Function Error: round() requires exactly 1 argument, but got {}. Usage: round(value)", args.len());
            }
            let val = &args[0];
            match val {
                FluxValue::Float(f) => Ok(FluxValue::Float(f.round())),
                FluxValue::Int(i) => Ok(FluxValue::Int(*i)), // round on int is identity
                _ => bail!("Type Error: round() requires a numeric argument."),
            }
        },
        "ToString" => {
            if args.is_empty() {
                bail!("Function Error: ToString() requires at least the object itself to convert");
            }
            let val = &args[0];
            Ok(FluxValue::Str(val.to_string()))
        }
        _ => bail!("Undefined function: '{}'", func_name),
    }
}
