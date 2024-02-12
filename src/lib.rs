pub mod game;
pub mod settings;

use crate::settings::*;
use bevy::prelude::*;

pub fn get_window_resolution() -> Vec2 {
    let width = 2.0 * MARGIN
        + ALIENS_PER_LINE as f32 * ALIEN_SIZE.x
        + (ALIENS_PER_LINE - 1) as f32 * SPACE_BETWEEN_ALIENS.x;
    let height = 600.0;
    Vec2::new(width, height)
}

pub fn get_shelter_size() -> Vec2 {
    SHELTER_SIZE * SHELTER_SCALE_FACTOR
}
