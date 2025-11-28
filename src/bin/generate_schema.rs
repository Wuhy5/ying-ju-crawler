use crawler_schema::core::CrawlerRule;
use schemars::schema_for;
use serde_json::Value;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(CrawlerRule);

    let mut schema_value: Value = serde_json::to_value(schema)?;

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
