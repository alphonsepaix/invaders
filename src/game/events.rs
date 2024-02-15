use super::components::Alien;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerHit;

#[derive(Event)]
pub struct AlienHit {
    pub alien_type: Alien,
    pub id: Entity,
    pub position: Vec2,
}

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct LaserExplosion(pub Entity);
