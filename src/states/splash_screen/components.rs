use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Bundle, Default)]
pub struct SplashScreenBundle {
    pub name: Name,
    pub ldtk_world_bundle: LdtkWorldBundle,
    pub splash_screen: SplashScreen,
}

#[derive(Component, Debug, Default)]
pub struct SplashScreen;

#[derive(Component, Debug, Default)]
pub struct IsReady;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Ground;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    ground: Ground,
}
