use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SIZE: Vec2 = Vec2::new(60.0, 30.0);
const PLAYER_SPEED: f32 = 400.0;
const LASER_SIZE: Vec2 = Vec2::new(5.0, 15.0);
const LASER_SPEED: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (add_resources, spawn_camera, spawn_player, play_main_music),
        )
        .add_systems(FixedUpdate, (move_player, restrict_player_movement).chain())
        .add_systems(FixedUpdate, (move_lasers, despawn_lasers).chain())
        .add_systems(FixedUpdate, player_shoot)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(PartialEq)]
enum LaserDirection {
    Up,
    Down,
}

#[derive(Component)]
struct Laser {
    direction: LaserDirection,
}

#[derive(Resource)]
struct ShootSound(Handle<AudioSource>);

#[derive(Component)]
struct MainMusic;

fn add_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shoot = asset_server.load("audio/shoot.ogg");
    commands.insert_resource(ShootSound(shoot));
}

fn play_main_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music = asset_server.load("audio/spaceinvaders.ogg");
    commands.spawn((
        AudioBundle {
            source: music,
            settings: PlaybackSettings::LOOP,
        },
        MainMusic,
    ));
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
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            transform: Transform::from_xyz(window.width() / 2.0, 30.0, 0.0),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
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
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_width = PLAYER_SIZE.x / 2.0;
    let x_min = half_player_width;
    let x_max = window.width() - half_player_width;

    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
    }
}

fn player_shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    lasers_query: Query<&Laser>,
    keyboard_input: Res<Input<KeyCode>>,
    shoot_sound: Res<ShootSound>,
) {
    if lasers_query
        .iter()
        .any(|Laser { direction }| *direction == LaserDirection::Up)
    {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Shooting!");
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
                    direction: LaserDirection::Up,
                },
            ));
            commands.spawn(AudioBundle {
                source: shoot_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

fn move_lasers(mut lasers_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, Laser { direction }) in lasers_query.iter_mut() {
        let movement = match direction {
            LaserDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            LaserDirection::Down => Vec3::new(0.0, -1.0, 0.0),
        };
        transform.translation += movement * LASER_SPEED * time.delta_seconds();
    }
}

fn despawn_lasers(
    mut commands: Commands,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    lasers_query.iter().for_each(|(entity, transform)| {
        let y_bottom = transform.translation.y - LASER_SIZE.y / 2.0;

        if y_bottom > window.height() || y_bottom < 0.0 {
            commands.entity(entity).despawn();
        }
    })
}
