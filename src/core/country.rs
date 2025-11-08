use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryDef {
    pub tag: String,
    pub name: String,
    pub color: Option<Color>,
    pub ideology: Option<String>,
    pub resources: Option<serde_json::Value>,
    pub owned_provinces: Option<Vec<u32>>,
    pub controlled_provinces: Option<Vec<u32>>,
    pub capital: Option<u32>,
}
