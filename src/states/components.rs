use bevy::prelude::*;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    SplashScreen,
    MainMenu,
}
