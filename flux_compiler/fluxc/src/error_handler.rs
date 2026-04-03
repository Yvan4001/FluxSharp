/// Error handling module for FluxSharp compiler
/// Provides detailed, user-friendly error messages

use std::fmt;

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

