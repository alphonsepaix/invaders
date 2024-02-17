use crate::game::aliens::{Alien, AlienHit, Ufo, XpTimer};
use crate::game::lasers::Laser;
use crate::game::transition::TransitionState;
use crate::game::{EntityDirection, GameOver, GameState, OnGameScreen};
use crate::get_window_resolution;
use crate::resources::{
    AlienDirection, AlienSounds, AlienTimer, InvaderKilledSound, LivesRemaining, PlayerScore,
    UfoTimer,
};
use crate::settings::{
    ALIENS_PER_LINE, ALIEN_LASER_SPEED, ALIEN_SHOOT_PROB, ALIEN_SIZE, ALIEN_TICK_DURATION,
    FLOOR_HEIGHT, LASER_SIZE, MARGIN, MAX_ALIEN_LASERS, SPACE_BETWEEN_ALIENS, UFO_SIZE,
    UFO_SPAWN_PROB, UFO_SPEED, XP_GAIN_DURATION,
};
use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::Vec3;
use bevy::prelude::{
    default, Color, Commands, Entity, EventReader, EventWriter, Image, NextState, PositionType,
    Query, Res, ResMut, Sprite, SpriteBundle, Style, TextAlignment, TextBundle, TextStyle, Time,
    Timer, TimerMode, Transform, Val, Window, With, Without,
};
use bevy::window::PrimaryWindow;
use itertools::izip;
use rand::random;
use std::time::Duration;

pub fn spawn_aliens(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut alien_timer: ResMut<AlienTimer>,
) {
    let window = window_query.single();

    let sprites: [Handle<Image>; 3] = [
        asset_server.load("sprites/yellow.png"),
        asset_server.load("sprites/green.png"),
        asset_server.load("sprites/red.png"),
    ];
    let lines = [1_usize, 2, 2];
    let alien_types = [Alien::Yellow, Alien::Green, Alien::Red];

    let mut direction = Vec3::new(SPACE_BETWEEN_ALIENS.x + ALIEN_SIZE.x, 0.0, 0.0);
    let mut translation = Vec3::new(MARGIN + ALIEN_SIZE.x / 2.0, window.height() - MARGIN, 0.0);

    for (sprite, lines, alien_type) in izip!(sprites, lines, alien_types) {
        for _ in 0..lines {
            for j in 0..11 {
                commands.spawn((
                    SpriteBundle {
                        texture: sprite.clone(),
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    alien_type.clone(),
                    OnGameScreen,
                ));
                if j != ALIENS_PER_LINE - 1 {
                    translation += direction;
                }
            }
            direction.x *= -1.0;
            translation.y -= SPACE_BETWEEN_ALIENS.y + ALIEN_SIZE.y;
        }
    }

    // Reset the timer.
    alien_timer.set_duration(Duration::from_secs_f32(ALIEN_TICK_DURATION));
}

pub fn move_aliens(
    mut commands: Commands,
    mut aliens_query: Query<&mut Transform, (With<Alien>, Without<Laser>, Without<Ufo>)>,
    time: Res<Time>,
    mut alien_direction: ResMut<AlienDirection>,
    mut sounds: ResMut<AlienSounds>,
    mut timer: ResMut<AlienTimer>,
) {
    if timer.tick(time.delta()).just_finished() {
        let mut translation = Vec3::new(ALIEN_SIZE.x / 4.0, ALIEN_SIZE.y / 2.0, 0.0);
        let next = alien_direction.next.clone();
        translation *= next.mask();

        // Move each alien.
        aliens_query.iter_mut().for_each(|mut transform| {
            transform.translation += translation;
        });

        // Play the sound of the aliens moving, if any.
        if aliens_query.iter().count() > 0 {
            commands.spawn(AudioBundle {
                source: sounds.get(),
                settings: PlaybackSettings::ONCE,
            });
        }

        if let EntityDirection::Down = alien_direction.next {
            // If aliens were moving down we change their direction before the next call.
            alien_direction.next = match alien_direction.previous {
                EntityDirection::Left => EntityDirection::Right,
                EntityDirection::Right => EntityDirection::Left,
                _ => panic!("Previous alien direction should be either left or right."),
            };
            alien_direction.previous = alien_direction.next.clone();
        } else {
            // Check if an alien hit a side.
            let resolution = get_window_resolution();
            let half_alien_width = ALIEN_SIZE.x / 2.0;

            if aliens_query.iter().any(|transform| {
                let x = transform.translation.x;
                x <= half_alien_width || x >= resolution.x - half_alien_width
            }) {
                alien_direction.next = EntityDirection::Down;

                // Decrease the duration of the timer to make aliens move faster.
                let current_tick = timer.duration().as_secs_f32();
                timer.set_duration(Duration::from_secs_f32(current_tick / 1.1));
            }
        }
    }
}

pub fn aliens_shoot(
    mut commands: Commands,
    aliens_query: Query<(&Transform, &Alien), Without<Ufo>>,
    lasers_query: Query<&Laser, With<Alien>>,
) {
    let mut laser_count = lasers_query.iter().count();

    for (alien_transform, alien_type) in aliens_query.iter() {
        if laser_count == MAX_ALIEN_LASERS {
            break;
        }

        if random::<f32>() < ALIEN_SHOOT_PROB {
            let translation = alien_transform.translation;
            let half_alien_height = ALIEN_SIZE.y / 2.0;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: alien_type.color(),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            translation.x,
                            translation.y - half_alien_height,
                            0.0,
                        ),
                        scale: LASER_SIZE.extend(0.0),
                        ..default()
                    },
                    ..default()
                },
                Laser {
                    direction: EntityDirection::Down,
                    speed: ALIEN_LASER_SPEED,
                },
                alien_type.clone(),
                OnGameScreen,
            ));

            laser_count += 1;
        }
    }
}

pub fn alien_reach_floor(
    mut game_over_event_writer: EventWriter<GameOver>,
    aliens_query: Query<&Transform, (With<Alien>, Without<Laser>)>,
) {
    for alien_transform in aliens_query.iter() {
        if alien_transform.translation.y < FLOOR_HEIGHT {
            game_over_event_writer.send(GameOver);
        }
    }
}

pub fn handle_alien_hit(
    mut commands: Commands,
    mut alien_hit_event_reader: EventReader<AlienHit>,
    aliens_query: Query<&Alien, Without<Laser>>,
    asset_server: Res<AssetServer>,
    invader_killed_sound: Res<InvaderKilledSound>,
    mut alien_timer: ResMut<AlienTimer>,
    mut lives_remaining: ResMut<LivesRemaining>,
    mut score: ResMut<PlayerScore>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
) {
    if let Some(AlienHit {
        alien_type,
        id,
        position,
    }) = alien_hit_event_reader.read().next()
    {
        // A mystery ship has a sound bundled with it, so we need to stop it
        // when it gets hit with a recursive despawn.
        if let Some(entity_commands) = commands.get_entity(*id) {
            entity_commands.despawn_recursive();

            // Play an explosion sound when an alien dies.
            commands.spawn(AudioBundle {
                source: invader_killed_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });

            // Increase the player score.
            score.0 += alien_type.value();

            // Show the alien value.
            let text = format!("+{}XP", alien_type.value());
            let font = asset_server.load("fonts/font.ttf");
            commands.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        color: Color::WHITE,
                        font,
                        font_size: 20.0,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.x),
                    bottom: Val::Px(position.y),
                    ..default()
                }),
                XpTimer(Timer::from_seconds(XP_GAIN_DURATION, TimerMode::Once)),
                OnGameScreen,
            ));

            let aliens_remaining = aliens_query.iter().count() - 1;
            if aliens_remaining == 0 {
                next_game_state.set(GameState::Transition);
                next_transition_state.set(TransitionState::AliensKilled);
                if lives_remaining.0 < 5 {
                    lives_remaining.0 += 1;
                }
            } else if aliens_remaining < 25 {
                // If there are less than 25 aliens remaining, increase their speed
                // every time anyone one of them is killed.
                let current_duration = alien_timer.duration();
                let next_duration = current_duration.as_secs_f32() * 0.95;
                alien_timer.set_duration(Duration::from_secs_f32(next_duration));
            }
        }
    }
}

pub fn spawn_ufo(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ufo_query: Query<&Ufo>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut ufo_timer: ResMut<UfoTimer>,
) {
    if ufo_query.get_single().is_ok() {
        // A mystery ship is already on screen.
        return;
    }

    if ufo_timer.tick(time.delta()).just_finished() {
        // Spawn a mystery ship.
        if random::<f32>() < UFO_SPAWN_PROB {
            let window = window_query.single();

            let y = window.height() - UFO_SIZE.y;
            let (direction, spawn_position) = if random::<f32>() > 0.5 {
                let dir = EntityDirection::Left;
                // Spawn at the right edge of the window (with a little margin).
                let spawn = Vec3::new(window.width() + UFO_SIZE.x, y, 0.0);
                (dir, spawn)
            } else {
                let dir = EntityDirection::Right;
                // Spawn at the left edge of the window.
                let spawn = Vec3::new(-UFO_SIZE.x, y, 0.0);
                (dir, spawn)
            };

            commands
                .spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/ufo.png"),
                        transform: Transform {
                            translation: spawn_position,
                            ..default()
                        },
                        ..default()
                    },
                    Ufo(direction),
                    Alien::Ufo,
                    OnGameScreen,
                ))
                .with_children(|parent| {
                    parent.spawn(AudioBundle {
                        source: asset_server.load("audio/ufo_highpitch.ogg"),
                        settings: PlaybackSettings::LOOP,
                    });
                });
        }
    }
}

pub fn move_ufo(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ufo_query: Query<(Entity, &mut Transform, &Ufo)>,
    time: Res<Time>,
) {
    if let Ok((ufo_entity, mut transform, Ufo(direction))) = ufo_query.get_single_mut() {
        let window = window_query.single();

        transform.translation += direction.mask() * UFO_SPEED * time.delta_seconds();

        let x = transform.translation.x;
        // Add a little margin, so it does not get despawn immediately.
        let margin = 10.0;
        if x >= window.width() + UFO_SIZE.x + margin || x <= -(UFO_SIZE.x + margin) {
            commands.entity(ufo_entity).despawn_recursive();
        }
    }
}
