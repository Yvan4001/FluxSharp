use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use std::collections::HashMap;
use pest::Parser as PestParser;
use pest_derive::Parser as PestDeriveParser;

// --- 1. CONFIGURATION DU PARSEUR PEST ---
#[derive(PestDeriveParser)]
#[grammar = "flux_grammar.pest"]
pub struct FluxParser;

/// Compile a single .fsh source into NASM x86_64 assembly string
fn compile_fsh_to_asm(content: &str, source_path: &PathBuf) -> Result<String> {
    let source_lines: Vec<&str> = content.lines().collect();

    let file = FluxParser::parse(Rule::file, content)
        .with_context(|| "Syntax Error (check your .fsh file)")?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty file"))?;

    let mut symbols = SymbolTable {
        variables: HashMap::new(),
        structs: HashMap::new(),
        functions: HashMap::new(),
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
                        text_section.push_str(&format!("global {}\n{}:\n", method_name, method_name));
                        text_section.push_str("    push rbp\n    mov rbp, rsp\n");
                        compile_block(
                            block,
                            content,
                            &source_lines,
                            &mut symbols,
                            &mut data_section,
                            &mut text_section,
                            &mut unique_id,
                        )?;
                    }
                }
            }

            Rule::function => {
                let source_span = pair.as_span();
                let line_start = content[..source_span.start()].lines().count();

                let mut inner = pair.into_inner();
                
                // Collecte les idents (premier = type retour ou nom, second = nom fonction)
                let idents: Vec<_> = inner.clone().filter(|p| p.as_rule() == Rule::ident).collect();
                
                // Détermine le type de retour et le nom de la fonction
                let (ret_type_str, func_name) = if idents.len() >= 2 {
                    // Cas normal : type retour + nom fonction
                    (idents[0].as_str(), idents[1].as_str())
                } else if idents.len() == 1 {
                    // Cas spécial : pas de type retour explicite, utiliser "void"
                    ("void", idents[0].as_str())
                } else {
                    eprintln!("ERROR: No function name found in function definition");
                    ("void", "unknown")
                };

                let mut params = Vec::new();
                if let Some(param_list) = inner.clone().find(|p| p.as_rule() == Rule::param_list) {
                    for p in param_list.into_inner() {
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

                symbols.functions.insert(func_name.to_string(), FunctionSignature {
                    return_type: FluxType::from_str(ret_type_str),
                    parameters: params.clone(),
                });

                if let Some(block) = inner.find(|p| p.as_rule() == Rule::block) {
                    // --- C. Commentaire source pour la signature ---
                    if let Some(line) = source_lines.get(line_start.saturating_sub(1)) {
                        text_section.push_str(&format!("\n; --- {} ---\n", line.trim()));
                    }

                    text_section.push_str(&format!("global {}\n{}:\n", func_name, func_name));
                    text_section.push_str("    push rbp\n    mov rbp, rsp\n");

                    compile_block(
                        block,
                        content,
                        &source_lines,
                        &mut symbols,
                        &mut data_section,
                        &mut text_section,
                        &mut unique_id,
                    )?;
                }
            }
            _ => {}
        }
    }

    // Ensure there is a main symbol for runtime entrypoint
    if !symbols.functions.contains_key("main") {
        if symbols.functions.contains_key("Main") {
            text_section.push_str("global main\nmain:\n    call Main\n    ret\n\n");
        } else {
            text_section.push_str("global main\nmain:\n    mov rax, 0\n    ret\n\n");
        }
    }

    // Assemble the full output
    let mut asm = String::new();
    asm.push_str(&format!("; ============================\n"));
    asm.push_str(&format!("; Flux# compiled from {:?}\n", source_path));
    asm.push_str(&format!("; fluxc v0.1.0\n"));
    asm.push_str(&format!("; ============================\n\n"));

    if !data_section.is_empty() {
        asm.push_str("section .data\n");
        asm.push_str(&data_section);
        asm.push_str("\n");
    }

    asm.push_str("section .text\n");
    asm.push_str(&text_section);

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
) -> Result<()> {
    let mut stack_offset: i32 = 0;
    let mut var_offsets: HashMap<String, i32> = HashMap::new();

    for statement in block.into_inner() {
        let stmt_span = statement.as_span();
        let stmt_line = content[..stmt_span.start()].lines().count();
        if let Some(src_line) = source_lines.get(stmt_line.saturating_sub(1)) {
            text_section.push_str(&format!("\n    ; --- {} ---\n", src_line.trim()));
        }

        match statement.as_rule() {
            Rule::variable_decl => {
                let mut decl_inner = statement.into_inner();
                let _var_type = decl_inner.next().unwrap().as_str();
                let var_name = decl_inner.next().unwrap().as_str().to_string();

                stack_offset += 8;
                var_offsets.insert(var_name.clone(), stack_offset);
                text_section.push_str(&format!("    sub rsp, 8\n"));

                if let Some(expr_pair) = decl_inner.find(|p| p.as_rule() == Rule::expr) {
                    match eval_expr(expr_pair, &symbols.variables) {
                        Ok(val) => {
                            match &val {
                                FluxValue::Integer(n) => {
                                    text_section.push_str(&format!(
                                        "    mov qword [rbp-{}], {}\n",
                                        stack_offset, n
                                    ));
                                }
                                FluxValue::Float(f) => {
                                    // Store float value in data section
                                    let label = format!("float_{}", *unique_id);
                                    *unique_id += 1;
                                    let float_bits = f.to_bits();
                                    data_section.push_str(&format!("{}: dd 0x{:x}\n", label, float_bits));
                                    text_section.push_str(&format!(
                                        "    mov eax, [rel {}]\n    mov dword [rbp-{}], eax\n",
                                        label, stack_offset
                                    ));
                                }
                                FluxValue::Double(d) => {
                                    // Store double value in data section
                                    let label = format!("double_{}", *unique_id);
                                    *unique_id += 1;
                                    let double_bits = d.to_bits();
                                    data_section.push_str(&format!("{}: dq 0x{:x}\n", label, double_bits));
                                    text_section.push_str(&format!(
                                        "    mov rax, [rel {}]\n    mov qword [rbp-{}], rax\n",
                                        label, stack_offset
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
                                        label, stack_offset
                                    ));
                                }
                            }
                            symbols.variables.insert(var_name, val);
                        }
                        Err(e) => {
                            text_section.push_str(&format!(
                                "    ; ERROR evaluating expr: {}\n", e
                            ));
                        }
                    }
                }
            }

            Rule::function_call => {
                let mut call_inner = statement.into_inner();
                let callee = call_inner.next().unwrap().as_str();

                if callee == "serial_print" || callee == "print" {
                    if let Some(arg_pair) = call_inner.next() {
                        match eval_expr(arg_pair, &symbols.variables) {
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
                            Err(e) => {
                                text_section.push_str(&format!("    ; ERROR serial_print arg eval: {}\n", e));
                            }
                        }
                    } else {
                        text_section.push_str("    ; WARNING serial_print called without argument\n");
                    }
                }
            }

            Rule::assignment_stmt => {
                let mut assign_inner = statement.into_inner();
                let var_name = assign_inner.next().unwrap().as_str().to_string();
                let _assign_op = assign_inner.next().unwrap().as_str();
                let expr_pair = assign_inner.next().unwrap();

                if let Ok(val) = eval_expr(expr_pair, &symbols.variables) {
                    if let Some(&offset) = var_offsets.get(&var_name) {
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

            _ => {}
        }
    }

    text_section.push_str("\n    mov rsp, rbp\n    pop rbp\n    ret\n\n");
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
}

// --- 5. EXPRESSION EVALUATOR ---
fn eval_expr(pair: pest::iterators::Pair<Rule>, vars: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    let mut inner = pair.into_inner();
    let first = inner.next().ok_or_else(|| anyhow::anyhow!("Empty expression"))?;
    let mut result = eval_atom(first, vars)?;

    while let Some(op_pair) = inner.next() {
        let op = op_pair.as_str();
        let next_atom = inner.next().ok_or_else(|| anyhow::anyhow!("Missing operand after '{}'", op))?;
        let right = eval_atom(next_atom, vars)?;

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
                        if *r == 0.0 { anyhow::bail!("Modulo by zero"); }
                        l % (*r as f64)
                    },
                    _ => anyhow::bail!("Unknown operator: {}", op),
                })
            }
            (FluxValue::Str(l), FluxValue::Str(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("Unsupported operator {} on strings", op)
                }
            }
            (FluxValue::Str(l), FluxValue::Integer(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("Unsupported operator {} between string and integer", op)
                }
            }
            (FluxValue::Integer(l), FluxValue::Str(r)) => {
                if op == "+" {
                    FluxValue::Str(format!("{}{}", l, r))
                } else {
                    anyhow::bail!("Unsupported operator {} between integer and string", op)
                }
            }
            _ => anyhow::bail!("Arithmetic on non-numeric values"),
        };
    }

    Ok(result)
}

fn eval_atom(pair: pest::iterators::Pair<Rule>, vars: &HashMap<String, FluxValue>) -> Result<FluxValue> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::int_literal => {
            let n: i64 = inner.as_str().parse()
                .with_context(|| format!("Invalid integer: {}", inner.as_str()))?;
            Ok(FluxValue::Integer(n))
        }
        Rule::float_literal => {
            let raw = inner.as_str();
            // Remove 'f' or 'F' suffix
            let num_str = &raw[..raw.len() - 1];
            let n: f32 = num_str.parse()
                .with_context(|| format!("Invalid float: {}", raw))?;
            Ok(FluxValue::Float(n))
        }
        Rule::double_literal => {
            let raw = inner.as_str();
            let n: f64 = raw.parse()
                .with_context(|| format!("Invalid double: {}", raw))?;
            Ok(FluxValue::Double(n))
        }
        Rule::string_literal => {
            let raw = inner.as_str();
            // Remove surrounding quotes
            let content = &raw[1..raw.len() - 1];
            Ok(FluxValue::Str(content.to_string()))
        }
        Rule::char_literal => {
            let raw = inner.as_str();
            // Remove surrounding single quotes — treat as variable name reference
            let content = &raw[1..raw.len() - 1];
            // If it's a known variable, return its value
            if let Some(val) = vars.get(content) {
                Ok(val.clone())
            } else {
                // Otherwise treat as a character/string
                Ok(FluxValue::Str(content.to_string()))
            }
        }
        Rule::ident => {
            let name = inner.as_str();
            
            // Check for predefined math constants
            match name {
                "PI" => return Ok(FluxValue::Double(std::f64::consts::PI)),
                "E" => return Ok(FluxValue::Double(std::f64::consts::E)),
                "LN2" => return Ok(FluxValue::Double(std::f64::consts::LN_2)),
                "LN10" => return Ok(FluxValue::Double(std::f64::consts::LN_10)),
                "SQRT2" => return Ok(FluxValue::Double(std::f64::consts::SQRT_2)),
                _ => {}
            }
            
            vars.get(name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Undefined variable: '{}'", name))
        }
        _ => anyhow::bail!("Unexpected atom: {:?}", inner.as_rule()),
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
                println!("🔍 Reading source: {:?}", input);
                let content = fs::read_to_string(input)
                    .with_context(|| format!("Could not read file {:?}", input))?;

                let asm_output = compile_fsh_to_asm(&content, input)?;

                // Write .asm file
                let asm_path = input.with_extension("asm");
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
                            let output_run = std::process::Command::new(&bin_path)
                                .output();
                            match output_run {
                                Ok(out) => {
                                    print!("{}", String::from_utf8_lossy(&out.stdout));
                                    eprint!("{}", String::from_utf8_lossy(&out.stderr));
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to execute {}: {}", bin_path.display(), e);
                                }
                            }
                        }
                    }
                    Ok(_) => {
                        // Linker produced warnings but still created output
                        println!("⚠️  Linked with warnings: {:?}", bin_path);
                        if run {
                            let output_run = std::process::Command::new(&bin_path)
                                .output();
                            match output_run {
                                Ok(out) => {
                                    print!("{}", String::from_utf8_lossy(&out.stdout));
                                    eprint!("{}", String::from_utf8_lossy(&out.stderr));
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to execute {}: {}", bin_path.display(), e);
                                }
                            }
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


