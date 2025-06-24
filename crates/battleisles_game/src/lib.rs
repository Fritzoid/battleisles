use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_color::palettes::basic::*;
use battleisles_domain::map::Map;
use battleisles_domain::hex::Terrain;

mod ui;
pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Battle Isles".to_owned(),
                    resolution: (800.0, 600.0).into(),
                    resizable: false,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            //.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false, })
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

const HEX_SIZE: f32 = 2.0;
const HEX_THICKNESS: f32 = 0.01;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rows = 3;
    let collumns = 3;   
    let hex_mesh = meshes.add(Extrusion::new(RegularPolygon::new(HEX_SIZE, 6), HEX_THICKNESS));
    let map = Map::try_new(rows, collumns).expect("Failed to create map");
    let mut idx = 0;
    let mut terrain_materials = TerrainMaterials::default();
    let x_increment = 3.0_f32.sqrt() * HEX_SIZE;
    let y_increment = 1.5 * HEX_SIZE;
    let (center_x, center_y) = compute_map_center(rows as u32, collumns as u32, HEX_SIZE);
    
    map.hexes.iter().for_each(|hex| {
        let x = hex.position.1 as f32 * x_increment + 
            if hex.position.0 % 2 == 0 {
                0.0
            } else {
                x_increment / 2.0
            };
        let y = hex.position.0 as f32 * y_increment;
        let z = 0.0;
        let material = terrain_materials.get_or_create(hex.terrain, materials.as_mut());
        commands.spawn((
            Mesh3d(hex_mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform { 
                translation: Vec3::new(x - center_x, y - center_y, z), 
                ..default() 
            },
        ));
        idx += 1;
    });

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
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

use std::collections::HashMap;

#[derive(Resource, Default)]
struct TerrainMaterials {
    cache: HashMap<Terrain, Handle<StandardMaterial>>,
}

impl TerrainMaterials {
    fn get_or_create(
        &mut self,
        terrain: Terrain,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        self.cache.entry(terrain).or_insert_with(|| {
            let color = match terrain {
                Terrain::Plains => GREEN,
                Terrain::Hills => OLIVE,
                Terrain::Mountains => GRAY,
                Terrain::DeepWater => BLUE,
                Terrain::ShallowWater => AQUA,
            };
            materials.add(StandardMaterial::from_color(color))
        }).clone()
    }
}

fn compute_map_center(rows: u32, cols: u32, hex_size: f32) -> (f32, f32) {
    let dx = hex_size * f32::sqrt(3.0);
    let dy = hex_size * 1.5;

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut count = 0.0;

    for y in 0..rows {
        let row_cols = if y % 2 == 0 { cols } else { cols - 1 };
        let x_offset = if y % 2 == 0 { 0.0 } else { dx / 2.0 };

        for x in 0..row_cols {
            let world_x = x as f32 * dx + x_offset;
            let world_y = y as f32 * dy;

            sum_x += world_x;
            sum_y += world_y;
            count += 1.0;
        }
    }

    let center_x = sum_x / count;
    let center_y = sum_y / count;

    (center_x, center_y)
}