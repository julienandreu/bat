use bevy::app::App;
use bevy::prelude::*;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_event::<JumpEvent>()
            .add_event::<ToggleEvent>()
            .init_resource::<JoinedPlayers>()
            .add_systems(Update, (join, jump, toggle, disconnect));
    }
}
