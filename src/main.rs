use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_boxes)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
 
fn setup_boxes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh: Handle<Mesh> = mesh_assets.add(shape::Box::new(1., 1., 1.).into());   
    commands.spawn( PbrBundle { 
        mesh: mesh.clone(),
        transform: Transform::from_translation(Vec3::new(1., 0., -10.)), 
        ..Default::default()
    });
}
