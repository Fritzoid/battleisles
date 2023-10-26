use std::collections::HashMap;
use std::f32::consts::TAU;
use bevy::{
    prelude::*, 
    window::PrimaryWindow, 
    window::WindowResolution, 
    input::mouse::MouseWheel
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use hexx::*;
mod hexx_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600., 600.),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, handle_camera_fov)
        .run()
}


#[derive(Debug, Resource)]
struct BattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    default_material: Handle<StandardMaterial>,
    selected_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct MyGameCamera;

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout { hex_size: Vec2::splat(20.0),  ..default()};
    let default_material = materials.add(Color::BLUE.into());
    let selected_material = materials.add(Color::RED.into());
    let hexx_mesh: Mesh = hexx_mesh::hexx_mesh(&layout);
    let mesh = meshes.add(hexx_mesh);

    let projection = Projection::Perspective(PerspectiveProjection { fov: fov(600.,600., std::f32::consts::PI/4.0), ..default()});

    let cam: Camera3dBundle = Camera3dBundle {
        projection: projection,
        ..default()
    };

    commands.spawn((
        // Note we're setting the initial position below with alpha, beta, and radius, hence
        // we don't set transform on the camera.
        cam,
        MyGameCamera,
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::new(0.0, 0.0, 0.0),
            // Set the starting position, relative to focus (overrides camera's transform).
//            alpha: Some(TAU / 8.0),
//            beta: Some(TAU / 8.0),
            alpha: Some(0.),
            beta: Some(0.),
            radius: Some(200.0),
            // Set limits on rotation and zoom
//            alpha_upper_limit: Some(TAU / 4.0),
//            alpha_lower_limit: Some(-TAU / 4.0),
//            beta_upper_limit: Some(TAU / 3.0),
//            beta_lower_limit: Some(-TAU / 3.0),
//            zoom_upper_limit: Some(5.0),
//            zoom_lower_limit: Some(1.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
//            allow_upside_down: true,
            // Change the controls (these match Blender)
//            button_orbit: MouseButton::Middle,
//            button_pan: MouseButton::Middle,
//            modifier_pan: Some(KeyCode::ShiftLeft),
            // Reverse the zoom direction
//            reversed_zoom: true,
            ..default()
        },
    ));

    let entities = shapes::flat_rectangle([-5, 5, -5, 5])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(0.99)),
                    mesh: mesh.clone().into(),
                    material: default_material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(BattleMap {
        layout,
        entities,
        default_material, 
        selected_material,
    });
}

fn fov(height: f32, newheight: f32, fov: f32 ) -> f32 {
    ((fov / 2.0).tan() * (newheight/height)).atan() * 2.0
}


fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    map: Res<BattleMap>,
) {
    /*
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        if buttons.just_pressed(MouseButton::Left) {
            let coord = map.layout.world_pos_to_hex(pos);
            if let Some(entity) = map.entities.get(&coord).copied() {
                commands
                    .entity(entity)
                    .insert(map.selected_material.clone());
            }
        }

        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
        }
    }
     */
}

fn handle_camera_fov(mut q: Query<&mut Projection, With<MyGameCamera>>) {
    let mut projection = q.single_mut();
    dbg!(projection);
}