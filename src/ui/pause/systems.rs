use crate::settings::{SCOREBOARD_FONT_SIZE, TEXT_COLOR};
use crate::ui::pause::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{
    default, AlignItems, Commands, JustifyContent, NodeBundle, Res, Style, TextBundle, TextSection,
    TextStyle, Val,
};

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
