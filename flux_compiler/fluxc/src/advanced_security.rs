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

pub fn validate_output_path(path: &Path) -> Result<()> {
    // Check for ".." in path which indicates path traversal attempt
    let path_str = path.to_string_lossy();
    if path_str.contains("..") {
        bail!("Path traversal detected: {:?} contains '..'", path);
    }
    
    // Get the project's root directory, not the current working directory
    let project_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // Convert to absolute path
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        project_dir.join(path)
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
    
    // Check path doesn't escape project directory
    let normalized = match absolute_path.canonicalize() {
        Ok(canon) => canon,
        Err(_) => {
            // If canonicalize fails, it might be because the file doesn't exist yet.
            // In that case, we rely on the check against the non-canonicalized absolute path.
            absolute_path.clone()
        }
    };
    
    let normalized_project_dir = project_dir.canonicalize().unwrap_or(project_dir);

    if !normalized.starts_with(&normalized_project_dir) {
        bail!(
            "Path traversal detected: {:?} is outside allowed project directory {:?}",
            normalized,
            normalized_project_dir
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

pub fn validate_main_class(content: &str) -> Result<()> {
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

pub fn process_includes(content: &str, base_dir: &Path) -> Result<String> {
    let mut included_files = std::collections::HashSet::new();
    process_includes_internal(content, base_dir, &mut included_files)
}

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
