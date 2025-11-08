use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub id: u32,
    pub name: String,
    pub pos: Pos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
}
