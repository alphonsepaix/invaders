use crate::game::lasers::{Laser, LaserExplosion};
use crate::game::shelters::{Shelter, ShelterArmorText};
use crate::game::OnGameScreen;
use crate::settings::*;
use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_shelters(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let sprite = asset_server.load("sprites/shelter.png");
    let font = asset_server.load("fonts/font.ttf");

    let space_between_shelters =
        (window.width() - NUM_SHELTERS as f32 * SHELTER_SIZE.x) / (NUM_SHELTERS + 1) as f32;
    let height_below_shelter = 2.0 * FLOOR_HEIGHT + PLAYER_SIZE.y;
    let mut translation = Vec3::new(
        space_between_shelters + SHELTER_SIZE.x / 2.0,
        height_below_shelter,
        1.0,
    );

    let text_style = TextStyle {
        color: Color::WHITE,
        font: font.clone(),
        font_size: 20.0,
    };

    for _ in 0..NUM_SHELTERS {
        commands
            .spawn((
                SpriteBundle {
                    texture: sprite.clone(),
                    transform: Transform {
                        translation,
                        ..default()
                    },
                    ..default()
                },
                Shelter {
                    armor: INITIAL_ARMOR_VALUE,
                },
                OnGameScreen,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            INITIAL_ARMOR_VALUE.to_string(),
                            text_style.clone(),
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(Vec3::new(0.0, -40.0, 0.0)),
                        ..default()
                    },
                    ShelterArmorText(parent.parent_entity()),
                ));
            });

        translation.x += space_between_shelters + SHELTER_SIZE.x;
    }
}

pub fn shelter_hit(
    mut commands: Commands,
    mut laser_explosion_event_writer: EventWriter<LaserExplosion>,
    mut shelters_query: Query<(Entity, &Transform, &mut Shelter)>,
    mut armor_texts_query: Query<(&mut Text, &mut ShelterArmorText)>,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
) {
    for (laser_entity, laser_transform) in lasers_query.iter() {
        for (shelter_entity, shelter_transform, mut shelter) in shelters_query.iter_mut() {
            if shelter_transform
                .translation
                .distance(laser_transform.translation)
                <= SHELTER_SIZE.x / 2.0 + LASER_SIZE.x / 2.0
            {
                laser_explosion_event_writer.send(LaserExplosion(laser_entity));
                shelter.armor = shelter.armor.saturating_sub(5);

                // Retrieve the armor text corresponding to this shelter.
                let (mut text, _) = armor_texts_query
                    .iter_mut()
                    .find(|(_, t)| t.0 == shelter_entity)
                    .unwrap();
                text.sections[0].value = shelter.armor.to_string();

                if shelter.armor == 0 {
                    commands.entity(shelter_entity).despawn_recursive();
                }
            }
        }
    }
}
