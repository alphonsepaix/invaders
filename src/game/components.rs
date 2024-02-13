use crate::settings::*;
use bevy::prelude::*;

#[derive(Clone, PartialEq)]
pub enum EntityDirection {
    Up,
    Down,
    Left,
    Right,
}

impl EntityDirection {
    pub fn mask(&self) -> Vec3 {
        match self {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            EntityDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            EntityDirection::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OnMenuScreen;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component)]
pub enum Alien {
    Yellow,
    Green,
    Red,
    Ufo,
}

#[derive(Component)]
pub struct Ufo(pub EntityDirection);

#[derive(Component)]
pub struct Shelter {
    pub armor: u32,
}

#[derive(Component)]
pub struct Laser {
    pub direction: EntityDirection,
    pub speed: f32,
}

impl Alien {
    pub fn color(&self) -> Color {
        match self {
            Alien::Yellow => Color::YELLOW,
            Alien::Green => Color::GREEN,
            Alien::Red => Color::RED,
            Alien::Ufo => Color::RED,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Alien::Yellow => YELLOW_ALIEN_VALUE,
            Alien::Green => GREEN_ALIEN_VALUE,
            Alien::Red => RED_ALIEN_VALUE,
            Alien::Ufo => UFO_VALUE,
        }
    }
}
