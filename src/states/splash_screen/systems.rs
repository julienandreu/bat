use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::components::{AnimationIndices, AnimationTimer},
    entities::characters::julien::components::{EntityState, Julien},
    input::{
        components::ButtonAction, global::components::AnyKeyEvent,
    },
    player::components::{JumpEvent, ToggleEvent},
    states::AppState,
};

use super::components::*;

pub fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<SplashScreen>>,
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

pub fn toggle(
    mut listener: EventReader<ToggleEvent>,
    mut query: Query<(
        Entity,
        &mut Julien,
        &mut TextureAtlas,
        &mut AnimationIndices,
        &mut AnimationTimer,
    )>,
) {
    if let Ok((
        _,
        mut julien,
        mut texture_atlas,
        mut animation_indices,
        mut animation_timer,
    )) = query.get_single_mut()
    {
        for _ in listener.read() {
            julien.current_state = match julien.current_state {
                EntityState::Idle => EntityState::Walk,
                _ => EntityState::Idle,
            };

            let i = AnimationIndices::from(julien.as_ref());
            let t = AnimationTimer::from(julien.as_ref());

            texture_atlas.index = i.first;
            animation_indices.first = i.first;
            animation_indices.last = i.last;

            animation_timer.timer = t.timer;
        }
    }
}

pub fn thumb_up(
    mut listener: EventReader<JumpEvent>,
    mut query: Query<(
        Entity,
        &mut Julien,
        &mut TextureAtlas,
        &mut AnimationIndices,
        &mut AnimationTimer,
    )>,
) {
    if let Ok((
        _,
        mut julien,
        mut texture_atlas,
        mut animation_indices,
        mut animation_timer,
    )) = query.get_single_mut()
    {
        for _ in listener.read() {
            julien.current_state = EntityState::ThumbUp;

            let i = AnimationIndices::from(julien.as_ref());
            let t = AnimationTimer::from(julien.as_ref());

            texture_atlas.index = i.first;
            animation_indices.first = i.first;
            animation_indices.last = i.last;

            animation_timer.timer = t.timer;
        }
    }
}
