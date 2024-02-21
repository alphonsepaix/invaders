pub mod systems;

pub mod aliens;
pub mod lasers;
pub mod player;
pub mod shelters;
pub mod transition;

use crate::game::aliens::AliensPlugin;
use crate::game::lasers::LasersPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::shelters::SheltersPlugin;
use crate::game::transition::TransitionPlugin;
use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use systems::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    Transition,
}

#[derive(Event)]
pub struct GameOver;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Clone, Debug, PartialEq)]
pub enum EntityDirection {
    Up,
    Down,
    Left,
    Right,
}

impl EntityDirection {
    pub fn mask(&self) -> Vec3 {
        match self {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            EntityDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            EntityDirection::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (reset_game_state, spawn_floor))
            .add_plugins(PlayerPlugin)
            .add_plugins(AliensPlugin)
            .add_plugins(LasersPlugin)
            .add_plugins(SheltersPlugin)
            .add_plugins(TransitionPlugin)
            .add_state::<GameState>()
            .add_event::<GameOver>()
            .add_systems(
                FixedUpdate,
                handle_game_over
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_screen::<OnGameScreen>);
    }
}
