// disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bat::bootstrap::BootstrapPlugin;
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(BootstrapPlugin).run();
}
