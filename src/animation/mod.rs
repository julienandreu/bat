use bevy::app::App;
use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render);
    }
}
