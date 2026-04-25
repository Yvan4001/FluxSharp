/// Error handling module for FluxSharp compiler
/// Provides detailed, user-friendly error messages

use std::fmt;

/// Maps generated ASM line numbers back to the original FluxSharp source positions.
///
/// The compiler embeds `; FluxSharp:{line} | {source_text}` comments into the
/// generated assembly at every statement boundary. When NASM fails, this struct
/// scans backward from the failing ASM line to find the nearest such comment,
/// allowing precise source-level error reporting.
pub struct SourceMap;

impl SourceMap {
    /// Returns a formatted debug comment to embed in generated ASM.
    /// `source_line` is 1-based. `source_text` is the trimmed source snippet.
    pub fn debug_comment(source_line: usize, source_text: &str) -> String {
        format!("    ; FluxSharp:{} | {}", source_line, source_text.trim())
    }

    /// Parses a NASM error message and maps the failing ASM line back to the
    /// original FluxSharp source line using the embedded `; FluxSharp:N` markers.
    ///
    /// Returns a user-friendly error string with the mapped source location.
    pub fn map_nasm_error(asm_content: &str, nasm_stderr: &str) -> String {
        let mut result = format!("\nNASM Error:\n{}\n", nasm_stderr.trim());

        // NASM error format: "path/to/file.asm:LINE: error: ..."
        let asm_error_line = nasm_stderr.lines().find_map(|line| {
            let colon = line.find(':')?;
            let after_first = &line[colon + 1..];
            let second_colon = after_first.find(':')?;
            after_first[..second_colon].trim().parse::<usize>().ok()
        });

        if let Some(error_line) = asm_error_line {
            let asm_lines: Vec<&str> = asm_content.lines().collect();
            let scan_until = error_line.min(asm_lines.len());

            // Scan backward from the error line to find the nearest FluxSharp marker.
            for asm_line in asm_lines[..scan_until].iter().rev() {
                if let Some(rest) = asm_line.trim().strip_prefix("; FluxSharp:") {
                    let mut parts = rest.splitn(2, '|');
                    let src_line_num = parts.next().and_then(|s| s.trim().parse::<usize>().ok());
                    let src_text = parts.next().map(|s| s.trim()).unwrap_or("");

                    if let Some(line_num) = src_line_num {
                        result.push_str(&format!(
                            "\n  Originated from FluxSharp source line {}",
                            line_num
                        ));
                        if !src_text.is_empty() {
                            result.push_str(&format!(":\n    > {}", src_text));
                        }
                        result.push('\n');
                    }
                    break;
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CompileError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub source_line: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ErrorType {
    SyntaxError,
    TypeError,
    UndefinedVariable,
    UndefinedFunction,
    InvalidConversion,
    MissingCharacter,
    MissingArrayIndex,
    InvalidFunctionCall,
    ClassNotFound,
    DivisionByZero,
    Other,
}

impl CompileError {
    #[allow(dead_code)]
    pub fn new(
        line: usize,
        column: usize,
        message: String,
        source_line: String,
        error_type: ErrorType,
    ) -> Self {
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type,
        }
    }

    #[allow(dead_code)]
    pub fn missing_semicolon(line: usize, source_line: String) -> Self {
        CompileError {
            line,
            column: source_line.len(),
            message: "Missing semicolon at end of statement".to_string(),
            source_line,
            error_type: ErrorType::MissingCharacter,
        }
    }

    #[allow(dead_code)]
    pub fn type_mismatch(
        line: usize,
        column: usize,
        from_type: &str,
        to_type: &str,
        source_line: String,
    ) -> Self {
        let message = format!(
            "Type mismatch: cannot convert '{}' to '{}'. Consider using explicit casting.",
            from_type, to_type
        );
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type: ErrorType::InvalidConversion,
        }
    }

    #[allow(dead_code)]
    pub fn undefined_variable(
        line: usize,
        column: usize,
        var_name: &str,
        source_line: String,
    ) -> Self {
        let message = format!(
            "Undefined variable: '{}'. Make sure it's declared before use.",
            var_name
        );
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type: ErrorType::UndefinedVariable,
        }
    }

    #[allow(dead_code)]
    pub fn undefined_function(
        line: usize,
        column: usize,
        func_name: &str,
        source_line: String,
    ) -> Self {
        let message = format!(
            "Undefined function: '{}'. Make sure it's declared before use.",
            func_name
        );
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type: ErrorType::UndefinedFunction,
        }
    }

    pub fn float_literal_syntax(
        line: usize,
        column: usize,
        example: &str,
        source_line: String,
    ) -> Self {
        let message = format!(
            "Invalid float literal syntax. Use format like: {}",
            example
        );
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type: ErrorType::SyntaxError,
        }
    }

    pub fn invalid_array_access(
        line: usize,
        column: usize,
        array_name: &str,
        source_line: String,
    ) -> Self {
        let message = format!(
            "Invalid array access for '{}'. Use format: {}[index]",
            array_name, array_name
        );
        CompileError {
            line,
            column,
            message,
            source_line,
            error_type: ErrorType::MissingArrayIndex,
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_prefix = match self.error_type {
            ErrorType::SyntaxError => "❌ SYNTAX ERROR",
            ErrorType::TypeError => "⚠️  TYPE ERROR",
            ErrorType::UndefinedVariable => "❌ UNDEFINED VARIABLE",
            ErrorType::UndefinedFunction => "❌ UNDEFINED FUNCTION",
            ErrorType::InvalidConversion => "⚠️  INVALID CONVERSION",
            ErrorType::MissingCharacter => "❌ MISSING CHARACTER",
            ErrorType::MissingArrayIndex => "❌ ARRAY ACCESS ERROR",
            ErrorType::InvalidFunctionCall => "❌ INVALID FUNCTION CALL",
            ErrorType::ClassNotFound => "❌ CLASS NOT FOUND",
            ErrorType::DivisionByZero => "⚠️  DIVISION BY ZERO",
            ErrorType::Other => "❌ COMPILATION ERROR",
        };

        write!(
            f,
            "{} at line {}, column {}:\n  {}\n\n  > {}\n  {}\n",
            error_prefix,
            self.line,
            self.column,
            self.message,
            self.source_line,
            " ".repeat(self.column.saturating_sub(1)) + "^"
        )
    }
}

