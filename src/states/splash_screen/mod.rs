use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use systems::*;

use super::AppState;

pub mod components;
mod systems;

pub struct SplashScreenStatePlugin;

impl Plugin for SplashScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SplashScreen), on_enter)
            .add_systems(OnExit(AppState::SplashScreen), on_exit)
            .add_systems(
                Update,
                (
                    next_state.run_if(in_state(AppState::SplashScreen)),
                    activate
                        .run_if(on_timer(Duration::from_millis(250))),
                ),
            );
    }
}
