pub mod game;
pub mod menu;
pub mod pause;

pub use game::*;
pub use menu::MenuPlugin;
pub use pause::*;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Pause,
    InGame,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    Transition,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionState {
    #[default]
    Unset,
    PlayerKilled,
    AliensKilled,
    GameOver,
}

pub fn despawn_screen<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
