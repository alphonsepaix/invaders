pub mod systems;

use crate::game::components::Alien;
use crate::game::{GameState, TransitionState};
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct AlienHit {
    pub alien_type: Alien,
    pub id: Entity,
    pub position: Vec2,
}

pub struct AliensPlugin;

impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AlienHit>()
            .add_systems(OnEnter(AppState::InGame), spawn_aliens)
            .add_systems(
                FixedUpdate,
                (move_aliens, alien_reach_floor)
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (aliens_shoot, spawn_ufo, move_ufo, handle_alien_hit)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(TransitionState::AliensKilled), spawn_aliens);
    }
}
