use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::core::{
    components::*,
    province::ProvinceDef,
    types::CountryTag,
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

        let owner_tag: CountryTag = "GER".parse().unwrap();
        let base_color = get_country_color(&owner_tag);

        commands.spawn(ProvinceBundle {
            marker: ProvinceMarker { id: province.id.into() },
            ownership: ProvinceOwnership {
                owner: owner_tag,
                controller: owner_tag,
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
        .insert(Fill::color(base_color))
        .insert(Stroke::new(Color::WHITE, HOVER_OUTLINE_WIDTH));
    }
}

pub fn update_province_hover(
    mut provinces: Query<(&Transform, &mut Hoverable, &mut Stroke)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera.single();
            
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                for (transform, mut hoverable, mut stroke) in provinces.iter_mut() {
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

pub fn handle_province_selection(
    mut provinces: Query<(&mut Selectable, &Hoverable, &mut Fill, &ProvinceOwnership)>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (mut selectable, hoverable, mut fill, ownership) in provinces.iter_mut() {
            if hoverable.hovered {
                selectable.selected = !selectable.selected;
                
                // Set color based on owner + selection
                let base_color = get_country_color(&ownership.owner);
                let mut adjusted_color = base_color;
                
                // Lighten the color when selected
                if selectable.selected {
                    adjusted_color = Color::rgb(
                        (base_color.r() + 0.2).min(1.0),
                        (base_color.g() + 0.2).min(1.0),
                        (base_color.b() + 0.2).min(1.0),
                    );
                }
                
                fill.color = adjusted_color;
            }
        }
    }
}

fn get_country_color(tag: &CountryTag) -> Color {
    match tag.as_str().as_str() {
        "GER" => Color::rgb(0.2, 0.2, 0.7), // German blue
        "FRA" => Color::rgb(0.2, 0.7, 0.2), // French green
        "POL" => Color::rgb(0.7, 0.2, 0.2), // Polish red
        _ => Color::GRAY,
    }
}
