use bevy::prelude::*;
use bevy::window::close_on_esc;
use invaders::game::systems::*;
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
        .add_systems(Update, (make_visible, close_on_esc).chain())
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AliensPlugin)
        .add_plugins(LasersPlugin)
        .add_plugins(GamePlugin)
        .run();
}
