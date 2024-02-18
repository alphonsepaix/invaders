use crate::game::aliens::{Alien, AlienHit, XpTimer};
use crate::game::lasers::{ExplosionTimer, Laser, LaserExplosion};
use crate::game::player::{Player, PlayerHit};
use crate::game::{EntityDirection, OnGameScreen};
use crate::settings::{
    ALIEN_SIZE, EXPLOSION_DURATION, EXPLOSION_MAX_RADIUS, EXPLOSION_MIN_RADIUS, FLOOR_HEIGHT,
    FLOOR_THICKNESS, LASER_SIZE, PLAYER_SIZE, XP_GAIN_DURATION,
};
use bevy::asset::{Assets, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::shape::Circle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

pub fn move_lasers(mut lasers_query: Query<(&mut Transform, &Laser)>, time: Res<Time>) {
    for (mut transform, Laser { direction, speed }) in lasers_query.iter_mut() {
        let movement = match direction {
            EntityDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            EntityDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            _ => panic!("Laser is going the wrong way!"),
        };
        transform.translation += movement * *speed * time.delta_seconds();
    }
}

pub fn despawn_lasers(
    // mut commands: Commands,
    mut laser_explosion_event_writer: EventWriter<LaserExplosion>,
    lasers_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    lasers_query.iter().for_each(|(entity, transform)| {
        let y_bottom = transform.translation.y - LASER_SIZE.y / 2.0;

        if y_bottom > window.height() - LASER_SIZE.y
            || y_bottom < FLOOR_HEIGHT + FLOOR_THICKNESS / 2.0
        {
            laser_explosion_event_writer.send(LaserExplosion(entity));
        }
    })
}

pub fn handle_laser_explosion(
    mut commands: Commands,
    mut laser_explosion_event_reader: EventReader<LaserExplosion>,
    lasers_query: Query<(&Transform, &Laser)>,
    mut explosions_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Handle<ColorMaterial>,
            &mut ExplosionTimer,
        ),
        Without<Laser>,
    >,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for laser in laser_explosion_event_reader.read() {
        let laser_entity = laser.0;
        if let Ok((transform, Laser { direction, .. })) = lasers_query.get(laser_entity) {
            // Show an explosion.
            let mut translation = transform.translation;
            let half_laser_height = LASER_SIZE.y / 2.0;
            translation.y += match direction {
                EntityDirection::Up => half_laser_height,
                EntityDirection::Down => -half_laser_height,
                other => panic!("Laser should only go up and down, got {:?}", other),
            };
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default().into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(translation)
                        .with_scale(Vec2::splat(LASER_SIZE.y).extend(0.0)),
                    ..default()
                },
                ExplosionTimer(Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once)),
                OnGameScreen,
            ));
            commands.entity(laser_entity).despawn();
        }
    }

    for (entity, mut transform, color, mut explosion_timer) in explosions_query.iter_mut() {
        explosion_timer.0.tick(time.delta());
        let elapsed = explosion_timer.0.elapsed_secs();
        let ratio = elapsed / EXPLOSION_DURATION;
        let radius = EXPLOSION_MIN_RADIUS + (EXPLOSION_MAX_RADIUS - EXPLOSION_MIN_RADIUS) * ratio;
        transform.scale = Vec2::splat(radius).extend(0.0);
        let alpha = 1.0 - ratio;
        let color_mat = materials.get_mut(&*color).unwrap();
        color_mat.color.set_a(alpha);
        if explosion_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_for_collisions(
    // mut commands: Commands,
    mut alien_hit_event_writer: EventWriter<AlienHit>,
    mut player_hit_event_writer: EventWriter<PlayerHit>,
    mut laser_explosion_event_writer: EventWriter<LaserExplosion>,
    player_query: Query<&Transform, (With<Player>, Without<Laser>)>,
    aliens_query: Query<(Entity, &Transform, &Alien), Without<Laser>>,
    player_laser_query: Query<(Entity, &Transform), (With<Laser>, With<Player>)>,
    alien_lasers_query: Query<(Entity, &Transform), (With<Laser>, With<Alien>)>,
) {
    let half_player_height = PLAYER_SIZE.y / 2.0;
    let half_alien_height = ALIEN_SIZE.y / 2.0;
    let half_laser_height = LASER_SIZE.y / 2.0;

    // Check if an alien hit the player.
    if let Ok(player_transform) = player_query.get_single() {
        for (laser_entity, laser_transform) in alien_lasers_query.iter() {
            if player_transform
                .translation
                .distance(laser_transform.translation)
                < half_player_height + half_laser_height
            {
                laser_explosion_event_writer.send(LaserExplosion(laser_entity));
                player_hit_event_writer.send(PlayerHit);
            }
        }
    }

    // Check if player hit an alien.
    for (alien_entity, alien_transform, alien_type) in aliens_query.iter() {
        if let Ok((laser_entity, laser_transform)) = player_laser_query.get_single() {
            if alien_transform
                .translation
                .distance(laser_transform.translation)
                < half_alien_height + half_laser_height
            {
                laser_explosion_event_writer.send(LaserExplosion(laser_entity));
                alien_hit_event_writer.send(AlienHit {
                    alien_type: alien_type.clone(),
                    id: alien_entity,
                    position: alien_transform.translation.truncate(),
                });
            }
        }
    }
}

pub fn update_xp_texts(
    mut commands: Commands,
    mut texts_query: Query<(Entity, &mut Text, &mut XpTimer)>,
    time: Res<Time>,
) {
    for (entity, mut text, mut xp_timer) in texts_query.iter_mut() {
        xp_timer.0.tick(time.delta());
        if xp_timer.0.finished() {
            commands.entity(entity).despawn();
        }
        let alpha = (XP_GAIN_DURATION - xp_timer.0.elapsed_secs()) / XP_GAIN_DURATION;
        text.sections[0].style.color.set_a(alpha);
    }
}
