use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

const CAMERA_SPEED: f32 = 500.0;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 3.0;
const ZOOM_SPEED: f32 = 1.0;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * CAMERA_SPEED * time.delta_seconds();
    }
}

pub fn camera_zoom(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = query.single_mut();
    
    for event in mouse_wheel.iter() {
        projection.scale = (projection.scale - event.y * ZOOM_SPEED * projection.scale * 0.1)
            .clamp(MIN_ZOOM, MAX_ZOOM);
    }
}