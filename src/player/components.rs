use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::*;

#[derive(
    Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect,
)]
pub enum Action {
    Jump,
    Disconnect,
}

// This is used to check if a player already exists and which entity to disconnect
#[derive(Resource, Default)]
pub struct JoinedPlayers(pub HashMap<Gamepad, Entity>);

#[derive(Component)]
pub struct Player {
    // This gamepad is used to index each player
    pub gamepad: Gamepad,
}
