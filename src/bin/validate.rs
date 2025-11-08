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
    let country_files = files.iter()
        .filter(|(p, _)| p.components().any(|c| c.as_os_str() == "countries"));
    
    for (path, contents) in country_files {
        match serde_yaml::from_str::<CountryDef>(contents) {
            Ok(_) => println!("✓ Valid country definition in {}", path.display()),
            Err(e) => errors.push(e.into()),
        }
    }

    // Print summary and exit with appropriate code
    if errors.is_empty() {
        println!("\n✓ All validations passed!");
        return Ok(());
    } else {
        eprintln!("Validation complete with {} error(s).", errors.len());
        for (i, e) in errors.into_iter().enumerate() {
            eprintln!("{}. {}", i + 1, e);
        }
        std::process::exit(1);
    }

    // run structural validations (cross-file checks)
    match validator::structural_validations(&files) {
        Ok(()) => (),
        Err(e) => errors.push(e),
    }

    // check countries
    if let Some(cn) = files.iter().find(|(p, _)| p.ends_with("country.yaml")) {
        match serde_yaml::from_str::<CountryDef>(&cn.1) {
            Ok(ger) => println!("Country {} tag: {}", cn.0.display(), ger.tag),
            Err(e) => errors.push(e.into()),
        }
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
