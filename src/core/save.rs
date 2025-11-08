use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSave {
    pub version: String,
    pub day: u32,
}
