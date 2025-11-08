use bevy::prelude::*;

mod camera;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1))) // Dark gray background
            .add_systems(Startup, camera::setup_camera);
    }
}