#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::game::components::*;
use crate::game::events::*;
use crate::game::resources::*;
use crate::game::ui::*;
use crate::settings::*;
use crate::*;
use bevy::app::AppExit;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use itertools::izip;
use rand::random;
use std::time::Duration;

pub fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        window.single_mut().visible = true;
    }
}

pub fn add_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ButtonHoveredSound(asset_server.load("audio/hovered.ogg")));
    commands.insert_resource(ButtonPressedSound(asset_server.load("audio/pressed.ogg")));

    commands.insert_resource(AlreadyPlayed(false));

    commands.insert_resource(TransitionTimer(Timer::new(
        Duration::from_secs(1),
        TimerMode::Repeating,
    )));

    let shoot = asset_server.load("audio/shoot.ogg");
    commands.insert_resource(ShootSound(shoot));
    let explosion = asset_server.load("audio/explosion.ogg");
    commands.insert_resource(ExplosionSound(explosion));
    let invader_killed = asset_server.load("audio/invaderkilled.ogg");
    commands.insert_resource(InvaderKilledSound(invader_killed));

    let invader_1 = asset_server.load("audio/fastinvader1.ogg");
    let invader_2 = asset_server.load("audio/fastinvader2.ogg");
    let invader_3 = asset_server.load("audio/fastinvader3.ogg");
    let invader_4 = asset_server.load("audio/fastinvader4.ogg");
    commands.insert_resource(AlienSounds(InvadersMovingSound::new(
        0,
        [invader_1, invader_2, invader_3, invader_4],
    )));

    commands.insert_resource(PlayerScore(0));

    commands.insert_resource(AlienTimer(Timer::from_seconds(
        ALIEN_TICK_DURATION,
        TimerMode::Repeating,
    )));
    commands.insert_resource(UfoTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    commands.insert_resource(AlienDirection {
        previous: EntityDirection::Left,
        next: EntityDirection::Left,
    });

    commands.insert_resource(LivesRemaining(3));
}

#[allow(dead_code)]
pub fn play_main_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    frames: Res<FrameCount>,
) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        let music = asset_server.load("audio/spaceinvaders.ogg");
        commands.spawn(AudioBundle {
            source: music,
            settings: PlaybackSettings::LOOP,
        });
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(window.width() / 2.0, FLOOR_HEIGHT, 0.0),
            ..default()
        },
        Player,
        OnGameScreen,
    ));
}

pub fn spawn_aliens(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
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
}

pub fn spawn_shelters(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let sprite = asset_server.load("sprites/shelter.png");

    let shelter_size = get_shelter_size();
    let space_between_shelters =
        (window.width() - NUM_SHELTERS as f32 * shelter_size.x) / (NUM_SHELTERS + 1) as f32;
    let height_below_shelter = 2.0 * FLOOR_HEIGHT + PLAYER_SIZE.y;
    let mut translation = Vec3::new(
        space_between_shelters + shelter_size.x / 2.0,
        height_below_shelter,
        0.0,
    );

    for _ in 0..NUM_SHELTERS {
        commands.spawn((
            SpriteBundle {
                texture: sprite.clone(),
                transform: Transform {
                    translation,
                    scale: Vec3::new(SHELTER_SCALE_FACTOR, SHELTER_SCALE_FACTOR, 0.0),
                    ..default()
                },
                ..default()
            },
            Shelter { armor: 100 },
            OnGameScreen,
        ));

        translation.x += space_between_shelters + shelter_size.x;
    }
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

pub fn player_shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    laser_query: Query<&Laser, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    shoot_sound: Res<ShootSound>,
) {
    if laser_query.get_single().is_ok() {
        // A laser shot by the player is still visible.
        return;
    }

    if keyboard_input.pressed(KeyCode::Space) {
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

pub fn move_lasers(mut lasers_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, Laser { direction, speed }) in lasers_query.iter_mut() {
        let movement = match direction {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            _ => panic!("Laser is going the wrong way!"),
        };
        transform.translation += movement * *speed * time.delta_seconds();
    }
}

pub fn despawn_lasers(
    mut commands: Commands,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    lasers_query.iter().for_each(|(entity, transform)| {
        let y_bottom = transform.translation.y - LASER_SIZE.y / 2.0;

        if y_bottom > window.height() || y_bottom < 0.0 {
            commands.entity(entity).despawn();
        }
    })
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut alien_hit_event_writer: EventWriter<AlienHit>,
    mut player_hit_event_writer: EventWriter<PlayerHit>,
    player_query: Query<&Transform, (With<Player>, Without<Laser>)>,
    aliens_query: Query<(Entity, &Transform, &Alien), Without<Laser>>,
    player_laser_query: Query<(Entity, &Transform), (With<Laser>, With<Player>)>,
    alien_lasers_query: Query<(Entity, &Transform), (With<Laser>, With<Alien>)>,
) {
    let half_player_height = PLAYER_SIZE.y / 2.0;
    let half_alien_height = ALIEN_SIZE.y / 2.0;
    let half_laser_height = LASER_SIZE.y / 2.0;

    // Check if an alien hit the player.
    if let Ok(player_transform) = player_query.get_single() {
        for (laser_entity, laser_transform) in alien_lasers_query.iter() {
            if player_transform
                .translation
                .distance(laser_transform.translation)
                < half_player_height + half_laser_height
            {
                commands.entity(laser_entity).despawn();
                player_hit_event_writer.send(PlayerHit);
            }
        }
    }

    // Check if player hit an alien.
    for (alien_entity, alien_transform, alien_type) in aliens_query.iter() {
        if let Ok((laser_entity, laser_transform)) = player_laser_query.get_single() {
            if alien_transform
                .translation
                .distance(laser_transform.translation)
                < half_alien_height + half_laser_height
            {
                commands.entity(laser_entity).despawn();
                alien_hit_event_writer.send(AlienHit {
                    alien_type: alien_type.clone(),
                    id: alien_entity,
                });
            }
        }
    }
}

pub fn shelter_hit(
    mut commands: Commands,
    mut shelters_query: Query<(Entity, &Transform, &mut Shelter)>,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
) {
    for (laser_entity, laser_transform) in lasers_query.iter() {
        for (shelter_entity, shelter_transform, mut shelter) in shelters_query.iter_mut() {
            if shelter_transform
                .translation
                .distance(laser_transform.translation)
                <= get_shelter_size().x / 2.0 + LASER_SIZE.x / 2.0
            {
                commands.entity(laser_entity).despawn();
                shelter.armor = shelter.armor.saturating_sub(5);
            }
            if shelter.armor == 0 {
                commands.entity(shelter_entity).despawn();
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

pub fn handle_player_hit(
    mut commands: Commands,
    mut player_hit_event_reader: EventReader<PlayerHit>,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<Entity, (With<Player>, Without<Laser>)>,
    explosion_sound: Res<ExplosionSound>,
    mut lives_remaining: ResMut<LivesRemaining>,
    mut next_state: ResMut<NextState<GameState>>,
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
            println!("Lives remaining: {}", lives_remaining.0);

            if lives_remaining.0 > 0 {
                next_state.set(GameState::Transition);
            } else {
                // Game over.
                game_over_event_writer.send(GameOver);
            }
        }
    }
}

pub fn handle_alien_hit(
    mut commands: Commands,
    mut alien_hit_event_reader: EventReader<AlienHit>,
    aliens_query: Query<&Alien, Without<Laser>>,
    invader_killed_sound: Res<InvaderKilledSound>,
    mut alien_timer: ResMut<AlienTimer>,
    mut score: ResMut<PlayerScore>,
) {
    if let Some(AlienHit { alien_type, id }) = alien_hit_event_reader.read().next() {
        // A mystery ship has a sound bundled with it, so we need to stop it
        // when it gets hit with a recursive despawn.
        commands.entity(*id).despawn_recursive();

        // Play an explosion sound when an alien dies.
        commands.spawn(AudioBundle {
            source: invader_killed_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        // Increase the player score.
        score.0 += alien_type.value();
        println!("Score: {}", score.0);

        // If there are less than 25 aliens remaining, increase their speed
        // every time anyone one of them is killed.
        if aliens_query.iter().count() < 25 {
            let current_duration = alien_timer.duration();
            let next_duration = current_duration.as_secs_f32() * 0.95;
            alien_timer.set_duration(Duration::from_secs_f32(next_duration));
        }
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_state: ResMut<NextState<AppState>>,
    mut already_played: ResMut<AlreadyPlayed>,
) {
    if game_over_event_reader.read().next().is_some() {
        next_state.set(AppState::Menu);
        already_played.0 = true;
        println!("Game over!");
    }
}

pub fn reset_game_state(
    mut score: ResMut<PlayerScore>,
    mut lives_remaining: ResMut<LivesRemaining>,
) {
    score.0 = 0;
    lives_remaining.0 = 3;
}

pub fn handle_input(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut alien_timer: ResMut<AlienTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if let GameState::Running = current_state.get() {
            next_state.set(GameState::Pause);
            alien_timer.pause();
        } else {
            next_state.set(GameState::Running);
            alien_timer.unpause();
        }
    }
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn reset_transition_timer(mut timer: ResMut<TransitionTimer>) {
    timer.reset();
}

pub fn transition_countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<TransitionTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Running);
    }
}
