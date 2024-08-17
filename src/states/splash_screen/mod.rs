use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_ecs_ldtk::prelude::*;
use components::*;
use systems::*;

use crate::entities::characters::julien::JulienPlugin;
use crate::entities::props::pole::PolePlugin;

use super::AppState;

pub mod components;
mod systems;

pub struct SplashScreenStatePlugin;

impl Plugin for SplashScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LdtkPlugin, PolePlugin, JulienPlugin))
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell_for_layer::<GroundBundle>(
                "Ground", 1,
            )
            .add_systems(OnEnter(AppState::SplashScreen), on_enter)
            .add_systems(OnExit(AppState::SplashScreen), on_exit)
            .add_systems(
                Update,
                (
                    (toggle, thumb_up, next_state)
                        .run_if(in_state(AppState::SplashScreen)),
                    activate
                        .run_if(on_timer(Duration::from_millis(250))),
                ),
            );
    }
}
