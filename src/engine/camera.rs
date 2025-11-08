use bevy::prelude::*;

// Basic orthographic camera setup for 2D
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}