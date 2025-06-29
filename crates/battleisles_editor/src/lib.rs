use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResized};
use bevy_egui::EguiPlugin;
use bevy::render::camera::{ScalingMode};

use battleisles_bevy::map_model::MapModel;
mod ui;

pub struct BattleIslesEditor;

impl BattleIslesEditor {
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
            .add_systems(Startup, setup)
            .add_systems(Update, update_camera_to_fit_map.run_if(on_event::<WindowResized>))
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
    ).expect("Failed to create map model");
    commands.insert_resource(map_model);

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0),
    ));
    commands.spawn((
        Camera3d { ..default() },
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            scaling_mode: bevy::render::camera::ScalingMode::Fixed { width: 800.0, height: 600.0 },
            near: -1000.0,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}

fn update_camera_to_fit_map(
    mut query: Query<&mut Projection, With<Camera3d>>,
    window_query: Query<&Window>,
    map_model: Res<MapModel>,
) {
    let window = window_query.single().unwrap();

    let usable_w = window.resolution.physical_width() as f32
        - 100.0
        - 100.0;
    let usable_h = window.resolution.physical_height() as f32
        - 50.0
        - 50.0;

    let aspect_w = usable_w / usable_h;
    let map_width = map_model.map_right - map_model.map_left;
    let map_height = map_model.map_top - map_model.map_bottom;
    let aspect_map = map_width / map_height;

    let mut projection = query.single_mut().unwrap();

    if let Projection::Orthographic(ref mut ortho) = *projection {
        if aspect_w > aspect_map {
            // Window is wider than map → fit height
            let target_h = map_height;
            let target_w = target_h * aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        } else {
            // Window is taller than map → fit width
            let target_w = map_width;
            let target_h = target_w / aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        }
    }
}
