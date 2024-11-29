use bevy::prelude::*;

const HEX_SIZE: f32 = 5.0;
const HEX_THICKNESS: f32 = 0.01;

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
    materials: &mut Assets<StandardMaterial>) {

    let shape = RegularPolygon::new(HEX_SIZE, 6);
    let mesh = Extrusion::new(shape, HEX_THICKNESS);
    let mesh_handle = meshes.add(mesh);
    let deep_water_mat = materials.add(StandardMaterial::from_color(bevy::color::palettes::css::DARK_BLUE));
    let shallow_water_mat = materials.add(StandardMaterial::from_color(bevy::color::palettes::css::BLUE));
    let plains_mat = materials.add(StandardMaterial::from_color(bevy::color::palettes::css::GREEN));
    let mountains_mat = materials.add(StandardMaterial::from_color(bevy::color::palettes::css::GRAY));
    let hills_mat = materials.add(StandardMaterial::from_color(bevy::color::palettes::css::YELLOW));
    
    let mut x: f32;
    let mut z: f32 = 0.0;
    let mut i: usize = 0;

    for h in 1..=map.height {
        x = 0.0;
        for w in 1..=map.width {
            commands.spawn( PbrBundle {
                mesh: mesh_handle.clone(),
                material: match map.hexes[i] {
                    HexType::DeepWater => deep_water_mat.clone(),
                    HexType::ShallowWater => shallow_water_mat.clone(),
                    HexType::Plains => plains_mat.clone(),
                    HexType::Mountains => mountains_mat.clone(),
                    HexType::Hills => hills_mat.clone(),
                },
                transform: Transform { 
                    translation: Vec3::new(x, 0.0, z),
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2/3.0),
                    ..default()
                },
                ..default()
            });
            i += 1;
            x += 3.0/2.0 * HEX_SIZE;
            z += ((3.0f32).sqrt()/2.0 * HEX_SIZE) * if w % 2 == 0 { -1.0 } else { 1.0 };
        }
        z = h as f32 * HEX_SIZE * 3.0f32.sqrt();
    }
}
