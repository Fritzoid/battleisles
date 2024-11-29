use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use map::init_map;

mod ui;
mod center_marker;
mod map;
mod env;

use center_marker::center_marker;
use map::{MapInfo,HexType};
use env::init_env;

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
        width: 3,
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
        ],
    };

    center_marker(&mut commands, &mut meshes, &mut materials);
    init_map(map, &mut meshes, &mut commands, &mut materials);
    init_env(&mut commands);
}

