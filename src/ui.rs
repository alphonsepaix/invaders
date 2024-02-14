use bevy::prelude::*;

pub mod menu;
pub mod pause;

pub use menu::MenuPlugin;
pub use pause::*;

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

pub fn despawn_screen<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn return_to_menu(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}
