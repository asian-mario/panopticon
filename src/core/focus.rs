use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Focus {
    pub id: String,
    pub name: String,
    pub days: u32,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub mutually_exclusive: Vec<String>,
    #[serde(default)]
    pub effects: Vec<RawEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEffect {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(flatten)]
    pub params: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FocusTree { pub focuses: Vec<Focus> }

impl FocusTree {
    pub fn find(&self, id: &str) -> Option<&Focus> { self.focuses.iter().find(|f| f.id == id) }
}
