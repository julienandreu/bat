use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    input::{
        components::ButtonAction, global::components::AnyKeyEvent,
    },
    states::AppState,
};

use super::components::*;

pub fn on_enter(
    mut commands: Commands,
    query: Query<Entity, With<SplashScreen>>,
    asset_server: Res<AssetServer>,
) {
    info!("OnEnter Splash Screen");

    if let Ok(e) = query.get_single() {
        info!("Removing existing SplashScreenBundle {:?}", e);
        commands.entity(e).despawn_recursive();
    }

    let ldtk_handle = asset_server.load("levels/splash-screen.ldtk");
    commands.spawn(SplashScreenBundle {
        name: Name::new("SplashScreen"),
        ldtk_world_bundle: LdtkWorldBundle {
            ldtk_handle,
            ..Default::default()
        },
        ..default()
    });
}

pub fn on_exit(
    mut commands: Commands,
    query: Query<Entity, With<SplashScreen>>,
) {
    info!("OnExit Splash Screen");

    if let Ok(e) = query.get_single() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn activate(
    mut commands: Commands,
    query: Query<Entity, (With<SplashScreen>, Without<IsReady>)>,
) {
    if let Ok(e) = query.get_single() {
        info!("Activate Splash Screen");
        commands.entity(e).insert(IsReady);
    }
}

pub fn next_state(
    query: Query<&IsReady, With<SplashScreen>>,
    mut listener: EventReader<AnyKeyEvent>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    if query.get_single().is_ok() {
        for ev in listener.read() {
            if ev.0 == ButtonAction::JustPressed {
                info!("AnyKey JustPressed on Splash Screen");
                next_app.set(AppState::MainMenu);
            }
        }
    }
}
