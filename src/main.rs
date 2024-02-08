#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use itertools::izip;
use rand::random;
use std::time::Duration;

const WINDOW_VISIBLE_DELAY: u32 = 3;

const PLAYER_SIZE: Vec2 = Vec2::new(60.0, 30.0);
const PLAYER_SPEED: f32 = 300.0;

const NUM_SHELTERS: usize = 4;
const SHELTER_SIZE: Vec2 = Vec2::new(40.0, 20.0);
// The size before scale.
const SHELTER_SCALE_FACTOR: f32 = 2.5;

const ALIENS_PER_LINE: usize = 11;
const SPACE_BETWEEN_ALIENS: Vec2 = Vec2::new(20.0, 16.0);
const MARGIN: f32 = 80.0;
const ALIEN_SHOOT_PROB: f32 = 1.0 / 40.0 / 60.0;
const ALIEN_SIZE: Vec2 = Vec2::new(40.0, 30.0);
const YELLOW_ALIEN_VALUE: u32 = 30;
const GREEN_ALIEN_VALUE: u32 = 20;
const RED_ALIEN_VALUE: u32 = 10;
const ALIEN_TICK_DURATION: f32 = 0.8;

const UFO_VALUE: u32 = 300;
const UFO_SPAWN_PROB: f32 = 1.0 / 30.0;
const UFO_SIZE: Vec2 = Vec2::new(80.0, 30.0);
const UFO_SPEED: f32 = 150.0;

const LASER_SIZE: Vec2 = Vec2::new(5.0, 15.0);
const PLAYER_LASER_SPEED: f32 = 600.0;
const ALIEN_LASER_SPEED: f32 = 300.0;
const MAX_ALIEN_LASERS: usize = 4;

const HEIGHT_BELOW_PLAYER: f32 = 60.0;

fn get_window_resolution() -> Vec2 {
    let width = 2.0 * MARGIN
        + ALIENS_PER_LINE as f32 * ALIEN_SIZE.x
        + (ALIENS_PER_LINE - 1) as f32 * SPACE_BETWEEN_ALIENS.x;
    let height = 600.0;
    Vec2::new(width, height)
}

fn get_shelter_size() -> Vec2 {
    SHELTER_SIZE * SHELTER_SCALE_FACTOR
}

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
        .add_systems(Update, (make_visible, play_main_music).chain())
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
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Clone, Component)]
enum Alien {
    Yellow,
    Green,
    Red,
    Ufo,
}

impl Alien {
    fn color(&self) -> Color {
        match self {
            Alien::Yellow => Color::YELLOW,
            Alien::Green => Color::GREEN,
            Alien::Red => Color::RED,
            // Doesn't matter.
            Alien::Ufo => Color::PURPLE,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Alien::Yellow => YELLOW_ALIEN_VALUE,
            Alien::Green => GREEN_ALIEN_VALUE,
            Alien::Red => RED_ALIEN_VALUE,
            Alien::Ufo => UFO_VALUE,
        }
    }
}

#[derive(Component)]
struct Ufo(EntityDirection);

#[derive(Component)]
struct Shelter {
    armor: u32,
}

#[derive(Resource)]
struct PlayerScore(u32);

#[derive(Clone, PartialEq)]
enum EntityDirection {
    Up,
    Down,
    Left,
    Right,
}

impl EntityDirection {
    fn mask(&self) -> Vec3 {
        match self {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            EntityDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            EntityDirection::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Component)]
struct Laser {
    direction: EntityDirection,
    speed: f32,
}

#[derive(Resource)]
struct ShootSound(Handle<AudioSource>);

#[derive(Resource)]
struct ExplosionSound(Handle<AudioSource>);

#[derive(Resource)]
struct InvaderKilledSound(Handle<AudioSource>);

struct InvadersMovingSound {
    index: usize,
    sounds: [Handle<AudioSource>; 4],
}

impl InvadersMovingSound {
    fn get(&mut self) -> Handle<AudioSource> {
        let source = self.sounds[self.index].clone();
        self.index = (self.index + 1) % self.sounds.len();
        source
    }
}

#[derive(Deref, DerefMut, Resource)]
struct AlienSounds(InvadersMovingSound);

#[derive(Component)]
struct MainMusic;

#[derive(Deref, DerefMut, Resource)]
struct AlienTimer(Timer);

#[derive(Deref, DerefMut, Resource)]
struct UfoTimer(Timer);

#[derive(Resource)]
struct AlienDirection {
    previous: EntityDirection,
    next: EntityDirection,
}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        window.single_mut().visible = true;
    }
}

fn add_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    commands.insert_resource(AlienSounds(InvadersMovingSound {
        index: 0,
        sounds: [invader_1, invader_2, invader_3, invader_4],
    }));

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
}

fn play_main_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    frames: Res<FrameCount>,
) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        let music = asset_server.load("audio/spaceinvaders.ogg");
        commands.spawn((
            AudioBundle {
                source: music,
                settings: PlaybackSettings::LOOP,
            },
            MainMusic,
        ));
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(window.width() / 2.0, HEIGHT_BELOW_PLAYER, 0.0),
            ..default()
        },
        Player,
    ));
}

fn spawn_aliens(
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

fn spawn_shelters(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let sprite = asset_server.load("sprites/shelter.png");

    let shelter_size = get_shelter_size();
    let space_between_shelters =
        (window.width() - NUM_SHELTERS as f32 * shelter_size.x) / (NUM_SHELTERS + 1) as f32;
    let height_below_shelter = 2.0 * HEIGHT_BELOW_PLAYER + PLAYER_SIZE.y;
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
        ));

        translation.x += space_between_shelters + shelter_size.x;
    }
}

fn move_player(
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

fn restrict_player_movement(
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

fn move_aliens(
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

                // Play the sound of the aliens moving.
                commands.spawn(AudioBundle {
                    source: sounds.get(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}

fn player_shoot(
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

    if keyboard_input.just_pressed(KeyCode::Space) {
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
            ));
            commands.spawn(AudioBundle {
                source: shoot_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

fn aliens_shoot(
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
            ));

            laser_count += 1;
        }
    }
}

fn move_lasers(mut lasers_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, Laser { direction, speed }) in lasers_query.iter_mut() {
        let movement = match direction {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            _ => panic!("Laser is going the wrong way!"),
        };
        transform.translation += movement * *speed * time.delta_seconds();
    }
}

fn despawn_lasers(
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

fn check_for_collisions(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Laser>)>,
    aliens_query: Query<(Entity, &Transform, &Alien), Without<Laser>>,
    player_laser_query: Query<(Entity, &Transform), (With<Laser>, With<Player>)>,
    alien_lasers_query: Query<(Entity, &Transform), (With<Laser>, With<Alien>)>,
    explosion_sound: Res<ExplosionSound>,
    invader_killed_sound: Res<InvaderKilledSound>,
    mut score: ResMut<PlayerScore>,
) {
    let half_player_height = PLAYER_SIZE.y / 2.0;
    let half_alien_height = ALIEN_SIZE.y / 2.0;
    let half_laser_height = LASER_SIZE.y / 2.0;

    // Check if an alien hit the player.
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (laser_entity, laser_transform) in alien_lasers_query.iter() {
            if player_transform
                .translation
                .distance(laser_transform.translation)
                < half_player_height + half_laser_height
            {
                commands.entity(player_entity).despawn();
                commands.entity(laser_entity).despawn();

                // Play an explosion sound when the player dies.
                commands.spawn(AudioBundle {
                    source: explosion_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
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
                commands.entity(alien_entity).despawn_recursive();
                commands.entity(laser_entity).despawn();

                // Play an explosion sound when an alien dies.
                commands.spawn(AudioBundle {
                    source: invader_killed_sound.0.clone(),
                    settings: PlaybackSettings::ONCE,
                });

                score.0 += alien_type.value();
                println!("Score: {}", score.0);
            }
        }
    }
}

fn shelter_hit(
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

fn spawn_ufo(
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
                        texture: asset_server.load("sprites/red.png"),
                        transform: Transform {
                            translation: spawn_position,
                            scale: Vec3::new(2.0, 1.0, 0.0),
                            ..default()
                        },
                        ..default()
                    },
                    Ufo(direction),
                    Alien::Ufo,
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

fn move_ufo(
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
