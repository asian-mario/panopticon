use anyhow::{Context, Result};
use jsonschema::{JSONSchema, Draft};
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;
use petgraph::Graph;
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

use crate::core::{
    province::ProvincesList,
    adjacency::Adjacency,
    focus::FocusTree,
};

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

/// Structural validation for common game files that go beyond JSON Schema.
/// These checks detect cross-file issues like missing province references,
/// duplicated IDs, and cycles in focus trees.
pub fn structural_validations(files: &[(PathBuf, String)]) -> Result<(), anyhow::Error> {
    use std::fmt::Write;
    let mut errors: Vec<String> = Vec::new();

    // Collect provinces if present
    let provinces_file = files.iter().find(|(p, _)| p.ends_with("provinces.yaml")).map(|(_, s)| s);
    let adjacency_file = files.iter().find(|(p, _)| p.ends_with("adjacency.yaml")).map(|(_, s)| s);

    // Validate province references
    let mut province_ids = std::collections::HashSet::new();
    if let Some(provinces) = provinces_file {
        let def: ProvincesList = serde_yaml::from_str(provinces)?;
        for p in def.provinces {
            if !province_ids.insert(p.id) {
                errors.push(format!("Duplicate province ID {}", p.id));
            }
        }
    }

    // Validate adjacency references
    if let Some(adj) = adjacency_file {
        let def: Adjacency = serde_yaml::from_str(adj)?;
        for edge in def.edges {
            if !province_ids.contains(&edge.a) || !province_ids.contains(&edge.b) {
                errors.push(format!("Invalid province in adjacency edge {}↔{}", edge.a, edge.b));
            }
        }
    }

    // Focus tree cycle detection
    for (path, contents) in files {
        if path.ends_with("focus_tree.yaml") {
            let tree: FocusTree = serde_yaml::from_str(contents)?;
            
            // Build graph of focus dependencies
            let mut g = Graph::new();
            let mut node_map = HashMap::new();

            // Create nodes
            for focus in &tree.focuses {
                let idx = g.add_node(focus.id.clone());
                node_map.insert(focus.id.clone(), idx);
            }

            // Add edges for prerequisites
            for focus in &tree.focuses {
                let from = node_map[&focus.id];
                for prereq in &focus.prerequisites {
                    if let Some(&to) = node_map.get(prereq) {
                        g.add_edge(to, from, ());
                    } else {
                        errors.push(format!("Focus {} has unknown prerequisite {}", focus.id, prereq));
                    }
                }
            }

            // Check for cycles
            if is_cyclic_directed(&g) {
                errors.push(format!("Cycle detected in focus tree {}", path.display()));
            }
        }
    }

    let mut province_ids = std::collections::HashSet::new();
    if let Some(pv_s) = provinces_file {
        match serde_yaml::from_str::<serde_json::Value>(pv_s) {
            Ok(v) => {
                if let Some(arr) = v.get("provinces").and_then(|p| p.as_array()) {
                    for item in arr {
                        if let Some(id) = item.get("id").and_then(|i| i.as_u64()) {
                            let idu = id as u32;
                            if !province_ids.insert(idu) {
                                errors.push(format!("Duplicate province id {}", idu));
                            }
                        } else {
                            errors.push("Province entry missing numeric id".to_string());
                        }
                    }
                } else {
                    errors.push("provinces.yaml missing 'provinces' array".to_string());
                }
            }
            Err(e) => errors.push(format!("parsing provinces.yaml: {}", e)),
        }
    }

    if let Some(adj_s) = adjacency_file {
        match serde_yaml::from_str::<serde_json::Value>(adj_s) {
            Ok(v) => {
                if let Some(arr) = v.get("edges").and_then(|p| p.as_array()) {
                    for item in arr {
                        let a = item.get("a").and_then(|x| x.as_u64()).map(|x| x as u32);
                        let b = item.get("b").and_then(|x| x.as_u64()).map(|x| x as u32);
                        match (a, b) {
                            (Some(a), Some(b)) => {
                                if !province_ids.contains(&a) { errors.push(format!("Adjacency edge references missing province a={}", a)); }
                                if !province_ids.contains(&b) { errors.push(format!("Adjacency edge references missing province b={}", b)); }
                            }
                            _ => errors.push("Adjacency edge missing numeric a/b".to_string()),
                        }
                    }
                } else {
                    errors.push("adjacency.yaml missing 'edges' array".to_string());
                }
            }
            Err(e) => errors.push(format!("parsing adjacency.yaml: {}", e)),
        }
    }

    // Countries: check province references and capital validity
    for (p, s) in files.iter().filter(|(p, _)| p.to_string_lossy().contains("country.yaml")) {
        match serde_yaml::from_str::<serde_json::Value>(s) {
            Ok(v) => {
                if let Some(owned) = v.get("owned_provinces").and_then(|x| x.as_array()) {
                    for idv in owned {
                        if let Some(id) = idv.as_u64() {
                            let idu = id as u32;
                            if !province_ids.contains(&idu) {
                                errors.push(format!("Country {} references unknown owned_province {}", p.display(), idu));
                            }
                        }
                    }
                }
                if let Some(controlled) = v.get("controlled_provinces").and_then(|x| x.as_array()) {
                    for idv in controlled {
                        if let Some(id) = idv.as_u64() {
                            let idu = id as u32;
                            if !province_ids.contains(&idu) {
                                errors.push(format!("Country {} references unknown controlled_province {}", p.display(), idu));
                            }
                        }
                    }
                }
                if let Some(cap) = v.get("capital").and_then(|x| x.as_u64()).map(|x| x as u32) {
                    if !province_ids.contains(&cap) {
                        errors.push(format!("Country {} has capital referencing unknown province {}", p.display(), cap));
                    }
                }
            }
            Err(e) => errors.push(format!("parsing {}: {}", p.display(), e)),
        }
    }

    // Focus trees: prereqs exist and DAG check
    for (p, s) in files.iter().filter(|(p, _)| p.to_string_lossy().ends_with("focus_tree.yaml") || p.to_string_lossy().ends_with("focus_tree.yml")) {
        match serde_yaml::from_str::<crate::core::focus::FocusTree>(s) {
            Ok(ft) => {
                let ids: std::collections::HashSet<_> = ft.focuses.iter().map(|f| f.id.clone()).collect();
                // unique ids checked by set size vs vec len
                if ids.len() != ft.focuses.len() {
                    errors.push(format!("Focus tree {} contains duplicate focus ids", p.display()));
                }
                // build graph prereq -> id using a petgraph::Graph
                let mut g = Graph::<String, ()>::new();
                let mut node_map: HashMap<String, NodeIndex> = HashMap::new();
                for f in &ft.focuses {
                    let idx = g.add_node(f.id.clone());
                    node_map.insert(f.id.clone(), idx);
                }
                for f in &ft.focuses {
                    for pre in &f.prerequisites {
                        if !ids.contains(pre) {
                            errors.push(format!("Focus {} in {} has unknown prerequisite {}", f.id, p.display(), pre));
                        } else {
                            let a = node_map.get(pre).unwrap();
                            let b = node_map.get(&f.id).unwrap();
                            g.add_edge(*a, *b, ());
                        }
                    }
                }
                if is_cyclic_directed(&g) {
                    errors.push(format!("Focus tree {} contains cycles", p.display()));
                }
            }
            Err(e) => errors.push(format!("parsing {}: {}", p.display(), e)),
        }
    }

    // Units: battalions unique IDs
    for (p, s) in files.iter().filter(|(p, _)| p.to_string_lossy().ends_with("battalions.yaml") || p.to_string_lossy().ends_with("battalions.yml")) {
        match serde_yaml::from_str::<crate::core::units::Battalions>(s) {
            Ok(b) => {
                let ids: std::collections::HashSet<_> = b.battalions.iter().map(|b| b.id.clone()).collect();
                if ids.len() != b.battalions.len() {
                    errors.push(format!("Battalions {} contains duplicate battalion ids", p.display()));
                }
            }
            Err(e) => errors.push(format!("parsing {}: {}", p.display(), e)),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        let mut msg = String::new();
        writeln!(msg, "Structural validation failed:").unwrap();
        for error in &errors {
            writeln!(msg, "- {}", error).unwrap();
        }
        Err(anyhow::anyhow!(msg))
    }
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
