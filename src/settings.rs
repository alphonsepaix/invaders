use bevy::prelude::*;

pub const WINDOW_VISIBLE_DELAY: u32 = 3;

pub const PLAYER_SIZE: Vec2 = Vec2::new(60.0, 30.0);
pub const PLAYER_SPEED: f32 = 300.0;

pub const NUM_SHELTERS: usize = 4;
pub const SHELTER_SIZE: Vec2 = Vec2::new(40.0, 20.0);
pub const SHELTER_SCALE_FACTOR: f32 = 2.5;

pub const ALIENS_PER_LINE: usize = 11;
pub const SPACE_BETWEEN_ALIENS: Vec2 = Vec2::new(20.0, 16.0);
pub const MARGIN: f32 = 80.0;
pub const ALIEN_SHOOT_PROB: f32 = 1.0 / 40.0 / 60.0;
pub const ALIEN_SIZE: Vec2 = Vec2::new(40.0, 30.0);
pub const YELLOW_ALIEN_VALUE: u32 = 30;
pub const GREEN_ALIEN_VALUE: u32 = 20;
pub const RED_ALIEN_VALUE: u32 = 10;
pub const ALIEN_TICK_DURATION: f32 = 0.8;

pub const UFO_VALUE: u32 = 300;
pub const UFO_SPAWN_PROB: f32 = 1.0 / 30.0;
pub const UFO_SIZE: Vec2 = Vec2::new(82.0, 36.0);
pub const UFO_SPEED: f32 = 150.0;

pub const LASER_SIZE: Vec2 = Vec2::new(5.0, 15.0);
pub const PLAYER_LASER_SPEED: f32 = 600.0;
pub const ALIEN_LASER_SPEED: f32 = 300.0;
pub const MAX_ALIEN_LASERS: usize = 4;

pub const FLOOR_HEIGHT: f32 = 60.0;
