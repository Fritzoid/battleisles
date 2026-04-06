use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_camera::{OrthographicProjection, Projection};

const ZOOM_SPEED: f32 = 0.1;
const MIN_ZOOM: f32 = 0.4;
const MAX_ZOOM: f32 = 5.0;

#[derive(Resource)]
pub struct CameraZoom {
    pub factor: f32,
    pub base_scale: f32,
}

impl Default for CameraZoom {
    fn default() -> Self {
        Self {
            factor: 1.0,
            base_scale: 0.1,
        }
    }
}

pub struct CameraZoomPlugin;

impl Plugin for CameraZoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraZoom>()
            .add_systems(Update, handle_scroll_zoom);
    }
}

fn handle_scroll_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    mut zoom: ResMut<CameraZoom>,
    mut projections: Query<&mut Projection, With<Camera3d>>,
) {
    let mut scroll_y = 0.0;
    for event in scroll_events.read() {
        scroll_y += event.y;
    }

    if scroll_y == 0.0 {
        return;
    }

    // Scroll up = zoom in (decrease factor), scroll down = zoom out
    zoom.factor = (zoom.factor * (1.0 - scroll_y * ZOOM_SPEED)).clamp(MIN_ZOOM, MAX_ZOOM);

    let Ok(mut projection) = projections.single_mut() else {
        return;
    };

    if let Projection::Orthographic(OrthographicProjection { scale, .. }) = &mut *projection {
        *scale = zoom.base_scale * zoom.factor;
    }
}
