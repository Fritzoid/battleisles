use battleisles_domain::map::Map;
use bevy::prelude::*;

pub struct MapModelPlugin;
use crate::map_model::MapModel;

impl Plugin for MapModelPlugin {
    fn build(&self, app: &mut App) {}
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
