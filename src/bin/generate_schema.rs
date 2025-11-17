use crawler_schema::RuleFile;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::Path;

// Get version from Cargo.toml at compile time
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate JSON Schema automatically from RuleFile struct
    let schema = schema_for!(RuleFile);

    // Convert schema to JSON value to add version info
    let mut schema_value: Value = serde_json::to_value(schema)?;
    
    // Add version info to $comment field
    if let Some(obj) = schema_value.as_object_mut() {
        obj.insert(
            "$comment".to_string(),
            Value::String(format!("Schema version: {}", VERSION)),
        );
    }

    // Output path
    let output_path = "../ying-ju-crawler-docs/docs/schema/schema.json";

    // Create parent directory if needed
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Write schema to file
    let json_string = serde_json::to_string_pretty(&schema_value)?;
    fs::write(output_path, json_string)?;

    println!("✓ JSON Schema generated successfully at: {}", output_path);
    println!("  Schema version: {}", VERSION);

    Ok(())
}
