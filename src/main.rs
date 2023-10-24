use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow};
use hexx::*;
mod hexx_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_battlemap))
        .add_systems(Update, handle_input)
        .run()
}

fn setup_camera(mut commands: Commands) {
    let cam = 
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0.,300.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        };

    commands.spawn(cam);
}

#[derive(Debug, Resource)]
struct BattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    default_material: Handle<StandardMaterial>,
    selected_material: Handle<StandardMaterial>,
}
 
fn setup_battlemap(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let layout = HexLayout {
        hex_size: Vec2::splat(10.0),
        ..Default::default()
    };

    let default_material = 
        materials.add(StandardMaterial {
            base_color: Color::hex("#0000FF").unwrap(), 
            ..Default::default()
        });
    let selected_material = 
        materials.add(StandardMaterial {
            base_color: Color::hex("#FF0000").unwrap(), 
            ..Default::default()
        });

    let hexx_mesh: Mesh = hexx_mesh::hexx_mesh(&layout);
    let mesh = meshes.add(hexx_mesh);

    let entities = shapes::flat_rectangle([-5, 5, -5, 5])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(0.99)),
                    mesh: mesh.clone().into(),
                    material: default_material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(BattleMap {
        layout,
        entities,
        default_material, 
        selected_material,
    });
}

fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    map: Res<BattleMap>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
//        .and_then(|p| camera.viewport_to_world(cam_transform, p))
    {
/*
        let coord = map.layout.world_pos_to_hex(pos.direction.);
        if let Some(entity) = map.entities.get(&coord).copied() {
            commands
                .entity(entity)
                .insert(map.selected_material.clone());
        }
 */ 
   }
}