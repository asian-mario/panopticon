use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adjacency { pub edges: Vec<crate::core::data::Edge> }
