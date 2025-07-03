use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResized};
use bevy_egui::EguiPlugin;
use battleisles_bevy::map_model_plugin::MapModelPlugin;
use battleisles_domain::map::Map;

mod ui;

#[derive(Event)]
pub struct GenerateMapEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Event)]
pub struct MapChangedEvent;

pub struct BattleIslesEditor;

impl BattleIslesEditor {
    pub fn run() {
        App::new()
            .init_resource::<ui::UiState>()
            .add_event::<GenerateMapEvent>()
            .add_event::<MapChangedEvent>()
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
            .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: false,
            })
            .add_plugins(MapModelPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (ui::ui_system, handle_generate_map_event, handle_map_changed_event))
            .run();
    }
}

pub fn setup(
    mut commands: Commands,
) {
    // Initialize any resources or entities needed for the editor
    commands.insert_resource(ui::UiState::default());
}

fn handle_generate_map_event(
    mut events: EventReader<GenerateMapEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_changed_events: EventWriter<MapChangedEvent>,
) {
    for event in events.read() {
        println!("Generating map with dimensions: {}x{}", event.width, event.height);
        
        let map = Map::try_new(event.height as usize, event.width as usize).unwrap();
        
        // Call the MapModelPlugin's initialize_map method
        match MapModelPlugin::initialize_map(
            map,
            &mut commands,
            &mut meshes,
            &mut materials,
        ) {
            Ok(_) => {
                println!("Map generated successfully");
                // Send MapChangedEvent to trigger camera update
                map_changed_events.write(MapChangedEvent);
            },
            Err(e) => println!("Failed to generate map: {:?}", e),
        }
    }
}

fn handle_map_changed_event(
    mut events: EventReader<MapChangedEvent>,
    mut window_events: EventWriter<WindowResized>,
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