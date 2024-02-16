pub mod game;
pub mod settings;
pub mod ui;

use crate::settings::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Pause,
    InGame,
}

pub fn get_window_resolution() -> Vec2 {
    let width = 2.0 * MARGIN
        + ALIENS_PER_LINE as f32 * ALIEN_SIZE.x
        + (ALIENS_PER_LINE - 1) as f32 * SPACE_BETWEEN_ALIENS.x;
    let height = 600.0;
    Vec2::new(width, height)
}

pub fn despawn_screen<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
