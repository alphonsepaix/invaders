use crate::game::aliens::{Alien, Ufo};
use crate::game::lasers::Laser;
use crate::game::player::Player;
use crate::game::transition::TransitionState;
use crate::game::GameState;
use crate::resources::{LivesRemaining, TransitionTimer};
use crate::AppState;
use bevy::prelude::*;

pub fn transition_setup(
    mut commands: Commands,
    lasers: Query<Entity, With<Laser>>,
    mut next_transition_action: ResMut<NextState<TransitionState>>,
    mut timer: ResMut<TransitionTimer>,
) {
    // Reset the transition state.
    next_transition_action.set(TransitionState::Disabled);
    // Despawn all lasers.
    lasers.iter().for_each(|id| {
        commands.entity(id).despawn();
    });
    timer.reset();
}

pub fn set_transition_state(
    aliens_query: Query<&Alien, (Without<Laser>, Without<Ufo>)>,
    player_query: Query<&Player, Without<Laser>>,
    time: Res<Time>,
    remaining_lives: Res<LivesRemaining>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    mut timer: ResMut<TransitionTimer>,
) {
    let mut resume_game = true;
    if timer.tick(time.delta()).finished() {
        if aliens_query.is_empty() {
            next_transition_state.set(TransitionState::AliensKilled);
        }
        if player_query.is_empty() {
            if remaining_lives.0 == 0 {
                next_app_state.set(AppState::Menu);
                resume_game = false;
            } else {
                next_transition_state.set(TransitionState::SpawnPlayer);
            }
        }
        if resume_game {
            next_game_state.set(GameState::Running);
        } else {
            next_app_state.set(AppState::Menu);
        }
    }
}
