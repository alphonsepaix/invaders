pub mod menu;
pub mod panel;
pub mod pause;

pub use menu::MenuPlugin;
pub use panel::PanelPlugin;
pub use pause::PausePlugin;

use bevy::prelude::*;

#[derive(Component)]
pub struct OnMenuScreen;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin)
            .add_plugins(PausePlugin)
            .add_plugins(PanelPlugin);
    }
}
