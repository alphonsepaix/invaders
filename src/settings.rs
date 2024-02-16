use bevy::prelude::*;

pub const WINDOW_VISIBLE_DELAY: u32 = 3;

pub const PLAYER_SIZE: Vec2 = Vec2::new(60.0, 30.0);
pub const PLAYER_SPEED: f32 = 300.0;

pub const NUM_SHELTERS: usize = 4;
pub const SHELTER_SIZE: Vec2 = Vec2::new(40.0, 20.0);
pub const SHELTER_SCALE_FACTOR: f32 = 2.5;
pub const INITIAL_ARMOR_VALUE: u32 = 100;

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

pub const EXPLOSION_DURATION: f32 = 0.35;
pub const EXPLOSION_MIN_RADIUS: f32 = 13.0;
pub const EXPLOSION_MAX_RADIUS: f32 = 30.0;

pub const XP_GAIN_DURATION: f32 = 1.0;

pub const FLOOR_HEIGHT: f32 = 50.0;
pub const FLOOR_THICKNESS: f32 = 5.0;

pub const BACKGROUND_COLOR: Color = Color::BLACK;
pub const SCOREBOARD_FONT_SIZE: f32 = 28.0;
pub const TEXT_COLOR: Color = Color::YELLOW;
pub const MENU_TEXT_COLOR: Color = Color::YELLOW;
pub const TEXT_BUTTON_SIZE: f32 = 40.0;
pub const BUTTON_WIDTH: f32 = 250.0;
pub const BUTTON_HEIGHT: f32 = 65.0;
pub const BUTTON_MARGIN: f32 = 20.0;
pub const MENU_TITLE_SIZE: f32 = 40.0;
pub const NORMAL_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const PRESSED_BUTTON: Color = Color::rgb(0.45, 0.85, 0.45);
