pub mod systems;

use crate::game::GameState;
use crate::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use systems::*;

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                spawn_scoreboard,
                spawn_remaining_lives,
                spawn_remaining_aliens,
            ),
        )
        .add_systems(
            Update,
            (
                update_scoreboard,
                update_remaining_lives,
                update_remaining_aliens,
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}
