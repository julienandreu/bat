use bevy::prelude::*;

#[derive(Bundle, Debug, Default)]
pub struct MainMenuBundle {
    pub name: Name,
    pub main_menu: MainMenu,
}

#[derive(Component, Debug, Default)]
pub struct MainMenu;

#[derive(Component, Debug, Default)]
pub struct IsReady;
