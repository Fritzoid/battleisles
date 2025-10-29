use crate::terrain_materials::TerrainMaterials;
use battleisles_domain::map::Map;
use bevy::prelude::*;
use bevy_camera::{OrthographicProjection, Projection, ScalingMode};

#[derive(Component, Clone, Copy)]
pub struct TileIndex(pub usize);

#[derive(Resource)]
pub struct MapModel {
    map: Map,
    hex_mesh: Handle<Mesh>,
    terrain_materials: TerrainMaterials,
    light: Entity,
    camera: Entity,
    tile_entities: Vec<Entity>,
}

impl MapModel {
    pub fn try_new(
        map: Map,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<Self, bool> {
        let mut terrain_materials = TerrainMaterials::default();
        let hex_mesh = meshes.add(Extrusion::new(RegularPolygon::new(map.hex_size(), 6), 0.1));

        // Compute bounds to center the map and flip Y so row 0 is at the top
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for tile in &map.tiles {
            let (x_raw, y_raw) = map.tile_to_world_pos(tile);
            let x = x_raw;
            let y = -y_raw; // flip Y so r=0 is at the top
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        let center = ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);

        let mut tile_entities = Vec::with_capacity(map.tiles.len());
        for (i, tile) in map.tiles.iter().enumerate() {
            let (x_raw, y_raw) = map.tile_to_world_pos(tile);
            let x = x_raw - center.0;
            let y = -y_raw - center.1; // flip Y and center
            let material = terrain_materials.get_or_create(tile.terrain, materials.as_mut());
            let entity = commands
                .spawn((
                    Mesh3d(hex_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..default()
                    },
                    TileIndex(i),
                ))
                .id();
            tile_entities.push(entity);
        }

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
            ))
            .id();

        Ok(MapModel {
            map,
            hex_mesh,
            terrain_materials,
            light: light_entity,
            camera: camera_id,
            tile_entities,
        })
    }

    fn center_from_bounds(&self) -> (f32, f32) {
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for t in &self.map.tiles {
            let (x, y_raw) = self.map.tile_to_world_pos(t);
            let y = -y_raw;
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }
        ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5)
    }

    fn tile_world_centered(&self, index: usize) -> Vec2 {
        let (cx, cy) = self.center_from_bounds();
        let (x_raw, y_raw) = self.map.tile_to_world_pos(&self.map.tiles[index]);
        Vec2::new(x_raw - cx, -y_raw - cy)
    }

    pub(crate) fn find_nearest_tile_entity(&self, world_pos: Vec2) -> Option<(usize, Entity)> {
        let mut best_i = None;
        let mut best_d2 = f32::INFINITY;
        for i in 0..self.map.tiles.len() {
            let p = self.tile_world_centered(i);
            let d2 = p.distance_squared(world_pos);
            if d2 < best_d2 {
                best_d2 = d2;
                best_i = Some(i);
            }
        }
        best_i.map(|i| (i, self.tile_entities[i]))
    }

    pub(crate) fn set_tile_terrain(
        &mut self,
        index: usize,
        terrain: battleisles_domain::map::Terrain,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        commands: &mut Commands,
    ) {
        if let Some(tile) = self.map.tiles.get_mut(index) {
            tile.terrain = terrain;
            let entity = self.tile_entities[index];
            let handle = self.terrain_materials.get_or_create(terrain, materials);
            commands.entity(entity).insert(MeshMaterial3d(handle));
        }
    }
}
