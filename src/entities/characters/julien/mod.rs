use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::JulienBundle;
use systems::*;

use crate::states::components::AppState;

pub mod components;
mod systems;

pub struct JulienPlugin;

impl Plugin for JulienPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<JulienBundle>("Julien")
            .add_systems(
                Update,
                setup.run_if(in_state(AppState::SplashScreen)),
            );
    }
}
