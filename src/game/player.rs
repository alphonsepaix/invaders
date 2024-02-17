pub mod systems;

use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct PlayerHit;

#[derive(Component)]
pub struct Player;

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
                    .run_if(not(in_state(GameState::Pause))),
            )
            .add_systems(
                Update,
                (player_shoot, handle_player_hit)
                    .run_if(in_state(AppState::InGame))
                    .run_if(not(in_state(GameState::Pause))),
            );
    }
}
