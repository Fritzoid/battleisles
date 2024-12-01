use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

pub(crate) fn center(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
) {
    commands.spawn((Mesh3d(meshes.add(Cylinder::new(0.01, 1.0))), MeshMaterial3d(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::RED))), Transform {
            translation: Vec3::new(0.5, 0.0, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..default()
        }, NotShadowCaster));

    commands.spawn((Mesh3d(meshes.add(Cylinder::new(0.01, 1.0))), MeshMaterial3d(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::GREEN))), Transform {
            translation: Vec3::new(0.0, 0.5, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..default()
        }, NotShadowCaster));

    commands.spawn((Mesh3d(meshes.add(Cylinder::new(0.01, 1.0))), MeshMaterial3d(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::BLUE))), Transform {
            translation: Vec3::new(0.0, 0.0, 0.5),
            rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
            ..default()
        }, NotShadowCaster));
}
