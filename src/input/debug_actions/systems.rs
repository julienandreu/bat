use crate::input::components::ButtonAction;

use super::components::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn setup_debug_actions(mut commands: Commands) {
    commands.spawn((
        Name::new("DebugActions"),
        InputManagerBundle::with_map(DebugAction::input_map()),
    ));
}

pub fn handle_debug_perfs(
    mut ev: EventWriter<DebugPerfsEvent>,
    query: Query<&ActionState<DebugAction>>,
) {
    for input in query.iter() {
        if input.just_pressed(&DebugAction::Perfs) {
            ev.send(DebugPerfsEvent(ButtonAction::JustPressed));
        }

        if input.just_released(&DebugAction::Perfs) {
            ev.send(DebugPerfsEvent(ButtonAction::JustReleased));
        }

        if input.pressed(&DebugAction::Perfs) {
            ev.send(DebugPerfsEvent(ButtonAction::Pressed));
        }

        if input.released(&DebugAction::Perfs) {
            ev.send(DebugPerfsEvent(ButtonAction::Released));
        }
    }
}

pub fn handle_debug_physics(
    mut ev: EventWriter<DebugPhysicsEvent>,
    query: Query<&ActionState<DebugAction>>,
) {
    for input in query.iter() {
        if input.just_pressed(&DebugAction::Physics) {
            ev.send(DebugPhysicsEvent(ButtonAction::JustPressed));
        }

        if input.just_released(&DebugAction::Physics) {
            ev.send(DebugPhysicsEvent(ButtonAction::JustReleased));
        }

        if input.pressed(&DebugAction::Physics) {
            ev.send(DebugPhysicsEvent(ButtonAction::Pressed));
        }

        if input.released(&DebugAction::Physics) {
            ev.send(DebugPhysicsEvent(ButtonAction::Released));
        }
    }
}
