pub mod menu;
pub mod panel;
pub mod pause;

use bevy::prelude::*;
pub use menu::MenuPlugin;
pub use panel::PanelPlugin;
pub use pause::PausePlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MenuPlugin)
            .add_plugins(PausePlugin)
            .add_plugins(PanelPlugin);
    }
}
