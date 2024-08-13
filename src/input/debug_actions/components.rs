use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::components::ButtonAction;

#[derive(Debug, Event)]
pub struct DebugPerfsEvent(pub ButtonAction);

#[derive(Debug, Event)]
pub struct DebugPhysicsEvent(pub ButtonAction);

#[derive(
    Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect,
)]
pub enum DebugAction {
    Perfs,
    Physics,
}

impl DebugAction {
    pub fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(DebugAction::Perfs, KeyCode::KeyP);
        input_map.insert(DebugAction::Physics, KeyCode::KeyO);

        input_map
    }
}
