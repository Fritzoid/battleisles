use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

pub(crate) fn center(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.01, 1.0)),
            transform: Transform {
                translation: Vec3::new(0.5, 0.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..default()
            },
            material: materials.add(StandardMaterial::from_color(
                bevy::color::palettes::css::RED,
            )),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.01, 1.0)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.5, 0.0),
                ..default()
            },
            material: materials.add(StandardMaterial::from_color(
                bevy::color::palettes::css::GREEN,
            )),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.01, 1.0)),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
            material: materials.add(StandardMaterial::from_color(
                bevy::color::palettes::css::BLUE,
            )),
            ..default()
        },
        NotShadowCaster,
    ));
}
