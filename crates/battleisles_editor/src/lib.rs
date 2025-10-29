use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResized};
use bevy_camera::{OrthographicProjection, Projection, ScalingMode};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod ui;

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
            .add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(
                EguiPrimaryContextPass,
                (ui::ui_system, ui::paint_click_system),
            )
            .add_systems(
                Update,
                (handle_generate_map_event, handle_map_changed_event),
            )
            .run();
    }
}

pub fn setup(mut commands: Commands) {
    // Initialize any resources or entities needed for the editor
    commands.insert_resource(ui::UiState::default());
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
                // Send MapChangedEvent to trigger camera update
                map_changed_events.write(MapChangedEvent);
            }
            Err(e) => println!("Failed to generate map: {:?}", e),
        }
    }
}

fn handle_map_changed_event(
    mut events: MessageReader<MapChangedEvent>,
    mut window_events: MessageWriter<WindowResized>,
    window_query: Query<(Entity, &Window)>,
) {
    for _event in events.read() {
        println!("Map changed, triggering camera update");
        // Send a fake WindowResized event to trigger the camera update
        if let Ok((entity, window)) = window_query.single() {
            window_events.write(WindowResized {
                window: entity,
                width: window.resolution.physical_width() as f32,
                height: window.resolution.physical_height() as f32,
            });
        }
    }
}
