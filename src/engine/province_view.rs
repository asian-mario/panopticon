use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::core::{
    components::*,
    province::ProvinceDef,
    data::Pos,
};

const PROVINCE_RADIUS: f32 = 20.0;
const HOVER_OUTLINE_WIDTH: f32 = 2.0;

pub fn spawn_province_markers(
    mut commands: Commands,
    provinces: Vec<ProvinceDef>,
) {
    for province in provinces {
        let pos = province.pos;
        
        // Create circle shape
        let circle = shapes::Circle {
            radius: PROVINCE_RADIUS,
            center: Vec2::new(pos.x as f32, pos.y as f32),
        };

        commands.spawn(ProvinceBundle {
            marker: ProvinceMarker { id: province.id.into() },
            ownership: ProvinceOwnership {
                owner: "GER".parse().unwrap(), // Default for testing
                controller: "GER".parse().unwrap(),
            },
            hoverable: Hoverable { hovered: false },
            selectable: Selectable { selected: false },
            spatial: SpatialBundle::from_transform(
                Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0)
            ),
        })
        .insert(ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
            ..default()
        })
        .insert(Fill::color(Color::BLUE))
        .insert(Stroke::new(Color::WHITE, HOVER_OUTLINE_WIDTH));
    }
}

pub fn update_province_hover(
    mut provinces: Query<(Entity, &Transform, &mut Hoverable, &mut Stroke)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera.single();
            
            if let Some(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let world_pos = Vec2::new(world_pos.x, world_pos.y);

                for (_, transform, mut hoverable, mut stroke) in provinces.iter_mut() {
                    let province_pos = transform.translation.truncate();
                    let distance = world_pos.distance(province_pos);
                    
                    let is_hovered = distance < PROVINCE_RADIUS;
                    hoverable.hovered = is_hovered;
                    
                    stroke.color = if is_hovered { Color::YELLOW } else { Color::WHITE };
                }
            }
        }
    }
}
    let (camera, camera_transform) = camera.single();
    let window = windows.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            for (_, transform, mut hoverable) in &mut provinces {
                let province_pos = transform.translation.truncate();
                let distance = province_pos.distance(world_pos);
                hoverable.hovered = distance < PROVINCE_RADIUS;
            }
        }
    }
}

pub fn update_province_visuals(
    mut provinces: Query<(&Hoverable, &Selectable, &mut Stroke)>,
) {
    for (hoverable, selectable, mut stroke) in &mut provinces {
        stroke.color = if hoverable.hovered || selectable.selected {
            Color::WHITE
        } else {
            Color::DARK_GRAY
        };
    }
}