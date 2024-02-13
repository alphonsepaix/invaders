pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod ui;

use bevy::prelude::*;
use components::*;
use events::*;
use systems::*;
use ui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                spawn_player,
                spawn_shelters,
                spawn_aliens,
                play_main_music,
                add_resources,
            ),
        )
        .add_event::<PlayerHit>()
        .add_event::<AlienHit>()
        .add_event::<GameOver>()
        .add_systems(
            FixedUpdate,
            (move_player, restrict_player_movement)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (move_aliens, alien_reach_floor)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (move_lasers, despawn_lasers)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                player_shoot,
                aliens_shoot,
                check_for_collisions,
                shelter_hit,
                spawn_ufo,
                move_ufo,
                handle_player_hit,
                handle_alien_hit,
                handle_game_over,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnExit(AppState::InGame),
            (despawn_screen::<OnGameScreen>, reset_game_state),
        );
    }
}
