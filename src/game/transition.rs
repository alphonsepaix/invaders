pub mod systems;

use crate::game::aliens::systems::spawn_aliens;
use crate::game::player::systems::spawn_player;
use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionState {
    #[default]
    Disabled,
    SpawnPlayer,
    AliensKilled,
}

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TransitionState>()
            .add_systems(OnEnter(GameState::Transition), transition_setup)
            .add_systems(
                Update,
                set_transition_state
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Transition)),
            )
            .add_systems(OnEnter(TransitionState::SpawnPlayer), spawn_player)
            .add_systems(OnEnter(TransitionState::AliensKilled), spawn_aliens);
    }
}
