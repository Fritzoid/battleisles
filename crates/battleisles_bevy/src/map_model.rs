use crate::terrain_materials::TerrainMaterials;
use battleisles_domain::map::{Map, Terrain};
use bevy::prelude::*;
use std::collections::HashSet;

const EDGE_KEY_SCALE: f32 = 10_000.0;
const OUTLINE_Z: f32 = 0.06;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PointKey {
    x: i32,
    y: i32,
}

impl PointKey {
    fn from_vec2(point: Vec2) -> Self {
        Self {
            x: (point.x * EDGE_KEY_SCALE).round() as i32,
            y: (point.y * EDGE_KEY_SCALE).round() as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct EdgeKey {
    start: PointKey,
    end: PointKey,
}

impl EdgeKey {
    fn new(start: Vec2, end: Vec2) -> Self {
        let start = PointKey::from_vec2(start);
        let end = PointKey::from_vec2(end);
        if start <= end {
            Self { start, end }
        } else {
            Self {
                start: end,
                end: start,
            }
        }
    }
}

#[derive(Resource)]
pub struct MapModel {
    map: Map,
    map_size: Vec2,
    outline_segments: Vec<[Vec2; 2]>,
    terrain_materials: TerrainMaterials,
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
        let fill_mesh = meshes.add(Extrusion::new(RegularPolygon::new(map.hex_size(), 6), 0.1));

        let (center, map_size) = Self::layout_metrics(&map);
        let outline_segments = Self::build_outline_segments(&map, center);

        let mut tile_entities = Vec::with_capacity(map.tiles.len());
        for tile in &map.tiles {
            let (x_raw, y_raw) = map.tile_to_world_pos(tile);
            let x = x_raw - center.0;
            let y = -y_raw - center.1; // flip Y and center

            let material = terrain_materials.get_or_create(tile.terrain, materials.as_mut());
            let entity = commands
                .spawn((
                    Mesh3d(fill_mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform {
                        translation: Vec3::new(x, y, 0.0),
                        ..default()
                    },
                ))
                .id();
            tile_entities.push(entity);
        }

        commands.spawn((
            PointLight {
                shadow_maps_enabled: true,
                intensity: 10_000_000.,
                range: 100.0,
                shadow_depth_bias: 0.2,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 50.0),
        ));

        Ok(MapModel {
            map,
            map_size,
            outline_segments,
            terrain_materials,
            tile_entities,
        })
    }

    pub fn map_size(&self) -> Vec2 {
        self.map_size
    }

    pub fn outline_segments(&self) -> &[[Vec2; 2]] {
        &self.outline_segments
    }

    pub fn outline_z() -> f32 {
        OUTLINE_Z
    }

    pub fn outline_color() -> Color {
        Color::srgb_u8(191, 163, 92)
    }

    fn layout_metrics(map: &Map) -> ((f32, f32), Vec2) {
        if let Some((min_x, max_x, min_y, max_y)) = Self::map_bounds(map) {
            let center = ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);
            let size = Vec2::new(max_x - min_x, max_y - min_y);
            (center, size)
        } else {
            ((0.0, 0.0), Vec2::ZERO)
        }
    }

    fn map_bounds(map: &Map) -> Option<(f32, f32, f32, f32)> {
        if map.tiles.is_empty() {
            return None;
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for tile in &map.tiles {
            for (x_raw, y_raw) in map.tile_to_world_corners(tile) {
                let x = x_raw;
                let y = -y_raw;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }

        Some((min_x, max_x, min_y, max_y))
    }

    fn build_outline_segments(map: &Map, center: (f32, f32)) -> Vec<[Vec2; 2]> {
        let mut seen = HashSet::with_capacity(map.tiles.len() * 3);
        let mut segments = Vec::with_capacity(map.tiles.len() * 3);

        for tile in &map.tiles {
            if tile.terrain != Terrain::Unassigned {
                continue;
            }

            let corners = map
                .tile_to_world_corners(tile)
                .map(|(x_raw, y_raw)| Vec2::new(x_raw - center.0, -y_raw - center.1));

            for edge_idx in 0..corners.len() {
                let start = corners[edge_idx];
                let end = corners[(edge_idx + 1) % corners.len()];
                if seen.insert(EdgeKey::new(start, end)) {
                    segments.push([start, end]);
                }
            }
        }

        segments
    }

    fn rebuild_outlines(&mut self) {
        let center = self.center_from_bounds();
        self.outline_segments = Self::build_outline_segments(&self.map, center);
    }

    fn center_from_bounds(&self) -> (f32, f32) {
        if let Some((min_x, max_x, min_y, max_y)) = Self::map_bounds(&self.map) {
            ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5)
        } else {
            (0.0, 0.0)
        }
    }

    fn tile_world_centered(&self, index: usize) -> Vec2 {
        let (cx, cy) = self.center_from_bounds();
        let (x_raw, y_raw) = self.map.tile_to_world_pos(&self.map.tiles[index]);
        Vec2::new(x_raw - cx, -y_raw - cy)
    }

    pub(crate) fn find_nearest_tile(&self, world_pos: Vec2) -> Option<usize> {
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
        best_i
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
            self.rebuild_outlines();
        }
    }
}
