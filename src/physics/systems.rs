use bevy::prelude::*;
use bevy_rapier2d::render::DebugRenderContext;

use crate::input::{
    components::ButtonAction,
    debug_actions::components::DebugPhysicsEvent,
};

pub fn toggle_debug(
    mut listener: EventReader<DebugPhysicsEvent>,
    mut ctx: ResMut<DebugRenderContext>,
) {
    for ev in listener.read() {
        if ev.0 != ButtonAction::JustPressed {
            return;
        }

        ctx.enabled = !ctx.enabled;
    }
}
