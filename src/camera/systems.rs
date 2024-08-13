use bevy::{prelude::*, window::WindowResized};

use super::components::MainCamera;

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
        MainCamera,
    ));
}

pub fn on_resize(
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for event in resize_reader.read() {
        debug!(
            "Window resized to: {:.1}x{:.1}",
            event.width, event.height
        );

        let mut projection = query.single_mut();
        projection.scale =
            (REF_WIDTH / event.width).max(REF_HEIGHT / event.height);
    }
}
