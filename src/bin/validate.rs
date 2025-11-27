use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;

use panopticon::{
    content::{loader, validator},
    core::country::CountryDef,
};

fn main() -> Result<()> {
    println!("Running panopticon validate (content loader + schema checks)");

    let args: Vec<String> = std::env::args().collect();
    let root = if args.len() > 1 { Path::new(&args[1]) } else { Path::new(".") };
    
    // Load all content files
    let files = loader::load_game_and_mods(root)
        .with_context(|| format!("loading game and mods from {}", root.display()))?;
    println!("Loaded {} content files (game + mods)", files.len());

    let mut errors = Vec::new();

    // Schema validation
    let schemas_dir = root.join("schemas");
    if schemas_dir.exists() {
        for (path, contents) in &files {
            if let Some(stem) = path.file_stem() {
                let json_schema_path = schemas_dir.join(format!("{}.schema.json", stem.to_string_lossy()));
                if json_schema_path.exists() {
                    match validator::load_schema(&json_schema_path)
                        .and_then(|schema| {
                            // try parsing content as JSON first, then YAML -> JSON
                            let doc: Value = match serde_json::from_str(contents) {
                                Ok(v) => v,
                                Err(_) => serde_yaml::from_str::<Value>(contents)?,
                            };
                            validator::validate_value(schema, &doc)
                                .with_context(|| format!("validating {} against schema", path.display()))
                        }) {
                        Ok(_) => println!("✓ Schema validation passed for {}", path.display()),
                        Err(e) => {
                            eprintln!("✗ Schema validation failed for {}\n  {}", path.display(), e);
                            errors.push(e);
                        }
                    }
                }
            }
        }
    }

    // Load and validate country definitions
    // Only validate actual country definition files, not every file inside countries/*
    let country_files = files.iter()
        .filter(|(p, _)| p.ends_with("country.yaml"));
    
    for (path, contents) in country_files {
        match serde_yaml::from_str::<CountryDef>(contents) {
            Ok(_) => println!("✓ Valid country definition in {}", path.display()),
            Err(e) => errors.push(e.into()),
        }
    }

    // run structural validations (cross-file checks) AFTER schemas
    match validator::structural_validations(&files) {
        Ok(()) => (),
        Err(e) => errors.push(e),
    }

    // Extra country sanity (duplicate with schema but keeps future extensibility)
    for (path, contents) in files.iter().filter(|(p, _)| p.ends_with("country.yaml")) {
        match serde_yaml::from_str::<CountryDef>(contents) {
            Ok(_) => (),
            Err(e) => errors.push(e.into()),
        }
        println!("Checked country file {}", path.display());
    }

    if errors.is_empty() {
        println!("Validation complete. No errors.");
        Ok(())
    } else {
        eprintln!("Validation complete with {} error(s).", errors.len());
        for (i, e) in errors.into_iter().enumerate() {
            eprintln!("[{}] {}", i + 1, e);
        }
        Err(anyhow::anyhow!("validation failed"))
    }
}
