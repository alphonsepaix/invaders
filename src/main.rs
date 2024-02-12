use bevy::prelude::*;
use bevy::window::close_on_esc;
use invaders::game::events::*;
use invaders::game::systems::*;
use invaders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Invaders".into(),
                resolution: get_window_resolution().into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..default()
                },
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                add_resources,
                spawn_camera,
                spawn_player,
                spawn_aliens,
                spawn_shelters,
            ),
        )
        .add_event::<PlayerHit>()
        .add_event::<AlienHit>()
        .add_systems(
            Update,
            (make_visible, /* play_main_music, */ close_on_esc).chain(),
        )
        .add_systems(FixedUpdate, (move_player, restrict_player_movement).chain())
        .add_systems(FixedUpdate, move_aliens)
        .add_systems(FixedUpdate, (move_lasers, despawn_lasers).chain())
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
            ),
        )
        .run();
}
