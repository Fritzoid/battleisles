use battleisles_domain::map::Terrain;
use bevy::prelude::*;
use bevy_color::palettes::basic::*;
use std::collections::HashMap;

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
            .or_insert_with(|| materials.add(Self::terrain_material(terrain)))
            .clone()
    }

    fn terrain_material(terrain: Terrain) -> StandardMaterial {
        match terrain {
            Terrain::Unassigned => StandardMaterial {
                base_color: Color::srgba_u8(0, 0, 0, 0),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                perceptual_roughness: 1.0,
                ..default()
            },
            Terrain::Plains => Self::flat_material(GREEN),
            Terrain::Hills => Self::flat_material(OLIVE),
            Terrain::Mountains => Self::flat_material(GRAY),
            Terrain::DeepWater => Self::flat_material(BLUE),
            Terrain::ShallowWater => Self::flat_material(AQUA),
        }
    }

    fn flat_material(color: impl Into<Color>) -> StandardMaterial {
        StandardMaterial {
            base_color: color.into(),
            unlit: true,
            perceptual_roughness: 1.0,
            ..default()
        }
    }
}
