pub mod systems;

use crate::game::{EntityDirection, GameState};
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct LaserExplosion(pub Entity);

#[derive(Component)]
pub struct Laser {
    pub direction: EntityDirection,
    pub speed: f32,
    pub source: Option<Entity>,
}

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

pub struct LasersPlugin;

impl Plugin for LasersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserExplosion>()
            .add_systems(
                Update,
                (move_lasers, despawn_lasers, check_for_collisions)
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(not(in_state(GameState::Pause))),
            )
            .add_systems(
                Update,
                (handle_laser_explosion, update_xp_texts)
                    .run_if(in_state(AppState::InGame))
                    .run_if(not(in_state(GameState::Pause))),
            );
    }
}
