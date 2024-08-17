use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::animation::components::{AnimationIndices, AnimationTimer};

#[derive(Debug, Default)]
pub enum EntityState {
    #[default]
    Idle,
    Walk,
    ThumbUp,
}

#[derive(Default, Component)]
pub struct Julien {
    pub current_state: EntityState,
}

impl From<&Julien> for AnimationIndices {
    fn from(val: &Julien) -> Self {
        match val.current_state {
            EntityState::Idle => AnimationIndices { first: 0, last: 1 },
            EntityState::Walk => {
                AnimationIndices { first: 7, last: 10 }
            }
            EntityState::ThumbUp => AnimationIndices {
                first: 14,
                last: 20,
            },
        }
    }
}

impl From<&Julien> for AnimationTimer {
    fn from(val: &Julien) -> Self {
        let duration = match val.current_state {
            EntityState::Idle => 0.256,
            EntityState::Walk => 0.128,
            EntityState::ThumbUp => 0.128,
        };

        AnimationTimer {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct JulienBundle {
    julien: Julien,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    pub grid_coords: GridCoords,
}
