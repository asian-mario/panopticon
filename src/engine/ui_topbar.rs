use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use crate::core::time::Clock;

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

pub fn ui_topbar_system(mut contexts: EguiContexts<'_, '_>, mut clock: ResMut<Clock>) {
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
        });
    });
}
