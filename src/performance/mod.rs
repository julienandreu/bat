use bevy::app::App;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use iyes_perf_ui::PerfUiPlugin;
use systems::*;

mod systems;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .add_systems(
            PreUpdate,
            toggle_debug.before(iyes_perf_ui::PerfUiSet::Setup),
        );
    }
}
