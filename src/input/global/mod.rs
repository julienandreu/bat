use bevy::app::App;
use bevy::prelude::*;
use components::*;
use leafwing_input_manager::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GlobalAction>::default())
            .add_event::<AnyKeyEvent>()
            .register_type::<InputMap<GlobalAction>>()
            .add_systems(Startup, setup_global_actions)
            .add_systems(Update, handle_global_actions);
    }
}
