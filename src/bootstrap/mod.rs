use bevy::app::App;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};

use crate::camera::CameraPlugin;
use crate::input::InputPlugin;
use crate::performance::PerformancePlugin;
use crate::physics::PhysicsPlugin;
use crate::states::StatesPlugin;

const INITIAL_PHYSICAL_SIZE: (f32, f32) = (1280., 768.);
const INITIAL_WINDOW_MODE: WindowMode = WindowMode::Windowed;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        let (physical_width, physical_height) = INITIAL_PHYSICAL_SIZE;

        app.add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            physical_width,
                            physical_height,
                        ),
                        mode: INITIAL_WINDOW_MODE,
                        position: WindowPosition::Centered(
                            MonitorSelection::Primary,
                        ),
                        ..default()
                    }),
                    ..default()
                },
            ),
            CameraPlugin,
            PerformancePlugin,
            PhysicsPlugin,
            InputPlugin,
            StatesPlugin,
        ));
    }
}
