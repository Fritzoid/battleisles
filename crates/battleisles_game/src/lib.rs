use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::EguiPlugin;
use bevy_color::palettes::basic::*;
use battleisles_domain::map::Map;
use battleisles_domain::hex::Terrain;

mod ui;

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "Battle Isles".to_owned(),
                    resolution: (800.0, 600.0).into(),
                    resizable: false,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }))
            //.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false, })
            .add_systems(Startup, setup)
            .add_systems(Update, ui::ui_system)
            .run();
    }
}

const HEX_SIZE: f32 = 2.0;
const HEX_THICKNESS: f32 = 0.01;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hex_mesh = meshes.add(Extrusion::new(RegularPolygon::new(HEX_SIZE, 6), HEX_THICKNESS));
    let map = Map::try_new(3, 3).expect("Failed to create map");
    let mut idx = 0;
    let mut terrain_materials = TerrainMaterials::default();
    
    map.hexes.iter().for_each(|hex| {
        let x = hex.position.1 as f32 * (3.0_f32.sqrt() * HEX_SIZE) + 
            if hex.position.0 % 2 == 0 {
                0.0
            } else {
                (3.0_f32.sqrt() * HEX_SIZE) / 2.0
            };
        let y = hex.position.0 as f32 * (1.5 * HEX_SIZE);
        let z = 0.0;
        let material = terrain_materials.get_or_create(hex.terrain, materials.as_mut());
        commands.spawn((
            Mesh3d(hex_mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform { 
                translation: Vec3::new(x, y, z), 
                ..default() 
            },
        ));
        idx += 1;
    });

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

use std::collections::HashMap;

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
        self.cache.entry(terrain).or_insert_with(|| {
            let color = match terrain {
                Terrain::Plains => GREEN,
                Terrain::Hills => OLIVE,
                Terrain::Mountains => GRAY,
                Terrain::DeepWater => BLUE,
                Terrain::ShallowWater => AQUA,
            };
            materials.add(StandardMaterial::from_color(color))
        }).clone()
    }
}