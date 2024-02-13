use bevy::prelude::*;
use events::*;
use systems::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct PlayerPlugin;

pub struct AliensPlugin;

pub struct GamePlugin;

pub struct WorldPlugin;

pub struct LasersPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, (move_player, restrict_player_movement).chain())
            .add_event::<PlayerHit>();
    }
}

impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_aliens)
            .add_systems(FixedUpdate, (move_aliens, alien_reach_floor).chain())
            .add_event::<AlienHit>();
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
            ),
        )
        .add_event::<GameOver>();
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_resources, spawn_camera, spawn_shelters))
            .add_systems(Update, play_main_music);
    }
}

impl Plugin for LasersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (move_lasers, despawn_lasers).chain());
    }
}
