use bevy::prelude::*;
use battleisles_domain::hex_map::HexMap;

pub struct MapModelPlugin;
use crate::map_model::MapModel;

impl Plugin for MapModelPlugin {
    fn build(&self, app: &mut App) {
    }
}

impl MapModelPlugin {
    pub fn initialize_map(
        map: HexMap, 
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<(), bool> {
        let map_model = MapModel::try_new(map, commands, meshes, materials).unwrap();
        commands.insert_resource(map_model);
        Ok(())
    }
}
