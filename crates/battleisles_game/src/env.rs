use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::core_pipeline::Skybox;
use bevy::pbr::ScreenSpaceReflectionsBundle;
use bevy::pbr::ScreenSpaceReflectionsSettings;

#[derive(Component)]
struct GameCamera;

pub fn init_env(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(20.0, 20.0, 30.0)) // Set initial position
                .looking_at(Vec3::ZERO, Vec3::Y), // Make the camera look at the origin
            ..default()
        },
        GameCamera,
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.0, 0.0),
            radius: Some(50.0),
            pitch_lower_limit: Some(0.1),
            pitch_upper_limit: Some(std::f32::consts::FRAC_PI_2),
            ..default()
        },
    ))
/* 
    .insert(EnvironmentMapLight {
        diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        intensity: 5000.0,
    })
     .insert(Skybox {
        image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        brightness: 5000.0,
    })
 */
    .insert(ScreenSpaceReflectionsBundle::default())
    .insert(ScreenSpaceReflectionsSettings::default())
    .insert(Fxaa::default());
}
