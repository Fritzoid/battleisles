use bevy::prelude::*;
use bevy::pbr::MaterialExtension;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderType;
use bevy::render::render_resource::ShaderRef;

const SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Water {
    /// The normal map image.
    ///
    /// Note that, like all normal maps, this must not be loaded as sRGB.
    #[texture(100)]
    #[sampler(101)]
    pub normals: Handle<Image>,

    // Parameters to the water shader.
    #[uniform(102)]
    pub settings: WaterSettings,
}

/// Parameters to the water shader.
#[derive(ShaderType, Debug, Clone)]
pub struct WaterSettings {
    /// How much to displace each octave each frame, in the u and v directions.
    /// Two octaves are packed into each `vec4`.
    pub octave_vectors: [Vec4; 2],
    /// How wide the waves are in each octave.
    pub octave_scales: Vec4,
    /// How high the waves are in each octave.
    pub octave_strengths: Vec4,
}

impl MaterialExtension for Water {
    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
