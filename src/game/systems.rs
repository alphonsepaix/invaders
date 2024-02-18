use crate::game::{GameOver, GameState, OnGameScreen};
use crate::resources::*;
use crate::settings::*;
use crate::*;
use bevy::core::FrameCount;
use bevy::prelude::*;

pub fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == WINDOW_VISIBLE_DELAY {
        window.single_mut().visible = true;
    }
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
                translation: Vec3::new(window_width / 2.0, FLOOR_HEIGHT, 1.0),
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
    mut already_played: ResMut<AlreadyPlayed>,
) {
    if game_over_event_reader.read().next().is_some() {
        next_game_state.set(GameState::Transition);
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
