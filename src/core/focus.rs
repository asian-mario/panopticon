use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Focus {
    pub id: String,
    pub name: String,
    pub days: u32,
    #[serde(default)]
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusTree { pub focuses: Vec<Focus> }
