use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;

mod map_model;
use map_model::MapModel;
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
                    resizable: false,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            //.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false, })
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map_model = MapModel::try_new(
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    commands.insert_resource(map_model);
}

