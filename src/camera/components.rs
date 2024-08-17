use bevy::prelude::*;

#[derive(Debug, Default)]
pub enum ResizeMode {
    Contain,
    #[default]
    Cover,
}

#[derive(Component, Debug, Default)]
pub struct MainCamera {
    pub resize_mode: ResizeMode,
}
