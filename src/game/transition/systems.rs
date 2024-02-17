use crate::game::lasers::Laser;
use crate::game::transition::TransitionState;
use crate::game::GameState;
use crate::resources::TransitionTimer;
use crate::AppState;
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, State, Time, With};

pub fn reset_transition_timer(mut timer: ResMut<TransitionTimer>) {
    timer.reset();
}

pub fn transition_delay(
    mut commands: Commands,
    lasers_query: Query<Entity, With<Laser>>,
    time: Res<Time>,
    current_state: Res<State<TransitionState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    mut timer: ResMut<TransitionTimer>,
) {
    if timer.tick(time.delta()).finished() {
        match current_state.get() {
            TransitionState::PlayerKilled | TransitionState::AliensKilled => {
                next_game_state.set(GameState::Running);
                // Despawn all lasers.
                for entity in lasers_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            TransitionState::GameOver => next_app_state.set(AppState::Menu),
            TransitionState::Unset => (),
        }
        // Maybe a state is not the right structure here.
        // We could use a resource instead.
        next_transition_state.set(TransitionState::Unset);
    }
}
