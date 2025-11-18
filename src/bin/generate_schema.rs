use crawler_schema::RuleFile;
use schemars::schema_for;
use serde_json::Value;

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

    // Output schema to stdout
    let json_string = serde_json::to_string_pretty(&schema_value)?;
    println!("{}", json_string);

    Ok(())
}
