use bevy::app::App;
use bevy::prelude::*;
use components::*;
use main_menu::MainMenuStatePlugin;
use splash_screen::SplashScreenStatePlugin;

pub mod components;
mod main_menu;
mod splash_screen;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((
            SplashScreenStatePlugin,
            MainMenuStatePlugin,
        ));
    }
}
