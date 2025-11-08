use bevy_egui::{egui, EguiContexts};

pub fn ui_topbar_system(mut contexts: EguiContexts<'_, '_>) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("⏯").clicked() {
                // Toggle pause
            }
            
            ui.label("Speed: ");
            for i in 1..=5 {
                if ui.button(format!("{}×", i)).clicked() {
                    // Set speed
                }
            }
            
            ui.label("Date: 1936-01-01");
        });
    });
}
