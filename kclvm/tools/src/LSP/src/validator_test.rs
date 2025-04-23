#[cfg(test)]
mod tests {
    use super::*;
    use kclvm_parser::parse_program;
    use std::path::PathBuf;

    #[test]
    fn test_schema_validation() {
        let test_file = PathBuf::from("src/test_data/schema_validation/test_schema.k");
        let program = parse_program(&[test_file.clone()], None).unwrap();
        let scope = ProgramScope::new(&program);
        
        let result = validate_schema_attributes(&program, &scope);
        
        assert!(result.is_err(), "Expected validation errors");
        let diagnostics = result.unwrap_err();
        
        // Should find exactly 2 validation errors
        assert_eq!(diagnostics.len(), 2, "Expected 2 validation errors");
        
        // Verify first error (missing age)
        let error1 = &diagnostics[0];
        assert_eq!(error1.level, Level::Error);
        assert!(error1.message.contains("Missing required attributes in Person instance: age"));
        assert_eq!(error1.range.0.line, 14);
        
        // Verify second error (missing name)
        let error2 = &diagnostics[1];
        assert_eq!(error2.level, Level::Error);
        assert!(error2.message.contains("Missing required attributes in Person instance: name"));
        assert_eq!(error2.range.0.line, 19);
    }
} 