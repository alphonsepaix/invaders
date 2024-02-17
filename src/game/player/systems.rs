use crate::game::lasers::Laser;
use crate::game::player::{Player, PlayerHit};
use crate::game::{EntityDirection, GameOver, GameState, OnGameScreen};
use crate::resources::{ExplosionSound, LivesRemaining, ShootSound};
use crate::settings::{
    FLOOR_HEIGHT, FLOOR_THICKNESS, LASER_SIZE, PLAYER_LASER_SPEED, PLAYER_SIZE, PLAYER_SPEED,
};
use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{
    default, Color, Commands, Entity, EventReader, EventWriter, KeyCode, NextState, Query, Res,
    ResMut, Sprite, SpriteBundle, Time, Transform, Window, With, Without,
};
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let y_pos = FLOOR_HEIGHT + PLAYER_SIZE.y / 2.0 + FLOOR_THICKNESS / 2.0;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(window.width() / 2.0, y_pos, 0.0),
            ..default()
        },
        Player,
        OnGameScreen,
    ));
}

pub fn move_player(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Laser>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        movement.x = -1.0;
    } else if keyboard_input.pressed(KeyCode::I) || keyboard_input.pressed(KeyCode::Right) {
        movement.x = 1.0;
    }

    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation += movement * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn restrict_player_movement(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Laser>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let half_player_width = PLAYER_SIZE.x / 2.0;
    let x_min = half_player_width;
    let x_max = window.width() - half_player_width;

    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
    }
}

pub fn player_shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    laser_query: Query<&Laser, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    shoot_sound: Res<ShootSound>,
) {
    if laser_query.get_single().is_err() && keyboard_input.pressed(KeyCode::Space) {
        if let Ok(player_transform) = player_query.get_single() {
            let translation = player_transform.translation;
            let half_player_height = PLAYER_SIZE.x / 2.0;
            // Spawn a new laser shot by the player.
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::CYAN,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            translation.x,
                            translation.y + half_player_height,
                            0.0,
                        ),
                        scale: LASER_SIZE.extend(0.0),
                        ..default()
                    },
                    ..default()
                },
                Laser {
                    direction: EntityDirection::Up,
                    speed: PLAYER_LASER_SPEED,
                },
                Player,
                OnGameScreen,
            ));
            commands.spawn(AudioBundle {
                source: shoot_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn handle_player_hit(
    mut commands: Commands,
    mut player_hit_event_reader: EventReader<PlayerHit>,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<Entity, (With<Player>, Without<Laser>)>,
    explosion_sound: Res<ExplosionSound>,
    mut lives_remaining: ResMut<LivesRemaining>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if player_hit_event_reader.read().next().is_some() {
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(player_entity).despawn();

            // Play an explosion sound when the player dies.
            commands.spawn(AudioBundle {
                source: explosion_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });

            // Decrease the number of lives remaining.
            lives_remaining.0 = lives_remaining.0.saturating_sub(1);

            if lives_remaining.0 > 0 {
                next_game_state.set(GameState::Transition);
            } else {
                // Game over.
                game_over_event_writer.send(GameOver);
            }
        }
    }
}
