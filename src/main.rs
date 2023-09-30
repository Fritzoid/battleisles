use bevy::prelude::*;
mod hexx_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_plane)
        .run()
}

fn setup_camera(mut commands: Commands) {
    let cam = 
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 30.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        };

    commands.spawn(cam);
}
 
fn setup_plane(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let hexx_mesh: Mesh = hexx_mesh::hexx_mesh();
    let mesh = mesh_assets.add(hexx_mesh);
    commands.spawn( PbrBundle { 
        mesh: mesh.clone(),
        ..Default::default()
    });
}