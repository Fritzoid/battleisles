use crate::GenerateMapEvent;
use battleisles_bevy::map_model_plugin::ApplyTerrainAt;
use battleisles_domain::map::Terrain;
use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct UiState {
    pub map_width: String,
    pub map_height: String,
    pub selected_terrain: Terrain,
}

// Send paint events when user clicks in the main viewport (not over egui)
pub fn paint_click_system(
    mut contexts: EguiContexts,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    ui_state: Res<UiState>,
    mut paint_events: EventWriter<ApplyTerrainAt>,
) {
    let ctx = contexts.ctx_mut();
    if ctx.wants_pointer_input() {
        return;
    }
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let (camera, camera_transform) = match q_camera.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    if let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
        paint_events.write(ApplyTerrainAt {
            world_pos: world,
            terrain: ui_state.selected_terrain,
        });
    }
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

    // Left panel: terrain palette
    egui::SidePanel::left("left_panel")
        .default_width(140.0)
        .show(ctx, |ui| {
            ui.heading("Terrain");
            ui.separator();
            terrain_palette(ui, &mut ui_state.selected_terrain);
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

impl Default for UiState {
    fn default() -> Self {
        Self { map_width: String::new(), map_height: String::new(), selected_terrain: Terrain::Plains }
    }
}

fn terrain_palette(ui: &mut egui::Ui, selected: &mut Terrain) {
    use bevy_color::palettes::basic::*;
    let items = [
        (Terrain::Plains, GREEN, "Plains"),
        (Terrain::Hills, OLIVE, "Hills"),
        (Terrain::Mountains, GRAY, "Mountains"),
        (Terrain::DeepWater, BLUE, "Deep Water"),
        (Terrain::ShallowWater, AQUA, "Shallow Water"),
    ];

    for (terrain, color, label) in items {
        let size = egui::vec2(40.0, 40.0);
        let (id, rect) = ui.allocate_space(size);
    let stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);
        let fill = egui::Color32::from_rgb(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        );
        let center = rect.center();
        let r = 18.0;
        let mut points = Vec::with_capacity(6);
        for i in 0..6 {
            let a = (i as f32) * std::f32::consts::TAU / 6.0 + std::f32::consts::FRAC_PI_6; // pointy-top
            points.push(center + egui::vec2(a.cos() * r, a.sin() * r));
        }
        let painter = ui.painter_at(rect);
        painter.add(egui::epaint::PathShape::convex_polygon(points.clone(), fill, stroke));
        let is_selected = *selected == terrain;
        let resp = ui.interact(rect, id, egui::Sense::click());
        if resp.clicked() { *selected = terrain; }
        ui.label(label);
        if is_selected {
            let sel_stroke = egui::Stroke::new(2.0, egui::Color32::YELLOW);
            painter.add(egui::epaint::PathShape::convex_polygon(
                points,
                egui::Color32::TRANSPARENT,
                sel_stroke,
            ));
        }
        ui.add_space(4.0);
    }
}

