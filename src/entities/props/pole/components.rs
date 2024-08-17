use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::animation::components::{AnimationIndices, AnimationTimer};

#[derive(Default, Component)]
pub struct Pole;

impl From<&Pole> for AnimationIndices {
    fn from(_val: &Pole) -> Self {
        AnimationIndices { first: 0, last: 3 }
    }
}

impl From<&Pole> for AnimationTimer {
    fn from(_val: &Pole) -> Self {
        AnimationTimer {
            timer: Timer::from_seconds(0.128, TimerMode::Repeating),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PoleBundle {
    pole: Pole,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}
