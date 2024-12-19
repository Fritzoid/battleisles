use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy::pbr::ExtendedMaterial;

mod center;
mod env;
mod map;
mod ui;
mod water;

use center::center;
use water::{Water, WaterPlugin};
use env::init_env;
use map::init_map;
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
            .add_plugins(EguiPlugin)
            .add_plugins(WaterPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, Water>>>,
    asset_server: Res<AssetServer>,

) {
    // Make a map that is 20 x 20 hexes and set them all to deepwater
     let map = MapInfo {
        width: 20,
        height: 20,
        hexes: vec![HexType::DeepWater; 20 * 20],
    };

    /* 
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
    */

    center(&mut commands, &mut meshes, &mut materials);
    init_map(map, &mut meshes, &mut commands, &mut materials, &mut water_materials, &asset_server);
    init_env(&mut commands, &asset_server);
}
