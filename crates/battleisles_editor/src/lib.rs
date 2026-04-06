use battleisles_bevy::camera::{CameraZoom, CameraZoomPlugin};
use battleisles_bevy::map_model::MapModel;
use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_camera::{OrthographicProjection, Projection, ScalingMode};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod ui;

const MAP_FIT_PADDING: f32 = 1.05;

#[derive(Message, Event)]
pub struct GenerateMapEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Message, Event)]
pub struct MapChangedEvent;

pub struct BattleIslesEditor;

impl BattleIslesEditor {
    pub fn run() {
        App::new()
            .init_resource::<ui::UiState>()
            .add_message::<GenerateMapEvent>()
            .add_message::<MapChangedEvent>()
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
            .add_plugins(MapModelPlugin)
            .add_plugins(CameraZoomPlugin)
            .add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(
                EguiPrimaryContextPass,
                (ui::ui_system, ui::paint_click_system),
            )
            .add_systems(
                Update,
                (
                    handle_generate_map_event,
                    handle_map_changed_event,
                    fit_map_to_viewport,
                ),
            )
            .run();
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ui::UiState::default());
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
}

fn fit_map_to_viewport(
    map_model: Option<Res<MapModel>>,
    windows: Query<&Window>,
    mut zoom: ResMut<CameraZoom>,
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
        let base =
            (map_size.x / window.width()).max(map_size.y / window.height()) * MAP_FIT_PADDING;
        zoom.base_scale = base;
        *scale = base * zoom.factor;
    }
}

fn handle_generate_map_event(
    mut events: MessageReader<GenerateMapEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_changed_events: MessageWriter<MapChangedEvent>,
) {
    for event in events.read() {
        println!(
            "Generating map with dimensions: {}x{}",
            event.width, event.height
        );

        let map = Map::new(event.width, event.height);

        match MapModelPlugin::initialize_map_model(map, &mut commands, &mut meshes, &mut materials)
        {
            Ok(_) => {
                println!("Map generated successfully");
                map_changed_events.write(MapChangedEvent);
            }
            Err(e) => println!("Failed to generate map: {:?}", e),
        }
    }
}

fn handle_map_changed_event(
    mut events: MessageReader<MapChangedEvent>,
    mut zoom: ResMut<CameraZoom>,
) {
    for _event in events.read() {
        println!("Map changed, resetting zoom");
        zoom.factor = 1.0;
    }
}
