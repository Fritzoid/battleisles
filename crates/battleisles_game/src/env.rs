use bevy::prelude::*;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::pbr::ScreenSpaceReflections;
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::PI;

#[derive(Component)]
struct GameCamera;

pub fn init_env(commands: &mut Commands, asset_server: &Res<AssetServer>) {

    commands.spawn((
        Camera3d::default(), 
        PanOrbitCamera { 
            pitch_lower_limit: Some(PI/6.0),
            pitch_upper_limit: Some(PI/4.0),
            zoom_lower_limit: 10.0,
            zoom_upper_limit: Some(500.0),
            ..default() 
        },
        Transform {
            translation: Vec3::new(350.0, 180.0, 350.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }.looking_at(Vec3::ZERO, -Vec3::Z),
        GameCamera,
        ScreenSpaceReflections::default(),
    ))
    .insert(EnvironmentMapLight {
        diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        intensity: 5000.0,
        ..default()
    })
    .insert(Fxaa::default());
}
