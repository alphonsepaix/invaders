pub mod systems;

use crate::game::{EntityDirection, GameState};
use crate::settings::{GREEN_ALIEN_VALUE, RED_ALIEN_VALUE, UFO_VALUE, YELLOW_ALIEN_VALUE};
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(Event)]
pub struct AlienHit {
    pub alien_type: Alien,
    pub id: Entity,
    pub position: Vec2,
}

#[derive(Component)]
pub struct XpTimer(pub Timer);

#[derive(Clone, Component)]
pub enum Alien {
    Yellow,
    Green,
    Red,
    Ufo,
}

impl Alien {
    pub fn color(&self) -> Color {
        match self {
            Alien::Yellow => Color::YELLOW,
            Alien::Green => Color::GREEN,
            Alien::Red => Color::RED,
            Alien::Ufo => Color::RED,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Alien::Yellow => YELLOW_ALIEN_VALUE,
            Alien::Green => GREEN_ALIEN_VALUE,
            Alien::Red => RED_ALIEN_VALUE,
            Alien::Ufo => UFO_VALUE,
        }
    }
}

#[derive(Component)]
pub struct Ufo(pub EntityDirection);

pub struct AliensPlugin;

impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AlienHit>()
            .add_systems(OnEnter(AppState::InGame), spawn_aliens)
            .add_systems(
                FixedUpdate,
                (move_aliens, alien_reach_floor)
                    .chain()
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                aliens_shoot
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                FixedUpdate,
                (spawn_ufo, move_ufo, handle_alien_hit)
                    .run_if(in_state(AppState::InGame))
                    .run_if(not(in_state(GameState::Pause))),
            );
    }
}
