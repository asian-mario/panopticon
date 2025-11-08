use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceDef {
    pub id: u32,
    pub name: String,
    pub pos: crate::core::data::Pos,
}
