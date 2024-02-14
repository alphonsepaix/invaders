use crate::game::components::OnGameScreen;
use crate::game::resources::{LivesRemaining, PlayerScore};
use crate::settings::{SCOREBOARD_FONT_SIZE, TEXT_COLOR};
use crate::ui::{AppState, GameState};
use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_scoreboard, spawn_remaining_lives),
        )
        .add_systems(
            Update,
            (update_scoreboard, update_remaining_lives).run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Component)]
pub struct UiPlayerScore;

#[derive(Component)]
pub struct UiLivesRemaining;

pub fn spawn_scoreboard(commands: Commands, asset_server: Res<AssetServer>) {
    let style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Start,
        align_items: AlignItems::End,
        ..default()
    };
    let font = asset_server.load("fonts/font.ttf");
    spawn_text(commands, style, "Score = ", UiPlayerScore, font);
}

pub fn spawn_remaining_lives(commands: Commands, asset_server: Res<AssetServer>) {
    let style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::End,
        align_items: AlignItems::End,
        ..default()
    };
    let font = asset_server.load("fonts/font.ttf");
    spawn_text(
        commands,
        style,
        "Lives remaining = ",
        UiLivesRemaining,
        font,
    );
}

fn spawn_text(
    mut commands: Commands,
    style: Style,
    text: impl ToString,
    component: impl Component,
    font: Handle<Font>,
) {
    commands
        .spawn((NodeBundle { style, ..default() }, OnGameScreen))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                text.to_string().to_uppercase(),
                                TextStyle {
                                    font_size: SCOREBOARD_FONT_SIZE,
                                    color: TEXT_COLOR,
                                    font,
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: SCOREBOARD_FONT_SIZE,
                                color: TEXT_COLOR,
                                ..default()
                            }),
                        ]),
                        component,
                    ));
                });
        });
}

pub fn update_scoreboard(
    player_score: Res<PlayerScore>,
    mut query: Query<&mut Text, With<UiPlayerScore>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = player_score.0.to_string();
    }
}

pub fn update_remaining_lives(
    remaining_lives: Res<LivesRemaining>,
    mut query: Query<&mut Text, With<UiLivesRemaining>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = remaining_lives.0.to_string();
    }
}
