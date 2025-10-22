use battleisles_domain::map::Map;
use bevy::prelude::*;

pub struct MapModelPlugin;
use crate::map_model::MapModel;

impl Plugin for MapModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ApplyTerrainAt>()
            .add_systems(Update, handle_apply_terrain_at);
    }
}

impl MapModelPlugin {
    pub fn initialize_map_model(
        map: Map,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<(), bool> {
        let map_model = MapModel::try_new(map, commands, meshes, materials).unwrap();
        commands.insert_resource(map_model);
        Ok(())
    }
}

// Event sent by the editor when the user clicks in the viewport to paint a terrain
#[derive(Message, Event, Clone, Copy, Debug)]
pub struct ApplyTerrainAt {
    pub world_pos: Vec2,    // world coords in the main XY plane (already centered)
    pub terrain: battleisles_domain::map::Terrain,
}

fn handle_apply_terrain_at(
    mut ev: MessageReader<ApplyTerrainAt>,
    map_model: Option<ResMut<MapModel>>, // may not exist until initialize_map_model runs
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let Some(mut map_model) = map_model else { return; };
    for ApplyTerrainAt { world_pos, terrain } in ev.read().copied() {
        if let Some((idx, _entity)) = map_model.find_nearest_tile_entity(world_pos) {
            map_model.set_tile_terrain(idx, terrain, &mut materials, &mut commands);
        }
    }
}
