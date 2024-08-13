use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::components::ButtonAction;

#[derive(Debug, Event)]
pub struct AnyKeyEvent(pub ButtonAction);

#[derive(
    Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect,
)]
pub enum GlobalAction {
    AnyKey,
}

impl GlobalAction {
    pub fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_one_to_many(
            GlobalAction::AnyKey,
            vec![KeyCode::Escape],
        );

        input_map
    }
}
