use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use hexx::*;

const HEX_SIZE: Vec2 = Vec2::splat(13.0);

pub fn hexx_mesh() -> Mesh {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    return hexagonal_plane(&layout);
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info: MeshInfo = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
