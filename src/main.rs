use bevy::prelude::*;
use bevy::window::close_on_esc;
use invaders::game::systems::*;
use invaders::game::ui::*;
use invaders::game::*;
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
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_systems(Startup, (spawn_camera, add_resources))
        .add_systems(Update, (make_visible, close_on_esc).chain())
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
