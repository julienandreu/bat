use bevy::app::App;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use systems::*;

mod systems;

const GRAVITY_SCALE: f32 = 981.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                GRAVITY_SCALE,
            ),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .add_systems(PreUpdate, toggle_debug);
    }
}
