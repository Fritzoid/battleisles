use bevy::prelude::*;
use bevy::pbr::ExtendedMaterial;
use bevy::render::texture::ImageLoaderSettings;
use bevy::render::texture::ImageSampler;
use bevy::render::texture::ImageSamplerDescriptor;
use bevy::render::texture::ImageAddressMode;
use bevy::render::texture::ImageFilterMode;
use bevy::math::vec4;
use crate::water::Water;
use crate::water::WaterSettings;

const HEX_SIZE: f32 = 5.0;
const HEX_THICKNESS: f32 = 0.01;

#[derive(PartialEq)]
pub enum HexType {
    DeepWater,
    ShallowWater,
    Plains,
    Hills,
    Mountains,
}

pub struct MapInfo {
    pub width: u16,
    pub height: u16,
    pub hexes: Vec<HexType>,
}

pub fn init_map(
    map: MapInfo,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
    asset_server: &Res<AssetServer>,
) {
    let shape = RegularPolygon::new(HEX_SIZE, 6);
    let mesh = Extrusion::new(shape, HEX_THICKNESS);
    let mesh_handle = meshes.add(mesh);
    let plains_mat = materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::GREEN,
    ));
    let mountains_mat = materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::GRAY,
    ));
    let hills_mat = materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::YELLOW,
    ));

    let deepwater_mat = water_materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: bevy::color::palettes::css::DARK_BLUE.into(),
            perceptual_roughness: 0.0,
            ..default()
        },
        extension: Water {
            normals: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                "textures/water_normals.png",
                |settings| {
                    settings.is_srgb = false;
                    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        mag_filter: ImageFilterMode::Linear,
                        min_filter: ImageFilterMode::Linear,
                        ..default()
                    });
                },
            ),
            // These water settings are just random values to create some
            // variety.
            settings: WaterSettings {
                octave_vectors: [
                    vec4(0.080, 0.059, 0.073, -0.062),
                    vec4(0.153, 0.138, -0.149, -0.195),
                ],
                octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 5.0,
                octave_strengths: vec4(0.16, 0.18, 0.093, 0.044),
            },
        },
    });

    let shallowwater_mat = water_materials.add(ExtendedMaterial {
        base: StandardMaterial {
            base_color: bevy::color::palettes::css::BLUE.into(),
            perceptual_roughness: 0.0,
            ..default()
        },
        extension: Water {
            normals: asset_server.load_with_settings::<Image, ImageLoaderSettings>(
                "textures/water_normals.png",
                |settings| {
                    settings.is_srgb = false;
                    settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        mag_filter: ImageFilterMode::Linear,
                        min_filter: ImageFilterMode::Linear,
                        ..default()
                    });
                },
            ),
            // These water settings are just random values to create some
            // variety.
            settings: WaterSettings {
                octave_vectors: [
                    vec4(0.080, 0.059, 0.073, -0.062),
                    vec4(0.153, 0.138, -0.149, -0.195),
                ],
                octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 5.0,
                octave_strengths: vec4(0.16, 0.18, 0.093, 0.044),
            },
        },
    });

    let mut x: f32;
    let mut z: f32 = 0.0;
    let mut i: usize = 0;

    for h in 1..=map.height {
        x = 0.0;
        for w in 1..=map.width {
            if map.hexes[i] == HexType::DeepWater {
                commands.spawn(MaterialMeshBundle {
                    mesh: mesh_handle.clone(),
                    material: deepwater_mat.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, z),
                        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                            * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 3.0),
                        ..default()
                    },
                    ..default()
                });
            } else if map.hexes[i] == HexType::ShallowWater {
                commands.spawn(MaterialMeshBundle {
                    mesh: mesh_handle.clone(),
                    material: shallowwater_mat.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, z),
                        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                            * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 3.0),
                        ..default()
                    },
                    ..default()
                });
            }
            else {
                commands.spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: match map.hexes[i] {
                        HexType::Plains => plains_mat.clone(),
                        HexType::Mountains => mountains_mat.clone(),
                        HexType::Hills => hills_mat.clone(),
                        _ => unreachable!(),
                    },
                    transform: Transform {
                        translation: Vec3::new(x, 0.0, z),
                        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                            * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 3.0),
                        ..default()
                    },
                    ..default()
                });
            }

            i += 1;
            x += 3.0 / 2.0 * HEX_SIZE;
            z += ((3.0f32).sqrt() / 2.0 * HEX_SIZE) * if w % 2 == 0 { -1.0 } else { 1.0 };
        }
        z = h as f32 * HEX_SIZE * 3.0f32.sqrt();
    }
}
