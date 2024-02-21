pub mod systems;

use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Component)]
pub struct Shelter {
    pub armor: u32,
}

#[derive(Component)]
pub struct ShelterArmorText(pub Entity);

pub struct SheltersPlugin;

impl Plugin for SheltersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_shelters)
            .add_systems(
                FixedUpdate,
                shelter_hit
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            );
    }
}
