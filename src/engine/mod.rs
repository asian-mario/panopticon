use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod camera;
mod ui_topbar;
mod province_view;

use crate::core::province::{ProvincesList, ProvinceDef};
use crate::core::focus::FocusTree;
use crate::core::country::CountryDef;
use std::fs;
use std::path::Path;

#[derive(Resource, Default)]
pub struct LoadedProvinces(pub Vec<ProvinceDef>);
#[derive(Resource, Default)]
pub struct LoadedFocusTree(pub FocusTree);
#[derive(Resource, Default)]
pub struct LoadedCountries(pub Vec<CountryDef>);
#[derive(Resource)]
pub struct PlayerCountry(pub Option<String>); // active country tag

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, ShapePlugin))
            .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // Dark gray background
            .add_systems(Startup, (
                camera::setup_camera,
                load_and_spawn_provinces,
            ))
            .add_systems(Update, (
                ui_topbar::ui_topbar_system,
                province_view::update_province_hover,
                province_view::handle_province_selection,
                camera::camera_movement,
                camera::camera_zoom,
            ));
    }
}

fn load_and_spawn_provinces(mut commands: Commands) {
    let path = Path::new("game/map/provinces.yaml");
    let mut list: Vec<ProvinceDef> = Vec::new();
    if let Ok(contents) = fs::read_to_string(path) {
        if let Ok(pv_list) = serde_yaml::from_str::<ProvincesList>(&contents) {
            list = pv_list.provinces;
        } else {
            eprintln!("Failed to parse provinces.yaml; falling back to empty list");
        }
    } else {
        eprintln!("Could not read provinces.yaml; no provinces spawned");
    }
    // Load countries
    let mut countries: Vec<CountryDef> = Vec::new();
    let countries_root = Path::new("game/countries");
    if let Ok(entries) = fs::read_dir(countries_root) {
        for ent in entries.flatten() {
            if ent.path().is_dir() {
                let file = ent.path().join("country.yaml");
                if file.exists() {
                    if let Ok(s) = fs::read_to_string(&file) {
                        if let Ok(def) = serde_yaml::from_str::<CountryDef>(&s) {
                            countries.push(def);
                        }
                    }
                }
            }
        }
    }
    let player_tag = countries.first().map(|c| c.tag.clone());
    commands.insert_resource(PlayerCountry(player_tag));
    commands.insert_resource(LoadedCountries(countries.clone()));

    province_view::spawn_province_markers(&mut commands, &list, &countries);
    commands.insert_resource(LoadedProvinces(list));

    // Load focus tree for GER (v0.1 demo scope)
    let focus_path = Path::new("game/countries/GER/focus_tree.yaml");
    if let Ok(focus_contents) = fs::read_to_string(focus_path) {
        if let Ok(tree) = serde_yaml::from_str::<FocusTree>(&focus_contents) {
            commands.insert_resource(LoadedFocusTree(tree));
        } else {
            eprintln!("Failed to parse focus_tree.yaml");
        }
    }
}