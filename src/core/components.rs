use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::core::types::*;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct FocusProgress {
    pub focus_id: FocusId,
    pub days_remaining: u32,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct ResearchProgress {
    pub tech_id: String,
    pub days_remaining: u32,
}

// Province Components
#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct ProvinceOwnership {
    pub owner: CountryTag,
    pub controller: CountryTag,
}

#[derive(Component, Debug)]
pub struct ProvinceMarker {
    pub id: ProvinceId,
}

#[derive(Component, Debug)]
pub struct Hoverable {
    pub hovered: bool,
}

#[derive(Component, Debug)]
pub struct Selectable {
    pub selected: bool,
}

#[derive(Bundle)]
pub struct ProvinceBundle {
    pub marker: ProvinceMarker,
    pub ownership: ProvinceOwnership,
    pub hoverable: Hoverable,
    pub selectable: Selectable,
    pub spatial: SpatialBundle,
}

// Division Components
#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct DivisionComponent {
    pub id: DivisionId,
    pub location: ProvinceId,
}

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct DivisionMovement {
    pub to: ProvinceId,
    pub days_left: u32,
}