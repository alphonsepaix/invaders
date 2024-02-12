use super::components::Alien;
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerHit;

#[derive(Event)]
pub struct AlienHit {
    pub alien_type: Alien,
    pub id: Entity,
}
