use std::time::Duration;

use bevy::app::App;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use systems::*;

use super::AppState;

pub mod components;
mod systems;

pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), on_enter)
            .add_systems(OnExit(AppState::MainMenu), on_exit)
            .add_systems(
                Update,
                (
                    next_state.run_if(in_state(AppState::MainMenu)),
                    activate
                        .run_if(on_timer(Duration::from_millis(250))),
                ),
            );
    }
}
