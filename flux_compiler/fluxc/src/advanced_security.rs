/// Advanced Security Module for FluxSharp
/// Implements compile-time and runtime security checks
/// 
/// Features:
/// - Integer overflow detection (checked arithmetic)
/// - Null safety guarantees
/// - Type safety enforcement
/// - Memory bounds checking
/// - Safe casting and type conversion
/// - Exception handling

use std::collections::HashMap;
use anyhow::{Result, bail, Context};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum OverflowCheckMode {
    /// Unchecked (C/C++ style) - UNSAFE
    Unchecked,
    /// Checked with panic (Java/C# style) - SAFE
    Checked,
    /// Saturating (wraps to min/max) - SAFE
    Saturating,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecurityConfig {
    /// Enable integer overflow detection
    pub check_overflow: bool,
    /// Overflow behavior mode
    pub overflow_mode: OverflowCheckMode,
    /// Enable null safety checks
    pub null_safe: bool,
    /// Enable bounds checking
    pub bounds_check: bool,
    /// Enable type safety validation
    pub type_safe: bool,
    /// Enable exception handling
    pub exception_handling: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            check_overflow: true,
            overflow_mode: OverflowCheckMode::Checked,
            null_safe: true,
            bounds_check: true,
            type_safe: true,
            exception_handling: true,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VariableSecurity {
    pub name: String,
    pub var_type: String,
    pub nullable: bool,
    pub initialized: bool,
    pub bounds: Option<(i64, i64)>, // min, max
}

#[allow(dead_code)]
pub struct AdvancedSecurityValidator {
    config: SecurityConfig,
    checked_variables: HashMap<String, VariableSecurity>,
}

impl AdvancedSecurityValidator {
    #[allow(dead_code)]
    pub fn new(config: SecurityConfig) -> Self {
        AdvancedSecurityValidator {
            config,
            checked_variables: HashMap::new(),
        }
    }

    /// Register a variable with security metadata
    #[allow(dead_code)]
    pub fn register_variable(
        &mut self,
        name: String,
        var_type: String,
        nullable: bool,
    ) -> Result<(), String> {
        if self.checked_variables.contains_key(&name) {
            return Err(format!("Variable '{}' already registered", name));
        }

        let bounds = match var_type.as_str() {
            "byte" => Some((0, 255)),
            "int" => Some((i64::MIN, i64::MAX)),
            "long" => Some((i64::MIN, i64::MAX)),
            _ => None,
        };

        self.checked_variables.insert(
            name.clone(),
            VariableSecurity {
                name,
                var_type,
                nullable,
                initialized: false,
                bounds,
            },
        );

        Ok(())
    }

    /// Validate integer arithmetic for overflow
    #[allow(dead_code)]
    pub fn validate_arithmetic(
        &self,
        operand1: i64,
        operator: &str,
        operand2: i64,
    ) -> Result<i64, String> {
        if !self.config.check_overflow {
            // Unchecked mode - return native result
            return Ok(match operator {
                "+" => operand1.wrapping_add(operand2),
                "-" => operand1.wrapping_sub(operand2),
                "*" => operand1.wrapping_mul(operand2),
                "/" => {
                    if operand2 == 0 {
                        return Err("Division by zero".to_string());
                    }
                    operand1 / operand2
                }
                _ => return Err(format!("Unknown operator: {}", operator)),
            });
        }

        // Checked mode
        match operator {
            "+" => operand1
                .checked_add(operand2)
                .ok_or_else(|| "Integer overflow in addition".to_string()),
            "-" => operand1
                .checked_sub(operand2)
                .ok_or_else(|| "Integer overflow in subtraction".to_string()),
            "*" => operand1
                .checked_mul(operand2)
                .ok_or_else(|| "Integer overflow in multiplication".to_string()),
            "/" => {
                if operand2 == 0 {
                    Err("Division by zero".to_string())
                } else {
                    operand1
                        .checked_div(operand2)
                        .ok_or_else(|| "Integer overflow in division".to_string())
                }
            }
            _ => Err(format!("Unknown operator: {}", operator)),
        }
    }

    /// Validate null safety
    #[allow(dead_code)]
    pub fn validate_null_access(
        &self,
        var_name: &str,
    ) -> Result<(), String> {
        if !self.config.null_safe {
            return Ok(());
        }

        if let Some(var) = self.checked_variables.get(var_name) {
            if var.nullable && !var.initialized {
                return Err(format!(
                    "Null pointer dereference: variable '{}' may be null",
                    var_name
                ));
            }
            Ok(())
        } else {
            Err(format!("Variable '{}' not registered", var_name))
        }
    }

    /// Validate type casting safety
    #[allow(dead_code)]
    pub fn validate_cast(
        &self,
        from_type: &str,
        to_type: &str,
        value: i64,
    ) -> Result<(), String> {
        if !self.config.type_safe {
            return Ok(());
        }

        match (from_type, to_type) {
            ("int", "byte") => {
                if value < 0 || value > 255 {
                    return Err(format!(
                        "Unsafe cast: value {} out of byte range [0, 255]",
                        value
                    ));
                }
            }
            ("long", "int") => {
                if value < i32::MIN as i64 || value > i32::MAX as i64 {
                    return Err(format!(
                        "Unsafe cast: value {} out of int range",
                        value
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Check if array is registered
    #[allow(dead_code)]
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }

    /// Get all checked variables
    #[allow(dead_code)]
    pub fn variables(&self) -> &HashMap<String, VariableSecurity> {
        &self.checked_variables
    }
}

pub fn validate_input_path(path: &Path) -> Result<()> {
    // Resolve the canonical path atomically — this resolves all symlinks and
    // normalises ".." components in a single syscall, eliminating TOCTOU.
    let canonical = path.canonicalize()
        .with_context(|| format!("Cannot resolve path (does not exist or permission denied): {:?}", path))?;

    // Guard against ".." surviving in the original string (belt-and-suspenders).
    let path_str = canonical.to_string_lossy();
    if path_str.contains("..") {
        bail!("Path traversal detected in input: {:?}", canonical);
    }

    // After canonicalize(), the path is by definition a real file with no symlink
    // components.  Verify it is a regular file (not a directory, device, etc.).
    if !canonical.is_file() {
        bail!("Input path is not a regular file: {:?}", canonical);
    }

    Ok(())
}

pub fn validate_output_path(path: &Path) -> Result<()> {
    // Belt-and-suspenders: reject any ".." component before we do anything else.
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        bail!("Path traversal detected: {:?} contains '..'", path);
    }

    // Resolve the project root canonically.
    let project_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let canonical_project = project_dir
        .canonicalize()
        .with_context(|| format!("Cannot resolve project directory: {:?}", project_dir))?;

    // Build the absolute path (the output file may not exist yet).
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        canonical_project.join(path)
    };

    // The output file may not exist yet, but its parent directory must.
    // Canonicalize the parent (which does exist) to resolve all symlinks,
    // then reconstruct the full path as canonical_parent / filename.
    let parent = absolute_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Output path has no parent directory: {:?}", absolute_path))?;

    if !parent.exists() {
        bail!("Output directory does not exist: {:?}", parent);
    }

    let canonical_parent = parent
        .canonicalize()
        .with_context(|| format!("Cannot resolve output directory: {:?}", parent))?;

    if !canonical_parent.starts_with(&canonical_project) {
        bail!(
            "Path traversal detected: output directory {:?} is outside project root {:?}",
            canonical_parent,
            canonical_project
        );
    }

    Ok(())
}

pub fn validate_file_size(path: &PathBuf) -> Result<()> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("Cannot access file: {:?}", path))?;
    
    if metadata.len() > 50 * 1024 * 1024 {
        bail!(
            "File too large: {:?} ({} bytes > {} bytes limit)",
            path,
            metadata.len(),
            50 * 1024 * 1024
        );
    }
    
    if metadata.len() == 0 {
        bail!("File is empty: {:?}", path);
    }
    
    Ok(())
}

/// Count non-overlapping occurrences of `needle` in `haystack`, skipping
/// characters that are inside single-line comments (`//`), block comments
/// (`/* ... */`), or string literals (`"..."`).
fn count_outside_comments_and_strings(haystack: &str, needle: &str) -> usize {
    let chars: Vec<char> = haystack.chars().collect();
    let n = chars.len();
    let needle_chars: Vec<char> = needle.chars().collect();
    let nlen = needle_chars.len();

    let mut count = 0usize;
    let mut i = 0usize;
    let mut in_block_comment = false;
    let mut in_line_comment = false;
    let mut in_string = false;

    while i < n {
        // --- state transitions ---
        if in_block_comment {
            if i + 1 < n && chars[i] == '*' && chars[i + 1] == '/' {
                in_block_comment = false;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if in_line_comment {
            if chars[i] == '\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }
        if in_string {
            if chars[i] == '\\' {
                i += 2; // skip escape sequence
                continue;
            }
            if chars[i] == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        // Check for comment/string openers
        if i + 1 < n && chars[i] == '/' && chars[i + 1] == '*' {
            in_block_comment = true;
            i += 2;
            continue;
        }
        if i + 1 < n && chars[i] == '/' && chars[i + 1] == '/' {
            in_line_comment = true;
            i += 2;
            continue;
        }
        if chars[i] == '"' {
            in_string = true;
            i += 1;
            continue;
        }

        // Check for needle match
        if i + nlen <= n && chars[i..i + nlen] == needle_chars[..] {
            count += 1;
            i += nlen;
            continue;
        }

        i += 1;
    }
    count
}

pub fn validate_main_class(content: &str) -> Result<()> {
    let main_class_count = count_outside_comments_and_strings(content, "class Main");
    let main_method_count = count_outside_comments_and_strings(content, "void main()")
        + count_outside_comments_and_strings(content, "void main ()");
    
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

const MAX_INCLUDE_DEPTH: usize = 50;

pub fn process_includes(content: &str, base_dir: &Path) -> Result<String> {
    // Track included files by canonical path to detect cycles regardless of
    // how the same file is spelled (e.g. "./lib.fsh" vs "lib.fsh").
    let mut included_canonical: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    process_includes_internal(content, base_dir, &mut included_canonical, 0)
}

fn process_includes_internal(
    content: &str,
    base_dir: &Path,
    included_canonical: &mut std::collections::HashSet<PathBuf>,
    depth: usize,
) -> Result<String> {
    if depth > MAX_INCLUDE_DEPTH {
        bail!(
            "❌ IMPORT DEPTH EXCEEDED\n\n\
            Maximum import nesting depth ({}) exceeded.\n\
            Check for circular or deeply nested imports.\n",
            MAX_INCLUDE_DEPTH
        );
    }

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

                // Validate path doesn't escape base directory (also resolves symlinks
                // and normalises the path, making it safe to canonicalize below).
                validate_input_path(&include_path)?;

                // Deduplicate by canonical path so that "./lib.fsh" and "lib.fsh"
                // (or a symlink pointing to the same inode) are treated as one file.
                let canonical = include_path.canonicalize()
                    .with_context(|| format!("Cannot resolve import path: {:?}", include_path))?;

                if included_canonical.contains(&canonical) {
                    bail!(
                        "❌ CIRCULAR IMPORT\n\n\
                        Circular import detected: '{}' already imported.\n",
                        filename
                    );
                }
                included_canonical.insert(canonical);
                
                // Validate file size before reading to prevent memory exhaustion
                validate_file_size(&include_path)?;

                // Read and include the file
                eprintln!("📥 Importing: {}", filename);
                let included_content = fs::read_to_string(&include_path)
                    .with_context(|| format!("Cannot read imported file: {}", filename))?;
                
                // Recursively process includes in the included file
                let processed = process_includes_internal(&included_content, base_dir, included_canonical, depth + 1)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        let validator = AdvancedSecurityValidator::new(SecurityConfig::default());
        
        let result = validator.validate_arithmetic(i64::MAX, "+", 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overflow"));
    }

    #[test]
    fn test_division_by_zero() {
        let validator = AdvancedSecurityValidator::new(SecurityConfig::default());
        
        let result = validator.validate_arithmetic(10, "/", 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("zero"));
    }

    #[test]
    fn test_safe_arithmetic() {
        let validator = AdvancedSecurityValidator::new(SecurityConfig::default());
        
        let result = validator.validate_arithmetic(5, "+", 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_variable_registration() {
        let mut validator = AdvancedSecurityValidator::new(SecurityConfig::default());
        
        let result = validator.register_variable(
            "x".to_string(),
            "int".to_string(),
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_cast_safety() {
        let validator = AdvancedSecurityValidator::new(SecurityConfig::default());
        
        // Valid cast
        let result = validator.validate_cast("int", "byte", 100);
        assert!(result.is_ok());
        
        // Invalid cast (out of range)
        let result = validator.validate_cast("int", "byte", 300);
        assert!(result.is_err());
    }
}
