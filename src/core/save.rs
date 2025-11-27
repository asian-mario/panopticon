use serde::{Deserialize, Serialize};
use crate::core::time::Clock;
use crate::core::province::ProvinceDef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSave {
    pub version: String,
    pub day: u32,
    pub provinces: Vec<ProvinceDef>,
}

impl GameSave {
    pub fn from_world(clock: &Clock, provinces: &[ProvinceDef]) -> Self {
        GameSave {
            version: "0.1".to_string(),
            day: clock.current_day,
            provinces: provinces.to_vec(),
        }
    }
}

pub fn save_to_path(save: &GameSave, path: &std::path::Path) -> anyhow::Result<()> {
    let s = serde_json::to_string_pretty(save)?;
    std::fs::write(path, s)?;
    Ok(())
}

pub fn load_from_path(path: &std::path::Path) -> anyhow::Result<GameSave> {
    let s = std::fs::read_to_string(path)?;
    let save: GameSave = serde_json::from_str(&s)?;
    Ok(save)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::core::time::Clock;
    use crate::core::data::Pos;
    use crate::core::province::ProvinceDef;

    #[test]
    fn save_and_load_roundtrip() {
        let clock = Clock { current_day: 42, paused: false, speed_idx: 0, acc: 0.0 };
        let provinces = vec![ProvinceDef { id: 7, name: "Test".into(), pos: Pos { x: 10, y: 20 } }];
        let save = GameSave::from_world(&clock, &provinces);
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_save.json");
        save_to_path(&save, &path).unwrap();
        let loaded = load_from_path(&path).unwrap();
        assert_eq!(loaded.version, "0.1");
        assert_eq!(loaded.day, 42);
        assert_eq!(loaded.provinces.len(), 1);
        assert_eq!(loaded.provinces[0].id, 7);
    }
}
