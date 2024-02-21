pub mod game;
pub mod resources;
pub mod settings;
pub mod ui;

use crate::game::{EntityDirection, GameState};
use crate::resources::*;
use crate::settings::*;
use bevy::app::AppExit;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Pause,
    InGame,
}

#[derive(Component)]
pub struct MainMusic;

pub fn get_window_resolution() -> Vec2 {
    let width = 2.0 * MARGIN
        + ALIENS_PER_LINE as f32 * ALIEN_SIZE.x
        + (ALIENS_PER_LINE - 1) as f32 * SPACE_BETWEEN_ALIENS.x;
    let height = 600.0;
    Vec2::new(width, height)
}

pub fn despawn_screen<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn add_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));

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
    commands.insert_resource(BestScore(0));

    commands.insert_resource(AlienTimer(Timer::from_seconds(
        ALIEN_TICK_DURATION,
        TimerMode::Repeating,
    )));
    commands.insert_resource(AlienTimerDuration(Duration::from_secs_f32(
        ALIEN_TICK_DURATION,
    )));
    commands.insert_resource(UfoTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    commands.insert_resource(AlienDirection {
        previous: EntityDirection::Left,
        next: EntityDirection::Left,
    });

    commands.insert_resource(LivesRemaining(3));
}

pub fn play_main_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    frames: Res<FrameCount>,
) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        let music = asset_server.load("audio/music.ogg");
        commands.spawn((
            AudioBundle {
                source: music,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::Relative(VolumeLevel::new(0.5)),
                    ..default()
                },
            },
            MainMusic,
        ));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn handle_input(
    mut app_exit_event_writer: EventWriter<AppExit>,
    sinks_query: Query<&AudioSink>,
    keyboard_input: Res<Input<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    current_game_state: Res<State<GameState>>,
    mut alien_timer: ResMut<AlienTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let AppState::InGame = current_app_state.get() {
        // Pause or unpause the game if the user is currently playing.
        if keyboard_input.just_pressed(KeyCode::P) {
            let (next_game_state, toggle) = match current_game_state.get() {
                GameState::Running => {
                    alien_timer.pause();
                    (GameState::Pause, true)
                }
                GameState::Pause => {
                    alien_timer.unpause();
                    (GameState::Running, true)
                }
                other => (*other, false),
            };
            next_state.set(next_game_state);
            if toggle {
                for sink in sinks_query.iter() {
                    // Toggle all sounds.
                    sink.toggle();
                }
            }
        }
    }

    // Quit the app.
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_event_writer.send(AppExit);
    }
}
