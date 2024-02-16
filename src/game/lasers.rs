pub mod systems;

use crate::game::GameState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct LaserExplosion(pub Entity);

pub struct LasersPlugin;

impl Plugin for LasersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserExplosion>().add_systems(
            FixedUpdate,
            (
                move_lasers,
                despawn_lasers,
                handle_laser_explosion,
                check_for_collisions,
                update_xp_texts,
            )
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::Running)),
        );
    }
}
