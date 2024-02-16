use crate::game::resources::{AlreadyPlayed, ButtonHoveredSound, ButtonPressedSound};
use crate::settings::{
    BUTTON_HEIGHT, BUTTON_MARGIN, BUTTON_WIDTH, HOVERED_BUTTON, HOVERED_PRESSED_BUTTON,
    MENU_TEXT_COLOR, MENU_TITLE_SIZE, NORMAL_BUTTON, PRESSED_BUTTON, TEXT_BUTTON_SIZE,
};
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct OnMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

pub fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    already_played: Res<AlreadyPlayed>,
) {
    let button_style = Style {
        width: Val::Px(BUTTON_WIDTH),
        height: Val::Px(BUTTON_HEIGHT),
        margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: TEXT_BUTTON_SIZE,
        color: Color::WHITE,
        font: asset_server.load("fonts/font.ttf"),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Invaders".to_uppercase(),
                            TextStyle {
                                font_size: MENU_TITLE_SIZE,
                                color: MENU_TEXT_COLOR,
                                font: asset_server.load("fonts/font.ttf"),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    let play_button_text = if already_played.0 { "Replay" } else { "Play" };
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                play_button_text.to_uppercase(),
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit".to_uppercase(),
                                button_text_style,
                            ));
                        });
                });
        });
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    hovered_sound: Res<ButtonHoveredSound>,
    pressed_sound: Res<ButtonPressedSound>,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        if *interaction == Interaction::Hovered {
            commands.spawn(AudioBundle {
                source: hovered_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
        if *interaction == Interaction::Pressed {
            commands.spawn(AudioBundle {
                source: pressed_sound.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => game_state.set(AppState::InGame),
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
