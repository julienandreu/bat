use bevy::app::App;
use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::hsl(231., 0.43, 0.16)))
            .add_systems(Startup, setup_main_camera)
            .add_systems(Update, on_resize);
    }
}
