pub mod systems;

use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use systems::*;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct OnMenuScreen;

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
