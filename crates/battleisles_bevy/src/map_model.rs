use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, Projection};
use battleisles_domain::hex_map::HexMap;
use crate::terrain_materials::TerrainMaterials;

#[derive(Resource)]
pub struct MapModel {
    map: HexMap,
    hex_mesh: Handle<Mesh>,
    terrain_materials: TerrainMaterials,
    light: Entity,
    camera: Entity,
}

impl MapModel {
    pub fn try_new(
        map: HexMap,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<Self, bool> {
        let mut terrain_materials = TerrainMaterials::default();
        let hex_mesh = meshes.add(Extrusion::new(
            RegularPolygon::new(map.hex_size(), 6), 0.1,
        ));

        map.hexes.iter().for_each(|hex| {
            let (x, y) = map.hex_to_world_pos(hex);
            let material = 
                terrain_materials.get_or_create(hex.terrain, materials.as_mut());
            commands.spawn((
                Mesh3d(hex_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..default()
                },
            ));
        });

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
                    scaling_mode: bevy::render::camera::ScalingMode::Fixed {
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
        })
    }
}