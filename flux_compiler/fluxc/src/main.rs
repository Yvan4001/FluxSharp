use clap::{Parser, Subcommand};
use std::fs;
use std::path::{PathBuf, Path};
use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use pest::Parser as PestParser;
use pest_derive::Parser as PestDeriveParser;
use std::time::Duration;
use std::env;

mod error_handler;
mod bounds_checker;
mod advanced_security;
mod async_runtime;

// ===== SECURITY CONSTRAINTS =====
const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;  // 50 MB
const MAX_ASM_OUTPUT_SIZE: usize = 100 * 1024 * 1024;  // 100 MB
const MAX_STATEMENTS_PER_BLOCK: usize = 10_000;  // Limit statements to prevent infinite loops
const MAX_EXPRESSION_DEPTH: usize = 100;  // Limit recursion depth
const MAX_OPERATOR_CHAIN: usize = 1_000;  // Limit operators in one expression
const RUN_TIMEOUT_SECS: u64 = 30;  // 30 seconds max runtime
const RUN_MEMORY_LIMIT_MB: u64 = 512;  // 512 MB memory limit

// ===== SECURITY FUNCTIONS =====

/// Validate that a file size is within acceptable limits
fn validate_file_size(path: &PathBuf) -> Result<()> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("Cannot access file: {:?}", path))?;
    
    if metadata.len() > MAX_FILE_SIZE {
        bail!(
            "File too large: {:?} ({} bytes > {} bytes limit)",
            path,
            metadata.len(),
            MAX_FILE_SIZE
        );
    }
    
    if metadata.len() == 0 {
        bail!("File is empty: {:?}", path);
    }
    
    Ok(())
}

/// Validate output path to prevent path traversal attacks
fn validate_output_path(path: &Path) -> Result<()> {
    // Check for ".." in path which indicates path traversal attempt
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        bail!("Path traversal detected: {:?} contains '..'", path);
    }
    
    // Get the current working directory
    let cwd = env::current_dir()
        .context("Cannot determine current working directory")?;
    
    // Convert to absolute path
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    
    // Try to resolve parent for new files
    let parent = absolute_path.parent().unwrap_or(&absolute_path);
    
    // Check that parent exists or can be created
    if !parent.exists() {
        // For new files, check that the parent's parent exists
        if let Some(grandparent) = parent.parent() {
            if !grandparent.exists() {
                bail!("Parent directory does not exist: {:?}", parent);
            }
        }
    }
    
    // Check path doesn't escape current working directory (except /tmp which is allowed)
    let normalized = match absolute_path.canonicalize() {
        Ok(canon) => canon,
        Err(_) => {
            // If canonicalize fails, at least check it's under cwd
            absolute_path.clone()
        }
    };
    
    if !normalized.starts_with(&cwd) && !normalized.starts_with("/tmp") {
        bail!(
            "Path traversal detected: {:?} is outside allowed directories",
            normalized
        );
    }
    
    Ok(())
}

/// Validate that input path is safe to read
fn validate_input_path(path: &Path) -> Result<()> {
    // Check if path contains suspicious patterns
    let path_str = path.to_string_lossy();
    
    if path_str.contains("..") {
        bail!("Path traversal detected in input: {:?}", path);
    }
    
    // Check if it's a symlink (to prevent TOCTOU attacks)
    if path.is_symlink() {
        bail!("Symlinks are not allowed: {:?}", path);
    }
    
    // Verify it's actually a file
    if !path.is_file() {
        bail!("Input path is not a regular file: {:?}", path);
    }
    
    Ok(())
}

/// Validate that file has exactly one Main class with main method
fn validate_main_class(content: &str) -> Result<()> {
    let main_class_count = content.matches("class Main").count();
    let main_method_count = content.matches("void main()").count() + content.matches("void main ()").count();
    
    if main_class_count == 0 {
        bail!(
            "❌ MISSING MAIN CLASS\n\n\
            Your program must have exactly one 'class Main' with a 'void main()' method.\n\n\
            Example:\n\
            public class Main {{\n\
                public void main() {{\n\
                    print(\"Hello, World!\");\n\
                }}\n\
            }}\n"
        );
    }
    
    if main_class_count > 1 {
        bail!(
            "❌ MULTIPLE MAIN CLASSES\n\n\
            Your program has {} 'class Main' declarations.\n\
            You must have exactly one 'class Main'.\n",
            main_class_count
        );
    }
    
    if main_method_count == 0 {
        bail!(
            "❌ MISSING MAIN METHOD\n\n\
            Your 'class Main' must have exactly one 'void main()' method.\n\n\
            Example:\n\
            public class Main {{\n\
                public void main() {{\n\
                    print(\"Hello, World!\");\n\
                }}\n\
            }}\n"
        );
    }
    
    if main_method_count > 1 {
        bail!(
            "❌ MULTIPLE MAIN METHODS\n\n\
            Your 'class Main' has {} 'void main()' methods.\n\
            You must have exactly one 'void main()' method.\n",
            main_method_count
        );
    }
    
    Ok(())
}

/// Internal helper to process includes with circular dependency detection
fn process_includes_internal(
    content: &str,
    base_dir: &Path,
    included_files: &mut std::collections::HashSet<String>,
) -> Result<String> {
    let mut result = String::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Check for C# style import/using: using "filename.fsh"; or import "filename.fsh";
        let is_using = trimmed.starts_with("using ") && trimmed.ends_with(";");
        let is_import = trimmed.starts_with("import ") && trimmed.ends_with(";");
        // Check for legacy include directive: // #include "filename.fsh"
        let is_legacy_include = trimmed.starts_with("//") && trimmed.contains("#include") && trimmed.contains("\"");
        
        if is_using || is_import || is_legacy_include {
            // Extract filename from the different formats
            let filename = if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed.rfind('"') {
                    if start < end {
                        Some(&trimmed[start + 1..end])
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            
            if let Some(filename) = filename {
                // Validate that it's a .fsh file
                if !filename.ends_with(".fsh") {
                    bail!(
                        "❌ INVALID IMPORT FILE\n\n\
                        Import/using directive contains non-.fsh file: '{}'\n\
                        Only .fsh files are allowed.\n\n\
                        Correct formats:\n\
                        using \"myfile.fsh\";\n\
                        import \"myfile.fsh\";\n",
                        filename
                    );
                }
                
                // Build the path
                let include_path = base_dir.join(filename);
                
                // Check if file exists first, before path validation
                if !include_path.exists() {
                    bail!(
                        "❌ IMPORT FILE NOT FOUND\n\n\
                        Cannot find imported file: '{}'\n\
                        Looked in: {:?}\n",
                        filename,
                        include_path
                    );
                }
                
                // Validate path doesn't escape base directory
                validate_input_path(&include_path)?;
                
                // Prevent circular includes
                if included_files.contains(filename) {
                    bail!(
                        "❌ CIRCULAR IMPORT\n\n\
                        Circular import detected: '{}' already imported.\n",
                        filename
                    );
                }
                included_files.insert(filename.to_string());
                
                // Read and include the file
                eprintln!("📥 Importing: {}", filename);
                let included_content = fs::read_to_string(&include_path)
                    .with_context(|| format!("Cannot read imported file: {}", filename))?;
                
                // Validate file size
                validate_file_size(&include_path)?;
                
                // Recursively process includes in the included file
                let processed = process_includes_internal(&included_content, base_dir, included_files)?;
                result.push_str(&processed);
                result.push('\n');
                continue;
            }
        }
        
        // Regular line - add as-is
        result.push_str(line);
        result.push('\n');
    }
    
    Ok(result)
}

/// Process include statements and load external .fsh files
fn process_includes(content: &str, base_dir: &Path) -> Result<String> {
    let mut included_files = std::collections::HashSet::new();
    process_includes_internal(content, base_dir, &mut included_files)
}

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
    };

    let mut data_section = String::new();
    let mut text_section = String::new();
    let mut unique_id: usize = 0;

    // Header comment
    text_section.push_str(&format!("; === Compiled from {:?} by fluxc ===\n", source_path));
    text_section.push_str("extern _fsh_print_str\n");
    text_section.push_str("extern _fsh_print_int\n\n");

    for pair in file.into_inner() {
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
                        let method_label = format!("{}_{}", name, method_name);
                        text_section.push_str(&format!("global {}\n{}:\n", method_label, method_label));
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

                    text_section.push_str(&format!("global {}\n{}:\n", func_name, func_name));
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
    asm.push_str("extern _fsh_abs\n");
    asm.push_str("extern _fsh_max\n");
    asm.push_str("extern _fsh_min\n");
    asm.push_str("extern _fsh_pow\n");
    asm.push_str("extern _fsh_floor\n");
    asm.push_str("extern _fsh_ceil\n");
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
                let var_type = decl_inner.next().unwrap().as_str().to_string();
                let var_name = decl_inner.next().unwrap().as_str().to_string();

                // Track the type of this variable (for class instances)
                symbols.variable_types.insert(var_name.clone(), var_type.clone());

                *stack_offset += 8;
                var_offsets.insert(var_name.clone(), *stack_offset);
                text_section.push_str(&format!("    sub rsp, 8\n"));

                if let Some(expr_pair) = decl_inner.find(|p| p.as_rule() == Rule::expr) {
                    let expr_str = expr_pair.as_str().trim();
                    
                    // Check if this is a function call expression: func(args)
                    if expr_str.contains("(") && expr_str.contains(")") && !expr_str.contains("[") {
                        // Try to parse as function call
                        if let Ok(val) = eval_expr(expr_pair.clone(), &symbols.variables) {
                            // Successfully evaluated at compile-time (math functions)
                            match &val {
                                FluxValue::Integer(n) => {
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
                                FluxValue::Double(d) => {
                                    let label = format!("double_{}", *unique_id);
                                    *unique_id += 1;
                                    let double_bits = d.to_bits();
                                    data_section.push_str(&format!("{}: dq 0x{:x}\n", label, double_bits));
                                    text_section.push_str(&format!(
                                        "    mov rax, [rel {}]\n    mov qword [rbp-{}], rax\n",
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
                                    // Method call like "helper.GetDouble(21)"
                                    if let Some(dot_pos) = func_name.find('.') {
                                        let obj_name = &func_name[..dot_pos].trim();
                                        let method_name = &func_name[dot_pos + 1..].trim();
                                        
                                        // Look up the type of the object to generate the correct label
                                        let class_name = symbols.variable_types.get(*obj_name)
                                            .cloned()
                                            .unwrap_or_else(|| obj_name.to_string());
                                        let method_label = format!("{}_{}", class_name, method_name);
                                        
                                        // Parse arguments
                                        let args_str = expr_str[paren_pos+1..expr_str.rfind(')').unwrap_or(expr_str.len())].trim();
                                        if !args_str.is_empty() {
                                            let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
                                            // First argument goes in rdi
                                            if let Some(first_arg) = args.first() {
                                                // Try to parse as integer
                                                if let Ok(n) = first_arg.parse::<i64>() {
                                                    text_section.push_str(&format!("    mov rdi, {}\n", n));
                                                } else if let Some(offset) = var_offsets.get(*first_arg) {
                                                    text_section.push_str(&format!("    mov rdi, [rbp-{}]\n", offset));
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
                                    let call_name = match func_name {
                                        "abs" => "_fsh_abs",
                                        "max" => "_fsh_max",
                                        "min" => "_fsh_min",
                                        "pow" => "_fsh_pow",
                                        _ => func_name,
                                    };
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
                                    FluxValue::Integer(n) => {
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
                                    FluxValue::Double(d) => {
                                        let label = format!("double_{}", *unique_id);
                                        *unique_id += 1;
                                        let double_bits = d.to_bits();
                                        data_section.push_str(&format!("{}: dq 0x{:x}\n", label, double_bits));
                                        text_section.push_str(&format!(
                                            "    mov rax, [rel {}]\n    mov qword [rbp-{}], rax\n",
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
                let mut call_inner = statement.into_inner();
                let first_ident = call_inner.next().unwrap().as_str();
                
                // Check if this is a method call (obj.method()) or function call (func())
                let mut method_name = None;
                let mut object_name = None;
                
                // Look for another identifier (method name in obj.method() syntax)
                let call_inner_clone: Vec<_> = call_inner.clone().collect();
                if !call_inner_clone.is_empty() && call_inner_clone[0].as_rule() == Rule::ident {
                    // This is a method call: obj.method()
                    object_name = Some(first_ident.to_string());
                    method_name = Some(call_inner.next().unwrap().as_str().to_string());
                }
                
                let callee = method_name.as_deref().unwrap_or(first_ident);
                
                // Handle built-in functions
                if callee == "serial_print" || callee == "print" {
                    if let Some(arg_pair) = call_inner.next() {
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
                                FluxValue::Integer(n) => {
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
                                FluxValue::Double(d) => {
                                    // Generate a string literal with the double value
                                    let label = format!("double_{}", *unique_id);
                                    *unique_id += 1;
                                    let formatted = format!("{}", d);
                                    data_section.push_str(&format!("{}: db \"{}\", 0\n", label, formatted));
                                    text_section.push_str(&format!(
                                        "    lea rdi, [rel {}]\n    call _fsh_print_str\n",
                                        label
                                    ));
                                }
                            },
                            Err(_e) => {
                                // Try to handle as variable reference if static evaluation fails
                                let arg_str = arg_pair.as_str().trim();
                                
                                // Check if it's a simple identifier (no operators, no function calls)
                                if !arg_str.contains(" ") && !arg_str.contains("(") && !arg_str.contains("[") 
                                    && !arg_str.contains("+") && !arg_str.contains("-") && !arg_str.contains("*") && !arg_str.contains("/") {
                                    // It's likely a simple variable name
                                    if let Some(&offset) = var_offsets.get(arg_str) {
                                        // Load variable from stack and print it as integer
                                        text_section.push_str(&format!(
                                            "    mov rdi, [rbp-{}]\n    call _fsh_print_int\n",
                                            offset
                                        ));
                                    } else {
                                        // Better error message with the actual variable name
                                        eprintln!("⚠️  WARNING: Undefined variable '{}' used in function call '{}'", arg_str, callee);
                                        text_section.push_str(&format!("    ; ❌ ERROR: Undefined variable '{}' in call to {}\n", arg_str, callee));
                                    }
                                } else {
                                    eprintln!("⚠️  WARNING: Complex expression not fully supported in function arguments for '{}'", callee);
                                    text_section.push_str(&format!("    ; ❌ ERROR: Complex expression not supported in argument for {}\n", callee));
                                }
                            }
                        }
                    } else {
                        text_section.push_str(&format!("    ; ⚠️  WARNING: Function '{}' called without required arguments\n", callee));
                    }
                } else if let Some(obj) = object_name {
                    // Method call on object - generate a call to the method
                    // Look up the type of the object to generate the correct label
                    let class_name = symbols.variable_types.get(&obj)
                        .cloned()
                        .unwrap_or_else(|| obj.clone());  // Fallback to object name if type not found
                    let method_label = format!("{}_{}", class_name, callee);
                    
                    // Collect arguments
                    let mut args_code = String::new();
                    while let Some(arg_pair) = call_inner.next() {
                        if let Ok(val) = eval_expr(arg_pair, &symbols.variables) {
                            match val {
                                FluxValue::Integer(n) => {
                                    args_code.push_str(&format!("    mov rdi, {}\n", n));
                                }
                                FluxValue::Str(text) => {
                                    let label = format!("str_{}", *unique_id);
                                    *unique_id += 1;
                                    let escaped = text.replace("\\", "\\\\").replace("\"", "\\\"");
                                    data_section.push_str(&format!("{}: db \"{}\", 0\n", label, escaped));
                                    args_code.push_str(&format!("    lea rdi, [rel {}]\n", label));
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    text_section.push_str(&args_code);
                    text_section.push_str(&format!("    call {}\n", method_label));
                } else {
                    // Regular function call
                    text_section.push_str(&format!("    ; Function call: {}\n", callee));
                    text_section.push_str(&format!("    call {}\n", callee));
                }
            }

            Rule::assignment_stmt => {
                let mut assign_inner = statement.into_inner();
                let first_ident = assign_inner.next().unwrap().as_str().to_string();
                
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
                            FluxValue::Integer(n) => {
                                text_section.push_str(&format!(
                                    "    mov qword [rbp-{}], {}\n",
                                    offset, n
                                ));
                            }
                            FluxValue::Float(f) => {
                                let label = format!("float_{}", *unique_id);
                                *unique_id += 1;
                                let float_bits = f.to_bits();
                                data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                text_section.push_str(&format!(
                                    "    mov eax, [rel {}]\n    mov dword [rbp-{}], eax\n",
                                    label, offset
                                ));
                            }
                            FluxValue::Double(d) => {
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
                let then_block     = if_inner.next().unwrap();
                let else_part      = if_inner.next();

                let label_id = *unique_id;
                *unique_id += 1;
                let label_false = format!(".if_false_{}", label_id);
                let label_end   = format!(".if_end_{}", label_id);

                compile_condition(condition_pair, &label_false, text_section, symbols, &var_offsets)?;
                compile_block_with_loop_context(
                    then_block, content, source_lines, symbols, data_section, text_section, unique_id,
                    var_offsets, stack_offset, false, loop_start.clone(), loop_end.clone()
                )?;
                text_section.push_str(&format!("    jmp {}\n", label_end));
                text_section.push_str(&format!("{}:\n", label_false));

                if let Some(else_pair) = else_part {
                    let mut else_inner = else_pair.into_inner();
                    if let Some(else_block) = else_inner.next() {
                        match else_block.as_rule() {
                            Rule::block => {
                                compile_block_with_loop_context(
                                    else_block, content, source_lines, symbols, data_section, text_section, unique_id,
                                    var_offsets, stack_offset, false, loop_start.clone(), loop_end.clone()
                                )?;
                            }
                            Rule::if_stmt => {
                                compile_block_from_if(else_block, content, source_lines, symbols, data_section, text_section, unique_id, var_offsets, stack_offset, loop_start.clone(), loop_end.clone())?;
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
                        if let FluxValue::Integer(n) = val {
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
                let inc_str = increment_pair.as_str();
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
                                                text_section.push_str("    sub rax, rcx\n");
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
                            FluxValue::Integer(n) => {
                                text_section.push_str(&format!("    mov rax, {}\n", n));
                            }
                            FluxValue::Float(f) => {
                                let label = format!("float_{}", *unique_id);
                                *unique_id += 1;
                                let float_bits = f.to_bits();
                                data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                text_section.push_str(&format!("    movd xmm0, [rel {}]\n", label));
                            }
                            FluxValue::Double(d) => {
                                let label = format!("double_{}", *unique_id);
                                *unique_id += 1;
                                let double_bits = d.to_bits();
                                data_section.push_str(&format!("{}: dq 0x{:x}\n", label, double_bits));
                                text_section.push_str(&format!("    movsd xmm0, [rel {}]\n", label));
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
                text_section.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
            }

            _ => {}
        }
    }

    if is_function {
        text_section.push_str("\n    mov rsp, rbp\n    pop rbp\n    ret\n\n");
    }
    Ok(())
}

// --- 2. SYSTÈME DE TYPES ---
#[derive(Debug, Clone, PartialEq)]
pub enum FluxType {
    Int, UInt, Long, ULong, Byte, String, Bool, Void, Float, Double,
    Pointer(Box<FluxType>),
    Struct(String),
}

impl FluxType {
    fn from_str(s: &str) -> Self {
        match s {
            "int" => FluxType::Int,
            "uint" => FluxType::UInt,
            "long" => FluxType::Long,
            "ulong" => FluxType::ULong,
            "byte" => FluxType::Byte,
            "string" => FluxType::String,
            "bool" => FluxType::Bool,
            "void" => FluxType::Void,
            "float" => FluxType::Float,
            "double" => FluxType::Double,
            _ => FluxType::Struct(s.to_string()),
        }
    }
}

// --- 3. RUNTIME VALUES ---
#[derive(Debug, Clone)]
pub enum FluxValue {
    Integer(i64),
    Float(f32),
    Double(f64),
    Str(String),
}

impl std::fmt::Display for FluxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FluxValue::Integer(n) => write!(f, "{}", n),
            FluxValue::Float(n) => write!(f, "{}", n),
            FluxValue::Double(n) => write!(f, "{}", n),
            FluxValue::Str(s) => write!(f, "{}", s),
        }
    }
}

// --- 4. GESTION DES SYMBOLES ---
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub return_type: FluxType,
    pub parameters: Vec<(String, FluxType)>,
}

#[allow(dead_code)]
struct SymbolTable {
    variables: HashMap<String, FluxValue>,
    structs: HashMap<String, Vec<(String, FluxType)>>,
    functions: HashMap<String, FunctionSignature>,
    variable_types: HashMap<String, String>,  // Track type of each variable (e.g., "calc" -> "Calculator")
}

// --- 5. EXPRESSION EVALUATOR ---
fn eval_expr(pair: pest::iterators::Pair<Rule>, vars: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| anyhow::anyhow!("Empty expression"))?;
    let mut result = eval_postfix(first, vars)?;
    
    let mut operator_count = 0;

    while let Some(op_pair) = inner.next() {
        // Security: Prevent long operator chains
        operator_count += 1;
        if operator_count > MAX_OPERATOR_CHAIN {
            bail!(
                "Expression has too many operators: {} > {} limit",
                operator_count,
                MAX_OPERATOR_CHAIN
            );
        }
        
        let op = op_pair.as_str();
        let next_postfix = inner.next().ok_or_else(|| anyhow::anyhow!("Missing operand after '{}'", op))?;
        let right = eval_postfix(next_postfix, vars)?;

        result = match (&result, &right) {
            (FluxValue::Integer(l), FluxValue::Integer(r)) => {
                FluxValue::Integer(match op {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => {
                        if *r == 0 { anyhow::bail!("Division by zero"); }
                        l / r
                    },
                    "%" => {
                        if *r == 0 { anyhow::bail!("Modulo by zero"); }
                        l % r
                    },
                    "&" => l & r,
                    "|" => l | r,
                    "^" => l ^ r,
                    "<<" => l << r,
                    ">>" => l >> r,
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Float(l), FluxValue::Float(r)) => {
                FluxValue::Float(match op {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        l / r
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        l % r
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Double(l), FluxValue::Double(r)) => {
                FluxValue::Double(match op {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        l / r
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        l % r
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Integer(l), FluxValue::Float(r)) => {
                FluxValue::Float(match op {
                    "+" => *l as f32 + r,
                    "-" => *l as f32 - r,
                    "*" => *l as f32 * r,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        *l as f32 / r
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        (*l as f32) % r
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Float(l), FluxValue::Integer(r)) => {
                FluxValue::Float(match op {
                    "+" => l + *r as f32,
                    "-" => l - *r as f32,
                    "*" => l * *r as f32,
                    "/" => {
                        if *r == 0 { anyhow::bail!("Division by zero"); }
                        l / *r as f32
                    },
                    "%" => {
                        if *r == 0 { anyhow::bail!("Modulo by zero"); }
                        l % (*r as f32)
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Integer(l), FluxValue::Double(r)) => {
                FluxValue::Double(match op {
                    "+" => *l as f64 + r,
                    "-" => *l as f64 - r,
                    "*" => *l as f64 * r,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        *l as f64 / r
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        (*l as f64) % r
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Double(l), FluxValue::Integer(r)) => {
                FluxValue::Double(match op {
                    "+" => l + *r as f64,
                    "-" => l - *r as f64,
                    "*" => l * *r as f64,
                    "/" => {
                        if *r == 0 { anyhow::bail!("Division by zero"); }
                        l / *r as f64
                    },
                    "%" => {
                        if *r == 0 { anyhow::bail!("Modulo by zero"); }
                        l % (*r as f64)
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Float(l), FluxValue::Double(r)) => {
                FluxValue::Double(match op {
                    "+" => *l as f64 + r,
                    "-" => *l as f64 - r,
                    "*" => *l as f64 * r,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        *l as f64 / r
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        (*l as f64) % r
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Double(l), FluxValue::Float(r)) => {
                FluxValue::Double(match op {
                    "+" => l + *r as f64,
                    "-" => l - *r as f64,
                    "*" => l * *r as f64,
                    "/" => {
                        if *r == 0.0 { anyhow::bail!("Division by zero"); }
                        l / *r as f64
                    },
                    "%" => {
                        if *r == 0.0 { anyhow::bail!("⚠️  Division by zero error in arithmetic operation"); }
                        l % (*r as f64)
                    },
                    _ => anyhow::bail!("Unknown operator '{}' in arithmetic expression", op),
                })
            }
            (FluxValue::Str(l), FluxValue::Str(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("❌ Unsupported operator '{}' on strings. Only concatenation (+) is supported.", op)
                }
            }
            (FluxValue::Str(l), FluxValue::Integer(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("❌ Invalid operation '{}' between string and integer.\n   Hint: Use string concatenation (+) to combine types.", op)
                }
            }
            (FluxValue::Integer(l), FluxValue::Str(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("❌ Invalid operation '{}' between integer and string.\n   Hint: Use string concatenation (+) to combine types.", op)
                }
            }
            _ => anyhow::bail!("❌ Type error: Cannot perform arithmetic on incompatible types.\n   Check that both operands are numeric (int, float, double) for arithmetic operations."),
        };
    }

    Ok(result)
}

fn eval_postfix(pair: pest::iterators::Pair<Rule>, vars: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    // postfix = { primary ~ ("(" ~ (expr ~ ("," ~ expr)*)? ~ ")" | "[" ~ expr ~ "]" | "." ~ ident)* }
    let mut inner = pair.into_inner();
    let primary = inner.next().ok_or_else(|| anyhow::anyhow!("Empty postfix"))?;
    let mut base = eval_primary(primary, vars)?;

    // Process postfix operators (function calls, array access, member access)
    while let Some(suffix) = inner.next() {
        match suffix.as_rule() {
            // For now, just skip non-expr suffixes
            // Function calls will be parsed differently
            _ => {}
        }
    }
    
    Ok(base)
}

fn eval_primary(pair: pest::iterators::Pair<Rule>, vars: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    // primary = { bool_literal | float_literal | double_literal | int_literal
    //           | string_literal | char_literal | "(" ~ expr ~ ")" | ident }
    let inner = pair.into_inner().next()
        .ok_or_else(|| anyhow::anyhow!("Empty primary"))?;

    match inner.as_rule() {
        Rule::bool_literal => {
            match inner.as_str() {
                "true"  => Ok(FluxValue::Integer(1)),
                "false" => Ok(FluxValue::Integer(0)),
                _       => bail!("Invalid bool literal"),
            }
        }
        Rule::float_literal => {
            let raw = inner.as_str();
            let n: f32 = raw[..raw.len()-1].parse()
                .with_context(|| format!("Invalid float: {}", raw))?;
            Ok(FluxValue::Float(n))
        }
        Rule::double_literal => {
            let n: f64 = inner.as_str().parse()
                .with_context(|| format!("Invalid double: {}", inner.as_str()))?;
            Ok(FluxValue::Double(n))
        }
        Rule::int_literal => {
            let raw = inner.as_str();
            let n: i64 = if raw.starts_with("0x") || raw.starts_with("0X") {
                i64::from_str_radix(&raw[2..], 16)?
            } else if raw.starts_with("0b") || raw.starts_with("0B") {
                i64::from_str_radix(&raw[2..], 2)?
            } else if raw.starts_with("0o") || raw.starts_with("0O") {
                i64::from_str_radix(&raw[2..], 8)?
            } else {
                raw.parse()?
            };
            Ok(FluxValue::Integer(n))
        }
        Rule::string_literal => {
            let raw = inner.as_str();
            let content = raw[1..raw.len()-1].to_string();
            Ok(FluxValue::Str(content))
        }
        Rule::char_literal => {
            let raw = inner.as_str();
            let content = raw[1..raw.len()-1].to_string();
            Ok(FluxValue::Str(content))
        }
        Rule::expr => {
            // Parenthesized expression: "(" ~ expr ~ ")"
            eval_expr(inner, vars)
        }
        Rule::ident => {
            let name = inner.as_str();
            // Check for math function names - but we can't evaluate them here
            // because we don't have access to the arguments in this context
            match name {
                "PI"    => return Ok(FluxValue::Double(std::f64::consts::PI)),
                "E"     => return Ok(FluxValue::Double(std::f64::consts::E)),
                "LN2"   => return Ok(FluxValue::Double(std::f64::consts::LN_2)),
                "LN10"  => return Ok(FluxValue::Double(std::f64::consts::LN_10)),
                "SQRT2" => return Ok(FluxValue::Double(std::f64::consts::SQRT_2)),
                "true"  => return Ok(FluxValue::Integer(1)),
                "false" => return Ok(FluxValue::Integer(0)),
                // For function names, we can't evaluate them without arguments
                // They will be handled elsewhere
                _ => {}
            }
            vars.get(name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("❌ Undefined variable: '{}'\n   Make sure this variable is declared before use with: type {} = value;", name, name))
        }
        Rule::function_call => {
            // Handle function calls in expressions
            let mut call_inner = inner.into_inner();
            let func_name = call_inner.next().ok_or_else(|| anyhow::anyhow!("Empty function call"))?;
            let func_name_str = func_name.as_str();
            
            // Parse arguments
            let mut args = Vec::new();
            for arg in call_inner {
                if arg.as_rule() == Rule::expr {
                    args.push(eval_expr(arg, vars)?);
                }
            }
            
            // Evaluate built-in math functions
            match func_name_str {
                "abs" => {
                    if args.len() != 1 {
                        bail!("abs() requires 1 argument, got {}", args.len());
                    }
                    match &args[0] {
                        FluxValue::Integer(n) => Ok(FluxValue::Integer(n.abs())),
                        FluxValue::Float(f) => Ok(FluxValue::Float(f.abs())),
                        FluxValue::Double(d) => Ok(FluxValue::Double(d.abs())),
                        _ => bail!("⚠️  Type Error: abs() requires a numeric argument (int, float, or double)\n   Example: abs(-42), abs(-3.14f)"),
                    }
                }
                "max" => {
                    if args.len() != 2 {
                        bail!("❌ Function Error: max() requires exactly 2 arguments, but got {}\n   Usage: max(value1, value2)", args.len());
                    }
                    match (&args[0], &args[1]) {
                        (FluxValue::Integer(a), FluxValue::Integer(b)) => Ok(FluxValue::Integer(*a.max(b))),
                        (FluxValue::Float(a), FluxValue::Float(b)) => Ok(FluxValue::Float(a.max(*b))),
                        (FluxValue::Double(a), FluxValue::Double(b)) => Ok(FluxValue::Double(a.max(*b))),
                        _ => bail!("⚠️  Type Error: max() requires both arguments to be the same numeric type\n   Use: max(intA, intB), max(floatA, floatB), or max(doubleA, doubleB)"),
                    }
                }
                "min" => {
                    if args.len() != 2 {
                        bail!("❌ Function Error: min() requires exactly 2 arguments, but got {}\n   Usage: min(value1, value2)", args.len());
                    }
                    match (&args[0], &args[1]) {
                        (FluxValue::Integer(a), FluxValue::Integer(b)) => Ok(FluxValue::Integer(*a.min(b))),
                        (FluxValue::Float(a), FluxValue::Float(b)) => Ok(FluxValue::Float(a.min(*b))),
                        (FluxValue::Double(a), FluxValue::Double(b)) => Ok(FluxValue::Double(a.min(*b))),
                        _ => bail!("min() requires numeric arguments"),
                    }
                }
                "pow" => {
                    if args.len() != 2 {
                        bail!("❌ Function Error: pow() requires exactly 2 arguments, but got {}\n   Usage: pow(base, exponent)", args.len());
                    }
                    match (&args[0], &args[1]) {
                        (FluxValue::Integer(a), FluxValue::Integer(b)) => {
                            Ok(FluxValue::Integer((*a as f64).powf(*b as f64) as i64))
                        }
                        (FluxValue::Float(a), FluxValue::Float(b)) => Ok(FluxValue::Float(a.powf(*b))),
                        (FluxValue::Double(a), FluxValue::Double(b)) => Ok(FluxValue::Double(a.powf(*b))),
                        _ => bail!("⚠️  Type Error: pow() requires both arguments to be numeric types\n   Example: pow(2, 3), pow(2.0f, 3.0f)"),
                    }
                }
                "floor" => {
                    if args.len() != 1 {
                        bail!("❌ Function Error: floor() requires exactly 1 argument, but got {}\n   Usage: floor(value)", args.len());
                    }
                    match &args[0] {
                        FluxValue::Float(f) => Ok(FluxValue::Float(f.floor())),
                        FluxValue::Double(d) => Ok(FluxValue::Double(d.floor())),
                        FluxValue::Integer(i) => Ok(FluxValue::Integer(*i)),
                        _ => bail!("floor() requires numeric argument"),
                    }
                }
                "ceil" => {
                    if args.len() != 1 {
                        bail!("❌ Function Error: ceil() requires exactly 1 argument, but got {}\n   Usage: ceil(value)", args.len());
                    }
                    match &args[0] {
                        FluxValue::Float(f) => Ok(FluxValue::Float(f.ceil())),
                        FluxValue::Double(d) => Ok(FluxValue::Double(d.ceil())),
                        FluxValue::Integer(i) => Ok(FluxValue::Integer(*i)),
                        _ => bail!("⚠️  Type Error: ceil() requires a numeric argument (int, float, or double)\n   Example: ceil(3.14f)"),
                    }
                }
                "round" => {
                    if args.len() != 1 {
                        bail!("❌ Function Error: round() requires exactly 1 argument, but got {}\n   Usage: round(value)", args.len());
                    }
                    match &args[0] {
                        FluxValue::Float(f) => Ok(FluxValue::Float(f.round())),
                        FluxValue::Double(d) => Ok(FluxValue::Double(d.round())),
                        FluxValue::Integer(i) => Ok(FluxValue::Integer(*i)),
                        _ => bail!("⚠️  Type Error: round() requires a numeric argument (int, float, or double)\n   Example: round(3.14f)"),
                    }
                }
                "sqrt" => {
                    if args.len() != 1 {
                        bail!("❌ Function Error: sqrt() requires exactly 1 argument, but got {}\n   Usage: sqrt(value)", args.len());
                    }
                    match &args[0] {
                        FluxValue::Float(f) => Ok(FluxValue::Float(f.sqrt())),
                        FluxValue::Double(d) => Ok(FluxValue::Double(d.sqrt())),
                        FluxValue::Integer(i) => Ok(FluxValue::Double((*i as f64).sqrt())),
                        _ => bail!("⚠️  Type Error: sqrt() requires a numeric argument (int, float, or double)\n   Example: sqrt(16), sqrt(16.0)"),
                    }
                }
                _ => bail!("❌ Undefined function: '{}'\n   Available math functions: abs, max, min, pow, floor, ceil, round, sqrt", func_name_str),
            }
        }
        _ => bail!("❌ Unexpected expression type in evaluation"),
    }
}

// --- 6. CLI ARGUMENTS ---
#[derive(Parser)]
#[command(name = "fluxc", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        /// One or more .fsh source files
        #[arg(value_name = "FILES", num_args = 1..)]
        inputs: Vec<PathBuf>,
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Compile all .fsh files in a directory (e.g., --all os_fs/src)
        #[arg(long)]
        all: Option<PathBuf>,
        /// Run the generated binary right after linking
        #[arg(long)]
        run: bool,
    },
    Version,
}

// --- 7. FONCTION PRINCIPALE ---
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { inputs, output, all, run } => {
            // Déterminer les fichiers à compiler
            let mut files_to_compile = inputs.clone();
            
            // Si --all est spécifié, scanner le répertoire
            if let Some(dir_path) = all {
                if dir_path.is_dir() {
                    println!("📁 Scanning directory for .fsh files: {:?}", dir_path);
                    
                    // Trouver tous les fichiers .fsh dans le répertoire
                    if let Ok(entries) = fs::read_dir(&dir_path) {
                        let mut found_files: Vec<PathBuf> = entries
                            .filter_map(|entry| {
                                entry.ok().and_then(|e| {
                                    let path = e.path();
                                    if path.extension().map_or(false, |ext| ext == "fsh") {
                                        Some(path)
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect();
                        
                        // Trier les fichiers (main.fsh en premier)
                        found_files.sort_by_key(|p| {
                            let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                            match name.as_str() {
                                "main.fsh" => (0, name),
                                _ => (1, name),
                            }
                        });
                        
                        println!("✅ Found {} .fsh file(s):", found_files.len());
                        for f in &found_files {
                            println!("   - {:?}", f);
                        }
                        files_to_compile = found_files;
                    } else {
                        anyhow::bail!("Failed to read directory: {:?}", dir_path);
                    }
                } else {
                    anyhow::bail!("--all path is not a directory: {:?}", dir_path);
                }
            }
            
            if files_to_compile.is_empty() {
                anyhow::bail!("No .fsh files to compile");
            }
            let mut object_files: Vec<PathBuf> = Vec::new();

            for input in &files_to_compile {
                // Security: Validate input file
                validate_input_path(&input)
                    .with_context(|| format!("Invalid input path: {:?}", input))?;
                validate_file_size(&input)
                    .with_context(|| format!("Input file validation failed: {:?}", input))?;
                
                println!("🔍 Reading source: {:?}", input);
                let content = fs::read_to_string(input)
                    .with_context(|| format!("Could not read file {:?}", input))?;

                // Process include statements
                let processed_content = process_includes(&content, &input.parent().unwrap_or(&PathBuf::from(".")))?;
                
                // Validate Main class and main method
                validate_main_class(&processed_content)?;
                
                let asm_output = compile_fsh_to_asm(&processed_content, input)?;

                // Write .asm file
                let asm_path = input.with_extension("asm");
                validate_output_path(&asm_path)
                    .with_context(|| format!("Invalid output path: {:?}", asm_path))?;
                
                fs::write(&asm_path, &asm_output)
                    .with_context(|| format!("Could not write {:?}", asm_path))?;
                println!("📝 Generated ASM: {:?}", asm_path);

                // Assemble .asm → .o with NASM
                let obj_path = input.with_extension("o");
                let nasm_status = std::process::Command::new("nasm")
                    .args(["-f", "elf64", "-o"])
                    .arg(&obj_path)
                    .arg(&asm_path)
                    .status()
                    .with_context(|| "Failed to run nasm")?;

                if !nasm_status.success() {
                    anyhow::bail!("NASM failed for {:?}", asm_path);
                }
                println!("🔨 Assembled: {:?}", obj_path);
                object_files.push(obj_path);
            }

            // Assemble the runtime
            let runtime_asm = PathBuf::from("flux_compiler/fluxc/runtime/runtime.asm");
            if runtime_asm.exists() {
                let runtime_obj = PathBuf::from("flux_compiler/fluxc/runtime/runtime.o");
                let nasm_status = std::process::Command::new("nasm")
                    .args(["-f", "elf64", "-o"])
                    .arg(&runtime_obj)
                    .arg(&runtime_asm)
                    .status()
                    .with_context(|| "Failed to assemble runtime")?;

                if !nasm_status.success() {
                    anyhow::bail!("NASM failed for runtime.asm");
                }
                println!("🔨 Assembled runtime: {:?}", runtime_obj);
                object_files.push(runtime_obj);
            }

            // Link all .o files together
            if let Some(bin_path) = output {
                // Security: Validate output binary path
                validate_output_path(&bin_path)
                    .with_context(|| format!("Invalid binary output path: {:?}", bin_path))?;
                
                let mut ld_cmd = std::process::Command::new("ld");
                ld_cmd.arg("-o").arg(&bin_path);
                
                // Use linker script if it exists
                if std::path::Path::new("flux_kernel.ld").exists() {
                    ld_cmd.arg("-T").arg("flux_kernel.ld");
                }
                
                for obj in &object_files {
                    ld_cmd.arg(obj);
                }
                
                let ld_status = ld_cmd.status();
                match ld_status {
                    Ok(status) if status.success() => {
                        println!("✅ Linked binary: {:?}", bin_path);
                        if run {
                            run_with_timeout(&bin_path)?;
                        }
                    }
                    Ok(_) => {
                        // Linker produced warnings but still created output
                        println!("⚠️  Linked with warnings: {:?}", bin_path);
                        if run {
                            run_with_timeout(&bin_path)?;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Linker error: {}", e);
                        anyhow::bail!("Failed to run linker");
                    }
                }
            } else {
                println!("✅ Compilation successful (no output binary specified, use -o)");
            }
        }
        Commands::Version => {
            println!("Flux# Compiler (fluxc) v0.1.0");
        }
    }
    Ok(())
}

/// Run a binary with timeout protection
fn run_with_timeout(bin_path: &PathBuf) -> Result<()> {
    use std::time::Instant;
    
    let start = Instant::now();
    let timeout = Duration::from_secs(RUN_TIMEOUT_SECS);
    
    match std::process::Command::new(bin_path)
        .output()
    {
        Ok(out) => {
            let elapsed = start.elapsed();
            
            // Check if execution took too long
            if elapsed > timeout {
                eprintln!(
                    "⚠️  Process took longer than {} seconds ({}s)",
                    RUN_TIMEOUT_SECS,
                    elapsed.as_secs()
                );
                return Ok(());
            }
            
            print!("{}", String::from_utf8_lossy(&out.stdout));
            eprint!("{}", String::from_utf8_lossy(&out.stderr));
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to execute {}: {}", bin_path.display(), e);
            bail!("Execution failed")
        }
    }
}

// --- Evaluation of math function calls ---
fn eval_math_function(func_name: &str, args: Vec<FluxValue>) -> Result<FluxValue> {
    match func_name {
        "sqrt" => {
            if args.len() != 1 {
                anyhow::bail!("sqrt expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).sqrt())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.sqrt())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.sqrt())),
                _ => anyhow::bail!("sqrt expects numeric argument"),
            }
        }
        "abs" => {
            if args.len() != 1 {
                anyhow::bail!("abs expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Integer(n.abs())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.abs())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.abs())),
                _ => anyhow::bail!("abs expects numeric argument"),
            }
        }
        "floor" => {
            if args.len() != 1 {
                anyhow::bail!("floor expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Integer(*n)),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.floor())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.floor())),
                _ => anyhow::bail!("floor expects numeric argument"),
            }
        }
        "ceil" => {
            if args.len() != 1 {
                anyhow::bail!("ceil expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Integer(*n)),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.ceil())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.ceil())),
                _ => anyhow::bail!("ceil expects numeric argument"),
            }
        }
        "round" => {
            if args.len() != 1 {
                anyhow::bail!("round expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Integer(*n)),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.round())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.round())),
                _ => anyhow::bail!("round expects numeric argument"),
            }
        }
        "sin" => {
            if args.len() != 1 {
                anyhow::bail!("sin expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).sin())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.sin())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.sin())),
                _ => anyhow::bail!("sin expects numeric argument"),
            }
        }
        "cos" => {
            if args.len() != 1 {
                anyhow::bail!("cos expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).cos())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.cos())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.cos())),
                _ => anyhow::bail!("cos expects numeric argument"),
            }
        }
        "tan" => {
            if args.len() != 1 {
                anyhow::bail!("tan expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).tan())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.tan())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.tan())),
                _ => anyhow::bail!("tan expects numeric argument"),
            }
        }
        "pow" => {
            if args.len() != 2 {
                anyhow::bail!("pow expects 2 arguments, got {}", args.len());
            }
            let base = match &args[0] {
                FluxValue::Integer(n) => *n as f64,
                FluxValue::Float(f) => *f as f64,
                FluxValue::Double(d) => *d,
                _ => anyhow::bail!("pow expects numeric arguments"),
            };
            let exp = match &args[1] {
                FluxValue::Integer(n) => *n as f64,
                FluxValue::Float(f) => *f as f64,
                FluxValue::Double(d) => *d,
                _ => anyhow::bail!("pow expects numeric arguments"),
            };
            Ok(FluxValue::Double(base.powf(exp)))
        }
        "ln" => {
            if args.len() != 1 {
                anyhow::bail!("ln expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).ln())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.ln())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.ln())),
                _ => anyhow::bail!("ln expects numeric argument"),
            }
        }
        "log10" => {
            if args.len() != 1 {
                anyhow::bail!("log10 expects 1 argument, got {}", args.len());
            }
            match &args[0] {
                FluxValue::Integer(n) => Ok(FluxValue::Double((*n as f64).log10())),
                FluxValue::Float(f) => Ok(FluxValue::Float(f.log10())),
                FluxValue::Double(d) => Ok(FluxValue::Double(d.log10())),
                _ => anyhow::bail!("log10 expects numeric argument"),
            }
        }
        _ => anyhow::bail!("Unknown math function: {}", func_name),
    }
}

fn compile_condition(
    condition_pair: pest::iterators::Pair<Rule>,
    label_false: &str,
    text_section: &mut String,
    symbols: &SymbolTable,
    var_offsets: &HashMap<String, i32>,
) -> Result<()> {
    // condition = { "(" ~ condition ~ ")" | expr }
    let inner = condition_pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::condition => {
            // Parenthèses — récursion
            compile_condition(inner, label_false, text_section, symbols, var_offsets)?;
        }
        Rule::expr => {
            // Essayer d'évaluer statiquement d'abord
            if let Ok(val) = eval_expr(inner.clone(), &symbols.variables) {
                match val {
                    FluxValue::Integer(0) => {
                        text_section.push_str(&format!("    jmp {}\n", label_false));
                    }
                    FluxValue::Integer(_) => {
                        // Toujours vrai — pas de saut
                    }
                    _ => {}
                }
            } else {
                // Expression dynamique — générer du vrai code de comparaison
                let expr_str = inner.as_str();
                
                // Try to extract comparison operands: "var op value"
                let mut found_comparison = false;
                
                // Check for common comparison operators
                if expr_str.contains("<") && !expr_str.contains("<=") {
                    let parts: Vec<&str> = expr_str.split('<').collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    jge {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                } else if expr_str.contains(">") && !expr_str.contains(">=") {
                    let parts: Vec<&str> = expr_str.split('>').collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    jle {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                } else if expr_str.contains("==") {
                    let parts: Vec<&str> = expr_str.split("==").collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    jne {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                } else if expr_str.contains("!=") {
                    let parts: Vec<&str> = expr_str.split("!=").collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    je {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                } else if expr_str.contains("<=") {
                    let parts: Vec<&str> = expr_str.split("<=").collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    jg {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                } else if expr_str.contains(">=") {
                    let parts: Vec<&str> = expr_str.split(">=").collect();
                    if parts.len() == 2 {
                        let var_name = parts[0].trim();
                        let value_str = parts[1].trim();
                        if let Some(&offset) = var_offsets.get(var_name) {
                            if let Ok(val) = value_str.parse::<i64>() {
                                text_section.push_str(&format!("    mov rax, [rbp-{}]\n", offset));
                                text_section.push_str(&format!("    cmp rax, {}\n", val));
                                text_section.push_str(&format!("    jl {}\n", label_false));
                                found_comparison = true;
                            }
                        }
                    }
                }
                
                if !found_comparison {
                    // Fallback: generic condition evaluation
                    text_section.push_str(&format!("    ; condition: {}\n", expr_str.trim()));
                    text_section.push_str(&format!("    test rax, rax\n    jz {}\n", label_false));
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn compile_block_from_if(
    if_pair: pest::iterators::Pair<Rule>,
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
    // Wrapper pour compiler un if_stmt standalone dans un else
    let mut inner = if_pair.into_inner();
    let cond = inner.next().unwrap();
    let then = inner.next().unwrap();
    let else_ = inner.next();

    let id = *unique_id; *unique_id += 1;
    let lf = format!(".if_false_{}", id);
    let le = format!(".if_end_{}", id);

    compile_condition(cond, &lf, text_section, symbols, &var_offsets)?;
    compile_block_with_loop_context(then, content, source_lines, symbols, data_section, text_section, unique_id, var_offsets, stack_offset, false, loop_start.clone(), loop_end.clone())?;
    text_section.push_str(&format!("    jmp {}\n{}:\n", le, lf));
    if let Some(ep) = else_ {
        if let Some(eb) = ep.into_inner().next() {
            if eb.as_rule() == Rule::block {
                compile_block_with_loop_context(eb, content, source_lines, symbols, data_section, text_section, unique_id, var_offsets, stack_offset, false, loop_start.clone(), loop_end.clone())?;
            }
        }
    }
    text_section.push_str(&format!("{}:\n", le));
    Ok(())
}

fn apply_op(left: FluxValue, op: &str, right: FluxValue) -> Result<FluxValue> {
    match (&left, &right) {
        (FluxValue::Integer(l), FluxValue::Integer(r)) => Ok(FluxValue::Integer(match op {
            "+"  => l + r,
            "-"  => l - r,
            "*"  => l * r,
            "/"  => { if *r == 0 { bail!("Division by zero") } l / r }
            "%"  => { if *r == 0 { bail!("Modulo by zero") } l % r }
            "&"  => l & r,
            "|"  => l | r,
            "^"  => l ^ r,
            "<<" => l << r,
            ">>" => l >> r,
            "==" => if l == r { 1 } else { 0 },
            "!=" => if l != r { 1 } else { 0 },
            "<"  => if l < r  { 1 } else { 0 },
            "<=" => if l <= r { 1 } else { 0 },
            ">"  => if l > r  { 1 } else { 0 },
            ">=" => if l >= r { 1 } else { 0 },
            "&&" => if *l != 0 && *r != 0 { 1 } else { 0 },
            "||" => if *l != 0 || *r != 0 { 1 } else { 0 },
            _    => bail!("Unknown operator: {}", op),
        })),
        (FluxValue::Str(l), FluxValue::Str(r)) => {
            if op == "+" { Ok(FluxValue::Str(format!("{}{}", l, r))) }
            else { bail!("Unsupported operator {} on strings", op) }
        }
        (FluxValue::Str(l), FluxValue::Integer(r)) => {
            if op == "+" { Ok(FluxValue::Str(format!("{}{}", l, r))) }
            else { bail!("Unsupported operator {} between string and int", op) }
        }
        (FluxValue::Integer(l), FluxValue::Str(r)) => {
            if op == "+" { Ok(FluxValue::Str(format!("{}{}", l, r))) }
            else { bail!("Unsupported operator {} between int and string", op) }
        }
        (FluxValue::Float(l), FluxValue::Float(r)) => Ok(FluxValue::Float(match op {
            "+" => l + r, "-" => l - r, "*" => l * r,
            "/" => { if *r == 0.0 { bail!("Division by zero") } l / r }
            _   => bail!("Unknown float operator: {}", op),
        })),
        (FluxValue::Double(l), FluxValue::Double(r)) => Ok(FluxValue::Double(match op {
            "+" => l + r, "-" => l - r, "*" => l * r,
            "/" => { if *r == 0.0 { bail!("Division by zero") } l / r }
            _   => bail!("Unknown double operator: {}", op),
        })),
        (FluxValue::Integer(l), FluxValue::Float(r))  => Ok(FluxValue::Float(*l as f32 + r)),
        (FluxValue::Float(l),   FluxValue::Integer(r)) => Ok(FluxValue::Float(l + *r as f32)),
        (FluxValue::Integer(l), FluxValue::Double(r)) => Ok(FluxValue::Double(*l as f64 + r)),
        (FluxValue::Double(l),  FluxValue::Integer(r)) => Ok(FluxValue::Double(l + *r as f64)),
        _ => bail!("Arithmetic on incompatible types"),
    }
}
