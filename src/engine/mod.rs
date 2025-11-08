use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod camera;
mod ui_topbar;
mod province_view;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, ShapePlugin))
            .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // Dark gray background
            .add_systems(Startup, (
                camera::setup_camera,
                startup_spawn_provinces,
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

fn startup_spawn_provinces(mut commands: Commands) {
    // Test data - replace with actual loading later
    let test_provinces = vec![
        crate::core::province::ProvinceDef {
            id: 1,
            name: "Berlin".to_string(),
            pos: crate::core::data::Pos { x: 0, y: 0 },
        },
        crate::core::province::ProvinceDef {
            id: 2,
            name: "Paris".to_string(),
            pos: crate::core::data::Pos { x: 100, y: 0 },
        },
        crate::core::province::ProvinceDef {
            id: 3,
            name: "London".to_string(),
            pos: crate::core::data::Pos { x: 0, y: 100 },
        },
    ];
    
    province_view::spawn_province_markers(commands, test_provinces);
}