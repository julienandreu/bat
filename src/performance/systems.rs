use bevy::prelude::*;
use iyes_perf_ui::{
    entries::PerfUiBundle,
    prelude::{PerfUiEntryEntityCount, PerfUiRoot},
};

use crate::input::{
    components::ButtonAction,
    debug_actions::components::DebugPerfsEvent,
};

pub fn toggle_debug(
    mut listener: EventReader<DebugPerfsEvent>,
    mut commands: Commands,
    query: Query<Entity, With<PerfUiRoot>>,
) {
    for ev in listener.read() {
        if ev.0 != ButtonAction::JustPressed {
            return;
        }

        if let Ok(e) = query.get_single() {
            commands.entity(e).despawn_recursive();
        } else {
            commands.spawn(PerfUiBundle {
                entity_count: PerfUiEntryEntityCount::default(),
                ..default()
            });
        }
    }
}
