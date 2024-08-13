use bevy::prelude::*;

use crate::{
    input::{
        components::ButtonAction, global::components::AnyKeyEvent,
    },
    states::AppState,
};

use super::components::*;

pub fn on_enter(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    info!("OnEnter Main Menu");

    if let Ok(e) = query.get_single() {
        info!("Removing existing MainMenuBundle {:?}", e);
        commands.entity(e).despawn_recursive();
    }

    commands.spawn(MainMenuBundle {
        name: Name::new("MainMenu"),
        ..default()
    });
}

pub fn on_exit(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    info!("OnExit Main Menu");

    if let Ok(e) = query.get_single() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn activate(
    mut commands: Commands,
    query: Query<Entity, (With<MainMenu>, Without<IsReady>)>,
) {
    if let Ok(e) = query.get_single() {
        info!("Activate Main Menu");
        commands.entity(e).insert(IsReady);
    }
}

pub fn next_state(
    query: Query<&IsReady, With<MainMenu>>,
    mut listener: EventReader<AnyKeyEvent>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    if query.get_single().is_ok() {
        for ev in listener.read() {
            if ev.0 == ButtonAction::JustPressed {
                info!("AnyKey JustPressed on Main Menu");
                next_app.set(AppState::SplashScreen);
            }
        }
    }
}
