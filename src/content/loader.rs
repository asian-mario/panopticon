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
                                // For mods, compute the path relative to the mod directory so that
                                // mods/<modname>/game/path.yaml maps to game/path.yaml and can override.
                                let rel = match entry.path().strip_prefix(md.path()) {
                                    Ok(p) => p.to_path_buf(),
                                    Err(_) => entry.path().strip_prefix(root).unwrap_or(entry.path()).to_path_buf(),
                                };
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


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn loader_reads_game_and_mods_and_mod_overrides() -> Result<(), anyhow::Error> {
        let dir = tempdir()?;
        let root = dir.path();

        // create game/foo.yaml
        let game_dir = root.join("game");
        fs::create_dir_all(&game_dir)?;
        fs::write(game_dir.join("foo.yaml"), "value: game")?;

    // create mods/mod1/game/foo.yaml overriding the game file by relative path
    let mods_dir = root.join("mods").join("mod1").join("game");
    fs::create_dir_all(&mods_dir)?;
    fs::write(mods_dir.join("foo.yaml"), "value: mod")?;

        let files = load_game_and_mods(root)?;
        // should contain single entry for foo.yaml with mod content
        assert_eq!(files.len(), 1);
        let (p, contents) = &files[0];
        assert!(p.ends_with("game/foo.yaml") || p.ends_with("mods/mod1/foo.yaml") || p.ends_with("foo.yaml"));
        assert!(contents.contains("mod"));
        Ok(())
    }
}
