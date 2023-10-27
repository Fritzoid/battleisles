use std::collections::HashMap;
use bevy::prelude::*; 
use bevy::window::WindowResolution;
use bevy::window::PrimaryWindow;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use hexx::*;

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
struct BattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    default_material: Handle<StandardMaterial>,
    selected_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct MyGameCamera;

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout { hex_size: Vec2::splat(20.0),  ..default()};
    let default_material = materials.add(Color::BLUE.into());
    let selected_material = materials.add(Color::RED.into());
    let hexx_mesh: Mesh = hexx_plane(&layout);
    let mesh = meshes.add(hexx_mesh);

    commands.spawn((
        Camera3dBundle::default(),
        MyGameCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.0, 0.0),
            alpha: Some(0.),
            beta: Some(0.),
            radius: Some(200.0),
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            ..default()
        },
    ));

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

fn hexx_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info: MeshInfo = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
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
            for e in map.entities.iter()
            {
                commands
                    .entity(*e.1)
                    .insert(map.default_material.clone());
            }

            if let Some(entity) = map.entities.get(&coord).copied() {
                commands
                    .entity(entity)
                    .insert(map.selected_material.clone());
            }
        }
    }
}