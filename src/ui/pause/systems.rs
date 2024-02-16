use crate::game::resources::AlienTimer;
use crate::game::GameState;
use crate::settings::{SCOREBOARD_FONT_SIZE, TEXT_COLOR};
use bevy::app::AppExit;
use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::prelude::*;
use bevy::prelude::{
    default, AlignItems, Commands, EventWriter, JustifyContent, KeyCode, NextState, NodeBundle,
    Res, ResMut, State, Style, TextBundle, TextSection, TextStyle, Val,
};

#[derive(Component)]
pub struct OnPauseScreen;

pub fn pause_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    "Pause".to_uppercase(),
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                        font: asset_server.load("fonts/font.ttf"),
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                }),
            ]));
        });
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
