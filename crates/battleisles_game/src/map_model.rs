use std::collections::HashMap;
use bevy_color::palettes::basic::*;
use bevy::prelude::*;
use battleisles_domain::map::Map;
use battleisles_domain::hex::Terrain;

#[derive(Resource)]
pub struct MapModel {
    map: Map,
    hex_mesh: Handle<Mesh>,
    terrain_materials: TerrainMaterials,
    hex_size: f32,
    hex_thickness: f32,
}

impl MapModel {
    pub fn try_new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let rows = 3;
        let collumns = 3;
        let hex_size = 2.0;
        let hex_thickness = 0.01;
        let hex_mesh = meshes.add(Extrusion::new(RegularPolygon::new(hex_size, 6), hex_thickness));
        let map = Map::try_new(rows, collumns).expect("Failed to create map");
        let mut idx = 0;
        let mut terrain_materials = TerrainMaterials::default();
        let x_increment = 3.0_f32.sqrt() * hex_size;
        let y_increment = 1.5 * hex_size;
        let (center_x, center_y) = compute_map_center(rows as u32, collumns as u32, hex_size);
        
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


        MapModel {
            map,
            hex_mesh,
            terrain_materials,
            hex_size,
            hex_thickness,
        }
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