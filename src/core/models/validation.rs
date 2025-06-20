// llmdoc/src/core/models/validation.rs

// Using jsonschema 0.30.0 API based on provided documentation
// ErrorIterator is confirmed unused by the compiler with the current working approach.
use jsonschema::{Validator as ActualJsonSchemaValidator, ValidationError};
use serde_json::Value;
use anyhow::Result;

// Renamed our struct to avoid conflict with jsonschema::Validator
// jsonschema::Validator does not take a lifetime parameter.
pub struct Validator {
    compiled_schema: ActualJsonSchemaValidator,
}

impl Validator {
    /// Creates a new validator from a JSON schema value using Draft 7.
    /// The schema_json's lifetime is handled by the jsonschema::Validator internally (e.g. by cloning or Arc).
    pub fn new(schema_json: &Value) -> Result<Self> {
        // Use draft7::options() to get a builder, then build()
        // The build method returns Result<Validator, ValidationError>
        // The ValidationError from build() will be tied to the lifetime of schema_json.
        let compiled_schema = jsonschema::draft7::options()
            .build(schema_json)
            .map_err(|validation_error: ValidationError| { // Lifetime elided, compiler should infer from schema_json
                // Format the single ValidationError from compilation
                anyhow::anyhow!("Failed to compile JSON schema: Error at '{}': {}", validation_error.instance_path, validation_error)
            })?;
        Ok(Validator { compiled_schema })
    }

    /// Validates a JSON value against the loaded schema.
    /// Returns Ok(()) if valid, or an Error with details if invalid.
    pub fn validate<'v>(&self, data: &'v Value) -> Result<()> { // 'v for data lifetime
        // jsonschema::Validator::validate returns Result<(), ErrorIterator>
        // The compiler insists the Err variant is a single ValidationError, not an ErrorIterator.
        // Let's proceed with that assumption, despite 0.30.0 source suggesting otherwise.
        match self.compiled_schema.validate(data) {
            Ok(_) => Ok(()),
            Err(single_validation_error) => { // Assuming this is ValidationError<'v>
                // If it's a single error, we don't iterate.
                let error_message = format!(
                    "Validation error at {}: {}",
                    single_validation_error.instance_path, single_validation_error
                );
                Err(anyhow::anyhow!("JSON validation failed: {}", error_message))
            }
        }
    }

    /// Checks if a JSON value is valid against the loaded schema.
    pub fn is_valid(&self, data: &Value) -> bool {
        self.compiled_schema.is_valid(data)
    }
}

/// Validates a task ID format (e.g., TASK-001).
pub fn is_valid_task_id(id: &str) -> bool {
    regex::Regex::new(r"^[A-Z]+-\d+$").unwrap().is_match(id)
}

/// Loads a schema from a file path, parses it into a serde_json::Value.
pub fn load_schema_value_from_file(schema_path: &std::path::Path) -> Result<Value> {
    let file_content = std::fs::read_to_string(schema_path)?;
    let schema_json: Value = serde_json::from_str(&file_content)?;
    Ok(schema_json)
}

pub fn validation_init_message() {
    tracing::debug!("Validation module initialized.");
}

#[cfg(test)]
mod tests {
    use super::*; // Imports Validator, load_schema_value_from_file, etc.
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_schema_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_validator_valid_data() -> Result<()> { // Renamed test to match struct
        let schema_json = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer", "minimum": 0 }
            },
            "required": ["name", "age"]
        });
        let validator = Validator::new(&schema_json)?;
        let valid_data = json!({ "name": "Alice", "age": 30 });
        assert!(validator.validate(&valid_data).is_ok());
        assert!(validator.is_valid(&valid_data));
        Ok(())
    }

    #[test]
    fn test_validator_invalid_data() -> Result<()> { // Renamed test
        let schema_json = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer", "minimum": 0 }
            },
            "required": ["name", "age"]
        });
        let validator = Validator::new(&schema_json)?;
        
        let invalid_data_missing_field = json!({ "name": "Bob" });
        assert!(validator.validate(&invalid_data_missing_field).is_err());
        assert!(!validator.is_valid(&invalid_data_missing_field));

        let invalid_data_wrong_type = json!({ "name": "Charlie", "age": "thirty" });
        assert!(validator.validate(&invalid_data_wrong_type).is_err());
        assert!(!validator.is_valid(&invalid_data_wrong_type));
        
        let invalid_data_below_minimum = json!({ "name": "David", "age": -5 });
        assert!(validator.validate(&invalid_data_below_minimum).is_err());
        assert!(!validator.is_valid(&invalid_data_below_minimum));
        Ok(())
    }

    #[test]
    fn test_load_schema_and_validate_with_validator() -> Result<()> { // Renamed test
        let schema_content = r#"{
            "type": "object",
            "properties": { "id": { "type": "string" } },
            "required": ["id"]
        }"#;
        let schema_file = create_temp_schema_file(schema_content);
        let schema_json_val = load_schema_value_from_file(schema_file.path())?;
        
        let validator = Validator::new(&schema_json_val)?;

        let valid_data = json!({ "id": "test-id" });
        assert!(validator.validate(&valid_data).is_ok());

        let invalid_data = json!({ "name": "no-id" });
        assert!(validator.validate(&invalid_data).is_err());
        Ok(())
    }

    #[test]
    fn test_is_valid_task_id() {
        assert!(is_valid_task_id("TASK-001"));
        assert!(is_valid_task_id("SPRINT-123"));
        assert!(!is_valid_task_id("task-001")); // Lowercase prefix
        assert!(!is_valid_task_id("TASK001")); // Missing hyphen
        assert!(!is_valid_task_id("TASK-ABC")); // Non-numeric suffix
        assert!(!is_valid_task_id("TASK-001-EXTRA")); // Extra parts
        assert!(!is_valid_task_id("")); // Empty string
    }
}