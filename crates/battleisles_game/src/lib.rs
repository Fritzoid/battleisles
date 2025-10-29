use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_camera::{OrthographicProjection, Projection, ScalingMode};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod ui;

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Battle Isles".to_owned(),
                    resolution: (800, 600).into(),
                    resizable: true,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin::default())
            .add_plugins(MapModelPlugin)
            .add_systems(Startup, setup)
            .add_systems(EguiPrimaryContextPass, ui::ui_system)
            .run();
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d { ..default() },
        Projection::Orthographic(OrthographicProjection {
            scale: 0.1,
            scaling_mode: ScalingMode::Fixed {
                width: 800.0,
                height: 600.0,
            },
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Initialize any resources or entities needed for the editor
    MapModelPlugin::initialize_map_model(
        Map::new(5, 5),
        &mut commands,
        &mut meshes,
        &mut materials,
    )
    .expect("Failed to initialize map model");
}
