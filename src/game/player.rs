pub mod systems;

use crate::game::{GameState, TransitionState};
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct PlayerHit;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHit>()
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                FixedUpdate,
                (move_player, restrict_player_movement)
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (player_shoot, handle_player_hit)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(TransitionState::PlayerKilled), spawn_player);
    }
}
