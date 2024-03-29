use crate::game::aliens::{Alien, Ufo};
use crate::game::lasers::Laser;
use crate::game::OnGameScreen;
use crate::resources::{LivesRemaining, PlayerScore};
use crate::settings::{SCOREBOARD_FONT_SIZE, TEXT_COLOR};
use crate::ui::panel::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub fn spawn_scoreboard(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_text(
        commands,
        JustifyContent::Start,
        "Score=",
        UiPlayerScore,
        asset_server,
    );
}

pub fn spawn_remaining_lives(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_text(
        commands,
        JustifyContent::End,
        "Lives=",
        UiLivesRemaining,
        asset_server,
    );
}

pub fn spawn_remaining_aliens(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_text(
        commands,
        JustifyContent::Center,
        "Aliens=",
        UiAliensRemaining,
        asset_server,
    );
}

fn spawn_text(
    mut commands: Commands,
    justify_content: JustifyContent,
    text: impl ToString,
    component: impl Component,
    asset_server: Res<AssetServer>,
) {
    let style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content,
        align_items: AlignItems::End,
        padding: UiRect::all(Val::Px(5.0)),
        ..default()
    };
    let font = asset_server.load("fonts/font.ttf");
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
                                    font: font.clone(),
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: SCOREBOARD_FONT_SIZE,
                                color: TEXT_COLOR,
                                font,
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

pub fn update_remaining_aliens(
    aliens_query: Query<&Alien, (Without<Ufo>, Without<Laser>)>,
    mut query: Query<&mut Text, With<UiAliensRemaining>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = aliens_query.iter().count().to_string();
    }
}
