pub mod menu;
pub mod pause;

pub use menu::MenuPlugin;
pub use pause::*;

use crate::game::components::OnGameScreen;
use crate::game::resources::{LivesRemaining, PlayerScore};
use crate::settings::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Pause,
    InGame,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    Transition,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TransitionState {
    #[default]
    Unset,
    PlayerKilled,
    AliensKilled,
    GameOver,
}

pub fn despawn_screen<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn return_to_menu(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}

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

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
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
                                "Score = ".to_uppercase(),
                                TextStyle {
                                    font_size: SCOREBOARD_FONT_SIZE,
                                    color: TEXT_COLOR,
                                    font: asset_server.load("fonts/font.ttf"),
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: SCOREBOARD_FONT_SIZE,
                                color: TEXT_COLOR,
                                font: asset_server.load("fonts/font.ttf"),
                            }),
                        ]),
                        UiPlayerScore,
                    ));
                });
        });
}

pub fn spawn_remaining_lives(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
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
                                "Lives = ".to_uppercase(),
                                TextStyle {
                                    font_size: SCOREBOARD_FONT_SIZE,
                                    color: TEXT_COLOR,
                                    font: asset_server.load("fonts/font.ttf"),
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: SCOREBOARD_FONT_SIZE,
                                color: TEXT_COLOR,
                                font: asset_server.load("fonts/font.ttf"),
                            }),
                        ]),
                        UiLivesRemaining,
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
