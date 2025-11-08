use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Loads all YAML files from `game/` then overlays `/mods/*` (alphabetical)
/// Returns vector of (relative_path, contents)
pub fn load_game_and_mods(root: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut files: Vec<(PathBuf, String)> = Vec::new();

    let game_dir = root.join("game");
    if game_dir.exists() {
        for entry in WalkDir::new(&game_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "yaml" || ext == "yml" || ext == "json" {
                        let rel = entry.path().strip_prefix(root).unwrap_or(entry.path()).to_path_buf();
                        let s = std::fs::read_to_string(entry.path())
                            .with_context(|| format!("reading {}", entry.path().display()))?;
                        files.push((rel, s));
                    }
                }
            }
        }
    }

    // load mods (alphabetical) and append/override by path
    let mods_dir = root.join("mods");
    if mods_dir.exists() {
        let mut mod_dirs: Vec<_> = std::fs::read_dir(&mods_dir)?.filter_map(|e| e.ok()).collect();
        mod_dirs.sort_by_key(|e| e.file_name());
        for md in mod_dirs {
            if md.path().is_dir() {
                for entry in WalkDir::new(md.path()).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "yaml" || ext == "yml" || ext == "json" {
                                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path()).to_path_buf();
                                let s = std::fs::read_to_string(entry.path())
                                    .with_context(|| format!("reading {}", entry.path().display()))?;
                                // If same relative path exists, replace previous (mods override)
                                if let Some(pos) = files.iter().position(|(p, _)| p == &rel) {
                                    files[pos] = (rel, s);
                                } else {
                                    files.push((rel, s));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}
