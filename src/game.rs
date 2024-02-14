pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use crate::ui::*;
use bevy::prelude::*;
use components::*;
use events::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (
                spawn_player,
                spawn_floor,
                spawn_shelters,
                spawn_aliens,
                play_main_music,
                add_resources,
            ),
        )
        .add_event::<PlayerHit>()
        .add_event::<AlienHit>()
        .add_event::<LaserExplosion>()
        .add_event::<GameOver>()
        .add_systems(Update, handle_input.run_if(in_state(AppState::InGame)))
        .add_systems(
            FixedUpdate,
            (move_player, restrict_player_movement)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            FixedUpdate,
            (move_aliens, alien_reach_floor)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            FixedUpdate,
            (move_lasers, despawn_lasers)
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (
                player_shoot,
                aliens_shoot,
                check_for_collisions,
                shelter_hit,
                spawn_ufo,
                move_ufo,
            )
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (
                handle_player_hit,
                handle_alien_hit,
                handle_game_over,
                handle_laser_explosion,
            ),
        )
        .add_systems(OnEnter(GameState::Pause), pause_setup)
        .add_systems(OnExit(GameState::Pause), despawn_screen::<OnPauseScreen>)
        .add_systems(OnEnter(GameState::Transition), reset_transition_timer)
        .add_systems(Update, transition_delay.run_if(in_state(AppState::InGame)))
        .add_systems(OnExit(TransitionState::PlayerKilled), spawn_player)
        .add_systems(OnExit(TransitionState::AliensKilled), spawn_aliens)
        .add_systems(
            OnExit(AppState::InGame),
            (despawn_screen::<OnGameScreen>, reset_game_state),
        );
    }
}
