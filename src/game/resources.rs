use super::components::EntityDirection;
use bevy::prelude::*;

pub struct InvadersMovingSound {
    index: usize,
    sounds: [Handle<AudioSource>; 4],
}

impl InvadersMovingSound {
    pub fn new(index: usize, sounds: [Handle<AudioSource>; 4]) -> Self {
        Self { index, sounds }
    }

    pub fn get(&mut self) -> Handle<AudioSource> {
        let source = self.sounds[self.index].clone();
        self.index = (self.index + 1) % self.sounds.len();
        source
    }
}

#[derive(Resource)]
pub struct ButtonHoveredSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ButtonPressedSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct AlreadyPlayed(pub bool);

#[derive(Deref, DerefMut, Resource)]
pub struct TransitionTimer(pub Timer);

#[derive(Resource)]
pub struct PlayerScore(pub u32);

#[derive(Resource)]
pub struct ShootSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ExplosionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct InvaderKilledSound(pub Handle<AudioSource>);

#[derive(Deref, DerefMut, Resource)]
pub struct AlienSounds(pub InvadersMovingSound);

#[derive(Deref, DerefMut, Resource)]
pub struct AlienTimer(pub Timer);

#[derive(Deref, DerefMut, Resource)]
pub struct UfoTimer(pub Timer);

#[derive(Resource)]
pub struct AlienDirection {
    pub previous: EntityDirection,
    pub next: EntityDirection,
}

#[derive(Resource)]
pub struct LivesRemaining(pub u32);
