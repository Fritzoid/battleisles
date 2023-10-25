use std::collections::HashMap;

use bevy::{prelude::*, window::PrimaryWindow, input::mouse::MouseWheel};
use hexx::*;
mod hexx_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_battlemap))
        .add_systems(Update, handle_input)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        global_transform: GlobalTransform::from_xyz(0., 0., 3000.),
        ..Default::default()
    });
}

#[derive(Debug, Resource)]
struct BattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    default_material: Handle<ColorMaterial>,
    selected_material: Handle<ColorMaterial>,
}
 
fn setup_battlemap(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let layout = HexLayout {
        hex_size: Vec2::splat(20.0),
        ..Default::default()
    };

    let default_material = 
        materials.add(Color::BLUE.into());
    let selected_material = 
        materials.add(Color::RED.into());

    let hexx_mesh: Mesh = hexx_mesh::hexx_mesh(&layout);
    let mesh = meshes.add(hexx_mesh);

    let entities = shapes::flat_rectangle([-5, 5, -5, 5])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(ColorMesh2dBundle {
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
    buttons: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    map: Res<BattleMap>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        if buttons.just_pressed(MouseButton::Left) {
            let coord = map.layout.world_pos_to_hex(pos);
            if let Some(entity) = map.entities.get(&coord).copied() {
                commands
                    .entity(entity)
                    .insert(map.selected_material.clone());
            }
            let x = coord.x;
            let y = coord.y;
//            println!("{x},{y}");  
        }

        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
        }
//        println!("{pos}");
    }
}