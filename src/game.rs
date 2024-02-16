pub mod components;
pub mod resources;
pub mod systems;

pub mod aliens;
pub mod lasers;
pub mod player;
pub mod shelters;

use crate::game::aliens::AliensPlugin;
use crate::game::lasers::LasersPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::shelters::SheltersPlugin;
use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use components::*;
use systems::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    Transition,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionState {
    #[default]
    Unset,
    PlayerKilled,
    AliensKilled,
    GameOver,
}

#[derive(Event)]
pub struct GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (play_main_music, spawn_floor, add_resources),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(AliensPlugin)
        .add_plugins(LasersPlugin)
        .add_plugins(SheltersPlugin)
        .add_state::<GameState>()
        .add_event::<GameOver>()
        .add_systems(
            FixedUpdate,
            handle_game_over
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(OnEnter(GameState::Transition), reset_transition_timer)
        .add_systems(Update, transition_delay.run_if(in_state(AppState::InGame)))
        .add_systems(
            OnExit(AppState::InGame),
            (despawn_screen::<OnGameScreen>, reset_game_state),
        );
    }
}
