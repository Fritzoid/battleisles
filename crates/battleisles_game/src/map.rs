use bevy::prelude::*;

const HEX_RADIUS: f32 = 5.0;
const HEX_THICKNESS: f32 = 1.0;

#[derive(PartialEq, Clone)]
pub enum HexType {
    Plains,
    Hills,
    Mountains,
    DeepWater,
    ShallowWater,
}

pub struct Hex {
    pub hex_type: HexType,
    pub position: Vec2,
}

pub struct Map {
    pub rows: u16,
    pub collumns: u16,
    pub hexes: Vec<HexType>,
}

impl Map {
    pub fn new(rows: u16, collumns: u16) -> Self {
        let total_hexes = rows * collumns - rows / 2 ;
        let hexes = vec![HexType::DeepWater; total_hexes as usize];
        Map {
            rows,
            collumns,
            hexes,
        }
    }
}

/*
pub fn init_map(
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
) {
    let hex_mesh_handle = create_hex_mesh(meshes);
    let hex_materials = create_hex_materials(materials);

    let mut i: usize = 0;
    for row in 0..map.rows {
    for col in 0..map.width {
        let x = 
        commands.spawn((
            Mesh3d(hex_mesh_handle.clone()),
            MeshMaterial3d(match map.hexes[i] {
                HexType::Plains => hex_materials[0].clone(),
                HexType::Mountains => hex_materials[1].clone(),
                HexType::Hills => hex_materials[2].clone(),
                HexType::DeepWater => hex_materials[3].clone(),
                HexType::ShallowWater => hex_materials[4].clone(),
            }),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
        ));
        i += 1;
    }
}
}

fn create_hex_mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
    let shape = RegularPolygon::new(HEX_RADIUS, 6);
    let mesh = Extrusion::new(shape, HEX_THICKNESS);
    let mesh_handle = meshes.add(mesh);
    mesh_handle
}

fn create_hex_materials(materials: &mut Assets<StandardMaterial>) -> Vec<Handle<StandardMaterial>> {
    let mut material_handles = Vec::<Handle<StandardMaterial>>::new();
    material_handles.push(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::GREEN,
    )));
    material_handles.push(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::GRAY,
    )));
    material_handles.push(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::YELLOW,
    )));
    material_handles.push(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::DARK_BLUE,
    )));
    material_handles.push(materials.add(StandardMaterial::from_color(
        bevy::color::palettes::css::LIGHT_BLUE,
    )));
    material_handles
}
*/