use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::pbr::ExtendedMaterial;
use bevy::pbr::MaterialExtension;
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderType;
use bevy::render::render_resource::ShaderRef;
use bevy_egui::EguiPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use map::init_map;

mod center_marker;
mod env;
mod map;
mod ui;

use center_marker::center_marker;
use env::init_env;
use map::{HexType, MapInfo};

const SHADER_ASSET_PATH: &str = "shaders/water_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct Water {
    /// The normal map image.
    ///
    /// Note that, like all normal maps, this must not be loaded as sRGB.
    #[texture(100)]
    #[sampler(101)]
    normals: Handle<Image>,

    // Parameters to the water shader.
    #[uniform(102)]
    settings: WaterSettings,
}

/// Parameters to the water shader.
#[derive(ShaderType, Debug, Clone)]
pub struct WaterSettings {
    /// How much to displace each octave each frame, in the u and v directions.
    /// Two octaves are packed into each `vec4`.
    octave_vectors: [Vec4; 2],
    /// How wide the waves are in each octave.
    octave_scales: Vec4,
    /// How high the waves are in each octave.
    octave_strengths: Vec4,
}

impl MaterialExtension for Water {
    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .insert_resource(Msaa::Off)
            .insert_resource(DefaultOpaqueRendererMethod::deferred())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin)
            .add_plugins(CursorRayPlugin)
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, Water>>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, Water>>>,
    asset_server: Res<AssetServer>,

) {
    let map = MapInfo {
        width: 5,
        height: 3,
        hexes: vec![
            HexType::DeepWater,
            HexType::Plains,
            HexType::ShallowWater,
            HexType::Mountains,
            HexType::Hills,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::DeepWater,
            HexType::Hills,
            HexType::ShallowWater,
            HexType::ShallowWater,
            HexType::Plains,
            HexType::Plains,
            HexType::Mountains,
        ],
    };

    center_marker(&mut commands, &mut meshes, &mut materials);
    init_map(map, &mut meshes, &mut commands, &mut materials, &mut water_materials, &asset_server);
    init_env(&mut commands, &asset_server);
}
