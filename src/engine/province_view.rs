use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::core::{
    components::*,
    province::ProvinceDef,
    country::CountryDef,
    types::CountryTag,
};

const PROVINCE_RADIUS: f32 = 20.0;
const HOVER_OUTLINE_WIDTH: f32 = 2.0;

pub fn spawn_province_markers(
    commands: &mut Commands,
    provinces: &[ProvinceDef],
    countries: &[CountryDef],
) {
    for province in provinces.iter() {
        let pos = &province.pos;
        // Find owning country (first whose owned_provinces contains id)
        let mut owner_tag: Option<CountryTag> = None;
        let mut owner_color: Color = Color::GRAY;
        for c in countries {
            if let Some(owned) = &c.owned_provinces {
                if owned.iter().any(|pid| *pid == province.id) {
                    if let Ok(tag) = c.tag.parse() { owner_tag = Some(tag); }
                    if let Some(col) = &c.color { owner_color = Color::rgb(col.r, col.g, col.b); }
                    break;
                }
            }
        }
        let owner_tag = owner_tag.unwrap_or_else(|| "ZZZ".parse().unwrap());
        
        // Create circle shape
        let circle = shapes::Circle {
            radius: PROVINCE_RADIUS,
            center: Vec2::new(pos.x as f32, pos.y as f32),
        };

        let base_color = owner_color;

        commands.spawn(ProvinceBundle {
            marker: ProvinceMarker { id: province.id.into() },
            ownership: ProvinceOwnership { owner: owner_tag, controller: owner_tag },
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

fn get_country_color(_tag: &CountryTag) -> Color { Color::WHITE }
