use std::collections::HashMap;
use bevy::prelude::*; 
use bevy::render::render_asset::RenderAssetUsages;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_pancam::{PanCamPlugin, PanCam};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use hexx::*;

mod battle_map;

pub struct BattleIslesGame;

impl BattleIslesGame {
    pub fn run() {
        App::new()
            .add_plugins((DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            }),PanCamPlugin::default()))
            .add_plugins(EguiPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_input,ui_system))
            .run();
    }
}

#[derive(Resource)]
struct BevyBattleMap {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
}

#[derive(Component)]
struct GameCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>    
) {
    let mapstr =  r#"{
        "size":[3,3],
        "hexes":[
            {"hex_type":"DeepWater"},
            {"hex_type":"Plains"},
            {"hex_type":"ShallowWater"},
            {"hex_type":"Mountains"}, 
            {"hex_type":"Hills"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}, 
            {"hex_type":"DeepWater"}
        ]
    }"#;

    let battle_map = battle_map::BattleMap::from_json(mapstr);
    let layout = HexLayout { hex_size: Vec2::splat(10.0),  ..default()};
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    commands.spawn((
        Camera2dBundle::default(),
        GameCamera,
    )).insert(PanCam::default());

    let right: i32 = battle_map.size.0 as i32 / 2;
    let left = -right;
    let bottom: i32 = battle_map.size.1 as i32 / 2;
    let top = -bottom;

    let entities = shapes::flat_rectangle([left, right, top, bottom])
        .zip(battle_map.hexes.iter())
        .map(|(hex, battle_hex)| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(ColorMesh2dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    mesh: mesh_handle.clone().into(),
                    material: match battle_hex.hex_type { 
                        battle_map::HexType::DeepWater => materials.add(ColorMaterial::from_color(bevy::color::palettes::css::DARK_BLUE)),
                        battle_map::HexType::ShallowWater => materials.add(ColorMaterial::from_color(bevy::color::palettes::css::BLUE)),
                        battle_map::HexType::Plains => materials.add(ColorMaterial::from_color(bevy::color::palettes::css::GREEN)),
                        battle_map::HexType::Mountains => materials.add(ColorMaterial::from_color(bevy::color::palettes::css::GRAY)),
                        battle_map::HexType::Hills => materials.add(ColorMaterial::from_color(bevy::color::palettes::css::YELLOW)),
                    },
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(BevyBattleMap {
        layout,
        entities,
    });
}

fn handle_input(    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    map: Res<BevyBattleMap>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        if buttons.just_pressed(MouseButton::Left) {
            let coord = map.layout.world_pos_to_hex(pos);
            if let Some(entity) = map.entities.get(&coord).copied() {
                dbg!(entity);
            }
        }
    }
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.95))
        .build();
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn ui_system(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    // Top panel
    egui::TopBottomPanel::top("top_panel")
        .default_height(50.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Top Panel"));
        });

    // Bottom panel
    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(50.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Bottom Panel"));
        });

    // Left panel
    egui::SidePanel::left("left_panel")
        .default_width(100.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Left Panel"));
        });

    // Right panel
    egui::SidePanel::right("right_panel")
        .default_width(100.0)
        .show(ctx, |ui| {
            ui.add(egui::Label::new("Right Panel"));
        });

    // Set the background color of the panels to light blue
    ctx.set_visuals(egui::Visuals {
        panel_fill: egui::Color32::from_rgb(173, 216, 230),
        ..Default::default()
    });
}
