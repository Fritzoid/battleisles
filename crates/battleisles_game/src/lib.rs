use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;

mod ui;

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Battle Isles".to_owned(),
                    resolution: (800.0, 600.0).into(),
                    resizable: true,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false, })
            .add_plugins(MapModelPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Initialize any resources or entities needed for the editor
    MapModelPlugin::initialize_map(
        Map::default(),
        &mut commands,
        &mut meshes, 
        &mut materials,
    ).expect("Failed to initialize map model");
}