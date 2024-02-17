pub mod systems;

use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionState {
    #[default]
    Unset,
    PlayerKilled,
    AliensKilled,
    GameOver,
}

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TransitionState>()
            .add_systems(OnEnter(GameState::Transition), reset_transition_timer)
            .add_systems(
                Update,
                transition_delay
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Transition)),
            );
    }
}
