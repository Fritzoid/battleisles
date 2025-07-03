use std::collections::HashMap;
use bevy::prelude::*;
use bevy_color::palettes::basic::*;
use battleisles_domain::hex::Terrain;

#[derive(Resource, Default)]
pub struct TerrainMaterials {
    cache: HashMap<Terrain, Handle<StandardMaterial>>,
}

impl TerrainMaterials {
    pub fn get_or_create(
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
