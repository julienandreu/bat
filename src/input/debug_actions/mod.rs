use bevy::app::App;
use bevy::prelude::*;
use components::*;
use leafwing_input_manager::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct DebugInputPlugin;

impl Plugin for DebugInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DebugAction>::default())
            .add_event::<DebugPerfsEvent>()
            .add_event::<DebugPhysicsEvent>()
            .register_type::<InputMap<DebugAction>>()
            .add_systems(Startup, setup_debug_actions)
            .add_systems(
                Update,
                (handle_debug_perfs, handle_debug_physics),
            );
    }
}
