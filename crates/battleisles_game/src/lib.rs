use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use map::init_map;

mod map;
mod ui;
mod center_marker;

use center_marker::center_marker;
use map::{MapInfo,HexType};

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

#[derive(Component)]
struct GameCamera;

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

    init_map(map, &mut meshes, &mut commands, &mut materials);
    center_marker(&mut commands, &mut meshes, &mut materials);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 20.0)) // Set initial position
                .looking_at(Vec3::ZERO, Vec3::Y), // Make the camera look at the origin
            ..default()
        },
        GameCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.0, 0.0),
            radius: Some(10.0),
            pitch_lower_limit: Some(0.1),
            pitch_upper_limit: Some(std::f32::consts::FRAC_PI_2),
            ..default()
        },
    ));

}

