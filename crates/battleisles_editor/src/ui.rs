use crate::GenerateMapEvent;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource, Default)]
pub struct UiState {
    pub map_width: String,
    pub map_height: String,
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut map_events: EventWriter<GenerateMapEvent>,
) {
    let ctx = contexts.ctx_mut();

    // Top panel
    egui::TopBottomPanel::top("top_panel")
        .default_height(50.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Map Width:");
                ui.add(
                    egui::TextEdit::singleline(&mut ui_state.map_width)
                        .hint_text("Width")
                        .desired_width(60.0),
                );
                ui.label("Map Height:");
                ui.add(
                    egui::TextEdit::singleline(&mut ui_state.map_height)
                        .hint_text("Height")
                        .desired_width(60.0),
                );
                if ui.add(egui::Button::new("Generate Map")).clicked() {
                    if let (Ok(width), Ok(height)) = (
                        ui_state.map_width.parse::<u32>(),
                        ui_state.map_height.parse::<u32>(),
                    ) {
                        map_events.write(GenerateMapEvent { width, height });
                    }
                }
            });
        });

    // Bottom panel
    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(50.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Bottom Panel"));
        });

    // Left panel
    egui::SidePanel::left("left_panel")
        .default_width(100.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Left Panel"));
        });

    // Right panel
    egui::SidePanel::right("right_panel")
        .default_width(100.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Right Panel"));
        });

    // Set the background color of the panels to light blue
    ctx.set_visuals(egui::Visuals {
        panel_fill: egui::Color32::from_rgb(173, 216, 230),
        ..Default::default()
    });
}
