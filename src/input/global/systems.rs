use crate::input::components::ButtonAction;

use super::components::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn setup_global_actions(mut commands: Commands) {
    commands.spawn((
        Name::new("GlobalAction"),
        InputManagerBundle::with_map(GlobalAction::input_map()),
    ));
}

pub fn handle_global_actions(
    mut ev: EventWriter<AnyKeyEvent>,
    query: Query<&ActionState<GlobalAction>>,
) {
    for input in query.iter() {
        if input.just_pressed(&GlobalAction::AnyKey) {
            ev.send(AnyKeyEvent(ButtonAction::JustPressed));
        }

        if input.just_released(&GlobalAction::AnyKey) {
            ev.send(AnyKeyEvent(ButtonAction::JustReleased));
        }

        if input.pressed(&GlobalAction::AnyKey) {
            ev.send(AnyKeyEvent(ButtonAction::Pressed));
        }

        if input.released(&GlobalAction::AnyKey) {
            ev.send(AnyKeyEvent(ButtonAction::Released));
        }
    }
}
