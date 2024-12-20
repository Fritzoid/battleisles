use bevy::prelude::*;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::pbr::ScreenSpaceReflections;
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Component)]
struct GameCamera;

pub fn init_env(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(), 
        PanOrbitCamera::default(),
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
