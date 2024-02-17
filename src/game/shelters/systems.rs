use crate::game::lasers::{Laser, LaserExplosion};
use crate::game::shelters::{Shelter, ShelterArmorText};
use crate::game::OnGameScreen;
use crate::settings::{
    FLOOR_HEIGHT, INITIAL_ARMOR_VALUE, LASER_SIZE, NUM_SHELTERS, PLAYER_SIZE, SHELTER_SCALE_FACTOR,
    SHELTER_SIZE,
};
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Color, Commands, Entity, EventWriter, PositionType, Query, Res, SpriteBundle, Style,
    Text, TextAlignment, TextBundle, TextStyle, Transform, Val, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn spawn_shelters(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let sprite = asset_server.load("sprites/shelter.png");
    let font = asset_server.load("fonts/font.ttf");

    let shelter_size = get_shelter_size();
    let space_between_shelters =
        (window.width() - NUM_SHELTERS as f32 * shelter_size.x) / (NUM_SHELTERS + 1) as f32;
    let height_below_shelter = 2.0 * FLOOR_HEIGHT + PLAYER_SIZE.y;
    let mut translation = Vec3::new(
        space_between_shelters + shelter_size.x / 2.0,
        height_below_shelter,
        0.0,
    );

    for _ in 0..NUM_SHELTERS {
        let shelter_id = commands
            .spawn((
                SpriteBundle {
                    texture: sprite.clone(),
                    transform: Transform {
                        translation,
                        scale: Vec3::new(SHELTER_SCALE_FACTOR, SHELTER_SCALE_FACTOR, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Shelter {
                    armor: INITIAL_ARMOR_VALUE,
                },
                OnGameScreen,
            ))
            .id();

        commands.spawn((
            TextBundle::from_section(
                INITIAL_ARMOR_VALUE.to_string(),
                TextStyle {
                    color: Color::WHITE,
                    font: font.clone(),
                    font_size: 20.0,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(translation.x - 15.0),
                bottom: Val::Px(translation.y - 50.0),
                ..default()
            }),
            OnGameScreen,
            ShelterArmorText(shelter_id),
        ));
        translation.x += space_between_shelters + shelter_size.x;
    }
}

pub fn shelter_hit(
    mut commands: Commands,
    mut laser_explosion_event_writer: EventWriter<LaserExplosion>,
    mut shelters_query: Query<(Entity, &Transform, &mut Shelter)>,
    mut armor_texts_query: Query<(Entity, &mut Text, &mut ShelterArmorText)>,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
) {
    for (laser_entity, laser_transform) in lasers_query.iter() {
        for (shelter_entity, shelter_transform, mut shelter) in shelters_query.iter_mut() {
            if shelter_transform
                .translation
                .distance(laser_transform.translation)
                <= get_shelter_size().x / 2.0 + LASER_SIZE.x / 2.0
            {
                laser_explosion_event_writer.send(LaserExplosion(laser_entity));
                shelter.armor = shelter.armor.saturating_sub(5);

                // Retrieve the armor text corresponding to this shelter.
                let (text_entity, mut text, _) = armor_texts_query
                    .iter_mut()
                    .find(|(_, _, t)| t.0 == shelter_entity)
                    .unwrap();
                text.sections[0].value = shelter.armor.to_string();

                if shelter.armor == 0 {
                    commands.entity(shelter_entity).despawn();
                    commands.entity(text_entity).despawn();
                }
            }
        }
    }
}

pub fn get_shelter_size() -> Vec2 {
    SHELTER_SIZE * SHELTER_SCALE_FACTOR
}
