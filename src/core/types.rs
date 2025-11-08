use bevy::prelude::*;

use std::str::FromStr;
use bevy::prelude::Component;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize, Component)]
pub struct ProvinceId(pub u32);

impl From<u32> for ProvinceId {
    fn from(id: u32) -> Self {
        ProvinceId(id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize, Component)]
pub struct DivisionId(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize, Component)]
pub struct CountryTag([char; 3]);

impl CountryTag {
    pub fn as_str(&self) -> String {
        self.0.iter().collect()
    }
}

impl FromStr for CountryTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 3 {
            anyhow::bail!("CountryTag must be exactly 3 characters");
        }
        Ok(CountryTag([chars[0], chars[1], chars[2]]))
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FocusId(pub String);

use serde::{Serialize, Deserialize};