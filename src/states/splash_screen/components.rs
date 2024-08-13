use bevy::prelude::*;

#[derive(Bundle, Debug, Default)]
pub struct SplashScreenBundle {
    pub name: Name,
    pub splash_screen: SplashScreen,
}

#[derive(Component, Debug, Default)]
pub struct SplashScreen;

#[derive(Component, Debug, Default)]
pub struct IsReady;
