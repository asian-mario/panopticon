use anyhow::{Context, Result};
use jsonschema::{JSONSchema, Draft};
use serde_json::Value;
use std::path::Path;

/// Validate a JSON value against a JSON Schema (schema as serde_json::Value)
pub fn validate_value(schema: Value, doc: &Value) -> Result<()> {
    // JSONSchema::compile requires the schema reference to have 'static lifetime.
    // For a CLI validator it's acceptable to leak the owned schema to obtain a
    // &'static Value for compilation. This avoids lifetime issues; the memory
    // will be reclaimed by the OS when the process exits.
    let boxed = Box::new(schema);
    let schema_static: &'static serde_json::Value = Box::leak(boxed);
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema_static)
        .with_context(|| "compiling schema")?;
    let res = compiled.validate(doc);
    if let Err(errors) = res {
        let mut msgs = String::new();
        for e in errors {
            msgs.push_str(&format!("{}\n", e));
        }
        anyhow::bail!("schema validation failed:\n{}", msgs);
    }
    Ok(())
}

/// Load a JSON schema from file path (expects JSON)
pub fn load_schema(path: &Path) -> Result<Value> {
    let s = std::fs::read_to_string(path).with_context(|| format!("reading schema {}", path.display()))?;
    let v: Value = serde_json::from_str(&s).with_context(|| format!("parsing schema {}", path.display()))?;
    Ok(v)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_value_accepts_valid_doc_and_rejects_invalid() -> Result<(), anyhow::Error> {
        // simple schema: object with required "name" string
        let schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "required": ["name"],
            "properties": { "name": { "type": "string" } }
        });

        let good = json!({ "name": "test" });
        let bad = json!({ "name": 123 });

        validate_value(schema.clone(), &good)?;
        let res = validate_value(schema, &bad);
        assert!(res.is_err());
        Ok(())
    }
}
