use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::{collections::HashMap};

mod map;
mod ui;
mod center_marker;

use center_marker::center_marker;

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin)
            .add_plugins(CursorRayPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

#[derive(Component)]
struct GameCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = RegularPolygon::new(5.0, 6);
    let mesh = Extrusion::new(shape, 0.01);
    let mesh_handle = meshes.add(mesh);

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: materials.add(Color::WHITE),
        transform: Transform { 
            translation: Vec3::new(x, y, z),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2/3.0),
            ..default()
        },
        ..default()
    });

    x += 3.0/2.0 * 5.0;
    z += (3.0f32).sqrt()/2.0 * 5.0;

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: materials.add(StandardMaterial::from_color(bevy::color::palettes::css::BLUE)),
        transform: Transform { 
            translation: Vec3::new(x, y, z),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2/3.0),
            ..default()
        },
        ..default()
    });

    x += 3.0/2.0 * 5.0;
    z += -((3.0f32).sqrt()/2.0 * 5.0);

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: materials.add(StandardMaterial::from_color(bevy::color::palettes::css::RED)),
        transform: Transform { 
            translation: Vec3::new(x, y, z),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2/3.0),
            ..default()
        },
        ..default()
    });

    center_marker(&mut commands, &mut meshes, &mut materials);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 20.0)) // Set initial position
                .looking_at(Vec3::ZERO, Vec3::Y), // Make the camera look at the origin
            ..default()
        },
        GameCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.0, 0.0),
            radius: Some(10.0),
            pitch_lower_limit: Some(0.1),
            pitch_upper_limit: Some(std::f32::consts::FRAC_PI_2),
            ..default()
        },
    ));

}

/*

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mapstr = r#"{
        "size":[3,3],
        "hexes":[
            {"hex_type":"DeepWater"},
            {"hex_type":"Plains"},
            {"hex_type":"ShallowWater"},
            {"hex_type":"Mountains"}, 
            {"hex_type":"Hills"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}
        ]
    }"#;

    let battle_map = map::Map::from_json(mapstr);
    let layout = HexLayout {
        hex_size: Vec2::splat(1.0),
        ..default()
    };
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let right: i32 = battle_map.size.0 as i32 / 2;
    let left = -right;
    let bottom: i32 = battle_map.size.1 as i32 / 2;
    let top = -bottom;

    let entities = shapes::flat_rectangle([left, right, top, bottom])
        .zip(battle_map.hexes.iter())
        .map(|(hex, battle_hex)| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, -pos.y),
                    mesh: mesh_handle.clone().into(),
                    material: match battle_hex.hex_type {
                        map::HexType::DeepWater => materials.add(
                            StandardMaterial::from_color(bevy::color::palettes::css::DARK_BLUE),
                        ),
                        map::HexType::ShallowWater => materials.add(
                            StandardMaterial::from_color(bevy::color::palettes::css::BLUE),
                        ),
                        map::HexType::Plains => materials.add(StandardMaterial::from_color(
                            bevy::color::palettes::css::GREEN,
                        )),
                        map::HexType::Mountains => materials.add(
                            StandardMaterial::from_color(bevy::color::palettes::css::GRAY),
                        ),
                        map::HexType::Hills => materials.add(StandardMaterial::from_color(
                            bevy::color::palettes::css::YELLOW,
                        )),
                    },
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(BevyBattleMap { layout, entities });
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, 0.2)
        .with_scale(Vec3::splat(0.95))
        .build();

    let num_vertices = mesh_info.vertices.len();    
    dbg!("Num vertices: {}", num_vertices);
    dbg!("Vertices: {:?}", &mesh_info.vertices);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn raycast(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mut gizmos: Gizmos,
) {
    if let Some(cursor_ray) = **cursor_ray {
        let hits = raycast.debug_cast_ray(cursor_ray, &default(), &mut gizmos);
        if let Some(hit) = hits.first() {
            let new_material = materials.add(StandardMaterial::from_color(
                bevy::color::palettes::css::PINK,
            ));
            commands.entity(hit.0).insert(new_material);
        }
    }
}
*/
