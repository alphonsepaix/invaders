#![allow(clippy::type_complexity)]

pub mod systems;

use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), menu_setup)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMenuScreen>);
    }
}
