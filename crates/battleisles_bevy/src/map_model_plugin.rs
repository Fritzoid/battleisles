use bevy::ecs::resource;
use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::window::{Window, WindowResized};
use bevy::render::camera::ScalingMode;
use battleisles_domain::map::Map;
use battleisles_domain::map::MapErrors;

pub struct MapModelPlugin;
use crate::map_model::MapModel;

impl Plugin for MapModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                update_camera_to_fit_map
                    .run_if(on_event::<WindowResized>)
                    .run_if(resource_exists::<MapModel>),
            ));
    }
}

impl MapModelPlugin {
    pub fn initialize_map(
        map: Map, 
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Result<(), MapErrors> {
        let map_model = MapModel::try_new(map, commands, meshes, materials).unwrap();
        commands.insert_resource(map_model);
        Ok(())
    }
}

pub fn update_camera_to_fit_map(
    mut query: Query<&mut Projection, With<Camera3d>>,
    window_query: Query<&Window>,
    map_model: Res<MapModel>,
) {
    let window = window_query.single().unwrap();
    let usable_w = window.resolution.physical_width() as f32 - 100.0 - 100.0;
    let usable_h = window.resolution.physical_height() as f32 - 50.0 - 50.0;
    let aspect_w = usable_w / usable_h;
    let map_width = map_model.map_right - map_model.map_left;
    let map_height = map_model.map_top - map_model.map_bottom;
    let aspect_map = map_width / map_height;
    let mut projection = query.single_mut().unwrap();

    if let Projection::Orthographic(ref mut ortho) = *projection {
        if aspect_w > aspect_map {
            // Window is wider than map → fit height
            let target_h = map_height;
            let target_w = target_h * aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        } else {
            // Window is taller than map → fit width
            let target_w = map_width;
            let target_h = target_w / aspect_w;

            ortho.scaling_mode = ScalingMode::Fixed {
                width: target_w,
                height: target_h,
            };
        }
    }
}