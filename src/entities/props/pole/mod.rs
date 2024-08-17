use bevy::app::App;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use components::*;
use systems::*;

use crate::states::components::AppState;

pub mod components;
mod systems;

pub struct PolePlugin;

impl Plugin for PolePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PoleBundle>("Pole").add_systems(
            Update,
            (setup, render).run_if(in_state(AppState::SplashScreen)),
        );
    }
}
