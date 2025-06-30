use battleisles_domain::hex::Terrain;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy_color::palettes::basic::*;
use std::collections::HashMap;
use bevy::render::camera::{ScalingMode};


#[derive(Resource)]
pub struct MapModel {
    map: Map,
    pub map_right: f32,
    pub map_left: f32,
    pub map_top: f32,
    pub map_bottom: f32,
    hex_mesh: Handle<Mesh>,
    terrain_materials: TerrainMaterials,
    hex_size: f32,
    hex_thickness: f32,
    light: Entity,
    camera: Entity,
}

impl MapModel {
    pub fn try_new(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<Self, bool> {
        let hex_size = 2.0;
        let hex_thickness = 0.01;
        let map = Map::default();
        let mut terrain_materials = TerrainMaterials::default();
        let hex_mesh = meshes.add(Extrusion::new(
            RegularPolygon::new(hex_size, 6),
            hex_thickness,
        ));
        let x_increment = 3.0_f32.sqrt() * hex_size;
        let y_increment = 1.5 * hex_size;
        let (center_x, center_y, map_top, map_bottom, map_left, map_right) =
            compute_map_specs(map.rows as u32, map.collumns as u32, hex_size);

        map.hexes.iter().for_each(|hex| {
            let x = hex.position.1 as f32 * x_increment
                + if hex.position.0 % 2 == 0 {
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
        });

        let light_entity = commands
            .spawn((
                PointLight {
                    shadows_enabled: true,
                    intensity: 10_000_000.,
                    range: 100.0,
                    shadow_depth_bias: 0.2,
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 50.0),
            ))
            .id();

        let camera_id = commands
            .spawn((
                Camera3d { ..default() },
                Projection::Orthographic(OrthographicProjection {
                    scale: 1.0,
                    scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                        width: 800.0,
                        height: 600.0,
                    },
                    near: -1000.0,
                    far: 1000.0,
                    ..OrthographicProjection::default_3d()
                }),
                Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
                GlobalTransform::default(),
            ))
            .id();

        Ok(MapModel {
            map,
            map_right,
            map_left,
            map_top,
            map_bottom,
            hex_mesh,
            terrain_materials,
            hex_size,
            hex_thickness,
            light: light_entity,
            camera: camera_id,
        })
    }
}

fn compute_map_specs(rows: u32, cols: u32, hex_size: f32) -> (f32, f32, f32, f32, f32, f32) {
    let dx = hex_size * f32::sqrt(3.0); // horizontal distance between columns
    let dy = hex_size * 1.5; // vertical distance between rows

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

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

            min_x = min_x.min(world_x);
            max_x = max_x.max(world_x);
            min_y = min_y.min(world_y);
            max_y = max_y.max(world_y);
        }
    }

    let center_x = sum_x / count;
    let center_y = sum_y / count;

    // True bounding box (from outer hex edges)
    let left = min_x - dx / 2.0;
    let right = max_x + dx / 2.0;
    let top = max_y + hex_size;
    let bottom = min_y - hex_size;

    (center_x, center_y, top, bottom, left, right)
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
        self.cache
            .entry(terrain)
            .or_insert_with(|| {
                let color = match terrain {
                    Terrain::Plains => GREEN,
                    Terrain::Hills => OLIVE,
                    Terrain::Mountains => GRAY,
                    Terrain::DeepWater => BLUE,
                    Terrain::ShallowWater => AQUA,
                };
                materials.add(StandardMaterial::from_color(color))
            })
            .clone()
    }
}

pub fn update_camera_to_fit_map(
    mut query: Query<&mut Projection, With<Camera3d>>,
    window_query: Query<&Window>,
    map_model: Res<MapModel>,
) {
    let window = window_query.single().unwrap();
    let usable_w = window.resolution.physical_width() as f32 - 100.0 - 100.0;
    let usable_h = window.resolution.physical_height() as f32 - 50.0 - 50.0;
    let aspect_w = usable_w / usable_h;
    let map_width = map_model.map_right - map_model.map_left;
    let map_height = map_model.map_top - map_model.map_bottom;
    let aspect_map = map_width / map_height;
    let mut projection = query.single_mut().unwrap();

    if let Projection::Orthographic(ref mut ortho) = *projection {
        if aspect_w > aspect_map {
            // Window is wider than map → fit height
            let target_h = map_height;
            let target_w = target_h * aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        } else {
            // Window is taller than map → fit width
            let target_w = map_width;
            let target_h = target_w / aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        }
    }
}
