use std::fs::File;
use std::io::Read;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

use panopticon::core::data::{Edge, Pos};
use panopticon::core::focus::FocusTree;
use panopticon::core::province::ProvinceDef;
use panopticon::core::adjacency::Adjacency;
use panopticon::core::country::CountryDef;
use panopticon::core::units::Battalions;
use panopticon::content::loader;
use panopticon::content::validator;

#[derive(Debug, Deserialize)]
struct ProvincesList { provinces: Vec<ProvinceDef> }

fn load_yaml<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Result<T> {
    let mut s = String::new();
    File::open(path).with_context(|| format!("opening {}", path))?.read_to_string(&mut s)?;
    let v = serde_yaml::from_str(&s).with_context(|| format!("parsing {}", path))?;
    Ok(v)
}

fn main() -> Result<()> {
    println!("Running panopticon validate (content loader + schema checks)");

    let args: Vec<String> = std::env::args().collect();
    let root = if args.len() > 1 { Path::new(&args[1]) } else { Path::new(".") };
    let files = loader::load_game_and_mods(root).with_context(|| format!("loading game and mods from {}", root.display()))?;
    println!("Loaded {} content files (game + mods)", files.len());

    let mut errors: Vec<anyhow::Error> = Vec::new();

    // If schemas exist, try to validate matching files
    let schemas_dir = root.join("schemas");
    if schemas_dir.exists() {
        for (path, contents) in &files {
            // heuristic: if there's a schema with same file stem, validate
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
                        }) {
                        Ok(()) => println!("Validated {} against {}", path.display(), json_schema_path.display()),
                        Err(e) => {
                            eprintln!("Validation error for {}: {}", path.display(), e);
                            errors.push(e);
                        }
                    }
                }
            }
        }
    }

    // run original basic checks for map files if present
    if let (Some(pv), Some(adj)) = (
        files.iter().find(|(p, _)| p.ends_with("provinces.yaml")),
        files.iter().find(|(p, _)| p.ends_with("adjacency.yaml")),
    ) {
        match (serde_yaml::from_str::<ProvincesList>(&pv.1), serde_yaml::from_str::<Adjacency>(&adj.1)) {
            (Ok(provinces), Ok(adjm)) => {
                let ids: std::collections::HashSet<u32> = provinces.provinces.iter().map(|p| p.id).collect();
                for e in &adjm.edges {
                    if !ids.contains(&e.a) {
                        errors.push(anyhow::anyhow!("Adjacency edge references missing province a={}", e.a));
                    }
                    if !ids.contains(&e.b) {
                        errors.push(anyhow::anyhow!("Adjacency edge references missing province b={}", e.b));
                    }
                }
                println!("Provinces: {} entries", provinces.provinces.len());
                println!("Adjacency edges: {}", adjm.edges.len());
            }
            (Err(e), _) => errors.push(e.into()),
            (_, Err(e)) => errors.push(e.into()),
        }
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
