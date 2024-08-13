use bevy::app::App;
use bevy::prelude::*;
use debug_actions::DebugInputPlugin;
use global::GlobalInputPlugin;

pub mod components;
pub mod debug_actions;
pub mod global;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugInputPlugin, GlobalInputPlugin));
    }
}
