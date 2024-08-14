use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::*;
use systems::*;

use super::AppState;

pub mod components;
mod systems;

pub struct JulienPlugin;

impl Plugin for JulienPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<JulienBundle>("Julien")
            .add_systems(
                Update,
                (setup, render)
                    .run_if(in_state(AppState::SplashScreen)),
            );
    }
}
