use bevy::{prelude::*, window::WindowResized};

use super::components::*;

const REF_WIDTH: f32 = 400.;
const REF_HEIGHT: f32 = 216.;

fn build_main_camera() -> Camera2dBundle {
    let mut main_camera = Camera2dBundle::default();
    main_camera.transform.translation.x += REF_WIDTH / 2.;
    main_camera.transform.translation.y += REF_HEIGHT / 2.;

    main_camera
}

pub fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("MainCamera"),
        build_main_camera(),
        MainCamera::default(),
    ));
}

pub fn resize(
    projection: &mut OrthographicProjection,
    width: f32,
    height: f32,
    resize_mode: &ResizeMode,
) {
    projection.scale = match resize_mode {
        ResizeMode::Contain => {
            (REF_WIDTH / width).max(REF_HEIGHT / height)
        }

        ResizeMode::Cover => {
            (REF_WIDTH / width).min(REF_HEIGHT / height)
        }
    }
}

pub fn on_resize(
    mut query: Query<(&MainCamera, &mut OrthographicProjection)>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for event in resize_reader.read() {
        debug!(
            "Window resized to: {:.1}x{:.1}",
            event.width, event.height
        );

        let (main_camera, mut projection) = query.single_mut();
        resize(
            &mut projection,
            event.width,
            event.height,
            &main_camera.resize_mode,
        );
    }
}
