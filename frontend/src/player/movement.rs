use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;
use bevy_xpbd_3d::plugins::spatial_query::RayHits;

use crate::camera::third_person_camera_update;

use super::Player;

pub fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    mut player_physics: Query<(&mut LinearVelocity, &RayHits)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_transform: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mouse_motion: EventReader<MouseMotion>,
) {
    let (mut player_transform, mut player) = player_query.single_mut();
    if !player.movement_enabled {
        return;
    }

    let mut camera_transform = camera_transform.single_mut();
    let (mut player_vel, ray_hits) = match player_physics.get_single_mut() {
        Ok((vel, hits)) => (vel, hits),
        Err(_) => return,
    };

    player_linear_movement(
        time,
        &mut player_vel,
        &player,
        keys,
        &mut camera_transform,
        ray_hits,
    );

    let camera_y_rotation = player_rotation(
        &mut player_transform,
        mouse_motion,
        player.mouse_sensitivity,
    );

    third_person_camera_update(
        &(player_transform.translation + Vec3::new(0.0, 2.0, 0.0)),
        &player_transform.rotation,
        player.mouse_sensitivity,
        &mut camera_transform,
        &mut player.camera_data,
        camera_y_rotation,
    );
}

pub fn player_linear_movement(
    time: Res<Time>,
    player_vel: &mut LinearVelocity,
    player: &Player,
    keys: Res<ButtonInput<KeyCode>>,
    camera_transform: &mut Transform,
    ray_hits: &RayHits,
) {
    let on_ground = ray_hits.iter().any(|hit| hit.time_of_impact < 0.01);

    // Movement
    let direction_change = keys
        .get_pressed()
        .fold(Vec3::ZERO, |mut acc, key| {
            let planar_movement = match key {
                KeyCode::KeyW => camera_transform.forward().xz(),
                KeyCode::KeyA => camera_transform.left().xz(),
                KeyCode::KeyS => camera_transform.back().xz(),
                KeyCode::KeyD => camera_transform.right().xz(),
                _ => Vec2::ZERO,
            };

            acc.x += planar_movement.x;
            acc.z += planar_movement.y;

            acc
        })
        .normalize_or_zero();

    // Gravity and jumping
    if !on_ground {
        player_vel.0.y -= 9.8 * time.delta_seconds();
    } else if keys.just_pressed(KeyCode::Space) {
        player_vel.0.y = 4.9;
    } else {
        player_vel.0.y = 0.0;
    }

    let new_change = direction_change * time.delta_seconds() * player.speed;

    // How much of the new change to apply. This is smoothing.
    let percent_change = time.delta_seconds()
        * match on_ground {
            true => 10.0,
            false => 1.0,
        };

    #[rustfmt::skip]
    player_vel.0 = new_change   * percent_change
                 + player_vel.0 * (1.0 - percent_change);
}

pub fn player_rotation(
    player_transform: &mut Transform,
    mut mouse_motion: EventReader<MouseMotion>,
    sensitivity: f32,
) -> f32 {
    let change = -mouse_motion
        .read()
        .fold(Vec2::ZERO, |acc, motion| acc + motion.delta);

    let player_quat = Quat::from_rotation_y(change.x * sensitivity);
    player_transform.rotation *= player_quat;

    -change.y
}
