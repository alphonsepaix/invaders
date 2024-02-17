#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::game::resources::*;
use crate::game::{EntityDirection, GameOver, GameState, OnGameScreen, TransitionState};
use crate::settings::*;
use crate::*;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;
use crate::game::lasers::Laser;

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

pub fn spawn_floor(mut commands: Commands) {
    let window_width = get_window_resolution().x;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(window_width / 2.0, FLOOR_HEIGHT, 0.0),
                scale: Vec3::new(window_width, FLOOR_THICKNESS, 0.0),
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    ));
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    mut already_played: ResMut<AlreadyPlayed>,
) {
    if game_over_event_reader.read().next().is_some() {
        next_game_state.set(GameState::Transition);
        next_transition_state.set(TransitionState::GameOver);
        already_played.0 = true;
    }
}

pub fn reset_game_state(
    mut score: ResMut<PlayerScore>,
    mut lives_remaining: ResMut<LivesRemaining>,
) {
    score.0 = 0;
    lives_remaining.0 = 3;
}

pub fn reset_transition_timer(mut timer: ResMut<TransitionTimer>) {
    timer.reset();
}

pub fn transition_delay(
    mut commands: Commands,
    lasers_query: Query<Entity, With<Laser>>,
    time: Res<Time>,
    current_state: Res<State<TransitionState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
    mut timer: ResMut<TransitionTimer>,
) {
    if timer.tick(time.delta()).finished() {
        match current_state.get() {
            TransitionState::PlayerKilled | TransitionState::AliensKilled => {
                next_game_state.set(GameState::Running);
                // Despawn all lasers.
                for entity in lasers_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            TransitionState::GameOver => next_app_state.set(AppState::Menu),
            TransitionState::Unset => (),
        }
        // Maybe a state is not the right structure here.
        // We could use a resource instead.
        next_transition_state.set(TransitionState::Unset);
    }
}
