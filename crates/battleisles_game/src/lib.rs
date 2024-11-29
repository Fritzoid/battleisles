use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use map::init_map;

mod center_marker;
mod env;
mod map;
mod ui;

use center_marker::center_marker;
use env::init_env;
use map::{HexType, MapInfo};

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin)
            .add_plugins(CursorRayPlugin)
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
    let map = MapInfo {
        width: 5,
        height: 3,
        hexes: vec![
            HexType::DeepWater,
            HexType::Plains,
            HexType::ShallowWater,
            HexType::Mountains,
            HexType::Hills,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::Hills,
            HexType::ShallowWater,
            HexType::ShallowWater,
            HexType::Plains,
            HexType::Plains,
            HexType::Mountains,
        ],
    };

    center_marker(&mut commands, &mut meshes, &mut materials);
    init_map(map, &mut meshes, &mut commands, &mut materials);
    init_env(&mut commands);
}
