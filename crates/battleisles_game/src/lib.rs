use battleisles_bevy::map_model::MapModel;
use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_camera::{OrthographicProjection, Projection, ScalingMode};

pub struct BattleIslesGame;
const MAP_FIT_PADDING: f32 = 1.05;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Battle Isles".to_owned(),
                    resizable: true,
                    canvas: Some("#bevy".to_owned()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(MapModelPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, fit_map_to_viewport)
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
            scaling_mode: ScalingMode::WindowSize,
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

fn fit_map_to_viewport(
    map_model: Option<Res<MapModel>>,
    windows: Query<&Window>,
    mut projections: Query<&mut Projection, With<Camera3d>>,
) {
    let Some(map_model) = map_model else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(mut projection) = projections.single_mut() else {
        return;
    };
    let map_size = map_model.map_size();
    if map_size.x <= 0.0 || map_size.y <= 0.0 || window.width() <= 0.0 || window.height() <= 0.0 {
        return;
    }

    if let Projection::Orthographic(OrthographicProjection {
        scaling_mode,
        scale,
        ..
    }) = &mut *projection
    {
        *scaling_mode = ScalingMode::WindowSize;
        *scale = (map_size.x / window.width()).max(map_size.y / window.height()) * MAP_FIT_PADDING;
    }
}
