use std::collections::HashMap;
use bevy::prelude::*; 
use bevy::window::WindowResolution;
use bevy::window::PrimaryWindow;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use hexx::*;

mod battle_map;
use battle_map::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run()
}


#[derive(Resource)]
struct BevyBattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
}

#[derive(Component)]
struct GameCamera;

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout { hex_size: Vec2::splat(1.0),  ..default()};
    let material = materials.add(Color::BLUE.into());
    let hexx_mesh: Mesh = hexx_mesh(&layout);
    let mesh = meshes.add(hexx_mesh);

    commands.spawn((
        Camera3dBundle::default(),
        GameCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.0, 0.0),
            alpha: Some(1.5),
            beta: Some(1.5),
            radius: Some(5.0),
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            ..default()
        },
    ));

    let entities = shapes::flat_rectangle([-1, 1, -1, 1])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, -pos.y),
                    mesh: mesh.clone().into(),
                    material: material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(BevyBattleMap {
        layout,
        entities,
    });
}

fn hexx_mesh(hex_layout: &HexLayout) -> Mesh {
    let mesh_info: MeshInfo = ColumnMeshBuilder::new(hex_layout, 0.2).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

fn handle_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    map: Res<BevyBattleMap>,
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
                dbg!(entity);
            }
        }
    }
}