pub mod systems;

use crate::despawn_screen;
use crate::game::GameState;
use bevy::prelude::*;
use systems::*;

#[derive(Component)]
pub struct OnPauseScreen;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pause), pause_setup)
            .add_systems(OnExit(GameState::Pause), despawn_screen::<OnPauseScreen>);
    }
}
