#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::close_on_esc;
use invaders::game::systems::*;
use invaders::game::*;
use invaders::ui::*;
use invaders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Invaders".into(),
                resolution: get_window_resolution().into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..default()
                },
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UiPlugin)
        .add_plugins(GamePlugin)
        .add_state::<AppState>()
        .add_systems(Startup, (set_window_icon, spawn_camera, add_resources))
        .add_systems(
            Update,
            (make_visible, play_main_music, handle_input, close_on_esc),
        )
        .run();
}
