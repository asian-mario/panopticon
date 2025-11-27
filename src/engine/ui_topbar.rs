use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use crate::core::time::Clock;
use crate::engine::{LoadedCountries, PlayerCountry};

fn format_date_from_day(day0: u32) -> String {
    // Assume epoch 1936-01-01 and 30-day months for a simple UI stub
    let base_year = 1936u32;
    let mut days = day0;
    let mut year = base_year;
    let mut month = 1u32;
    let mut day = 1u32;
    while days > 0 {
        day += 1;
        if day > 30 {
            day = 1;
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
            }
        }
        days -= 1;
    }
    format!("{:04}-{:02}-{:02}", year, month, day)
}

pub fn ui_topbar_system(
    mut contexts: EguiContexts<'_, '_>,
    mut clock: ResMut<Clock>,
    countries: Option<Res<LoadedCountries>>,
    mut player: ResMut<PlayerCountry>,
) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button(if clock.paused { "▶" } else { "⏸" }).clicked() {
                clock.paused = !clock.paused;
            }
            
            ui.label("Speed: ");
            for i in 1..=5 {
                if ui.button(format!("{}×", i)).clicked() {
                    clock.speed_idx = (i - 1) as usize;
                }
            }
            
            ui.label(format!("Date: {}", format_date_from_day(clock.current_day)));

            if let Some(cres) = &countries {
                ui.separator();
                ui.label("Country:");
                egui::ComboBox::from_id_source("country_select").selected_text(
                    player.0.clone().unwrap_or_else(|| "None".into())
                ).show_ui(ui, |ui| {
                    for c in &cres.0 {
                        let tag = &c.tag;
                        if ui.selectable_label(player.0.as_ref() == Some(tag), tag).clicked() {
                            player.0 = Some(tag.clone());
                        }
                    }
                });
                if let Some(active) = &player.0 {
                    if let Some(cdef) = cres.0.iter().find(|c| &c.tag == active) {
                        ui.label(format!("Manpower: {}", cdef.resources.as_ref().and_then(|r| r.get("manpower")).and_then(|m| m.as_u64()).unwrap_or(0)));
                        ui.label(format!("Civ: {} Mil: {}", 
                            cdef.resources.as_ref().and_then(|r| r.get("civ_factories")).and_then(|v| v.as_u64()).unwrap_or(0),
                            cdef.resources.as_ref().and_then(|r| r.get("mil_factories")).and_then(|v| v.as_u64()).unwrap_or(0)));
                    }
                }
            }
        });
    });
}
