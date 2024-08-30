use crate::components::player::{AnimationIndices, Player, PlayerDirection};
use bevy::prelude::*;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut player_query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            player.current_direction = PlayerDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            player.current_direction = PlayerDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            player.current_direction = PlayerDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            player.current_direction = PlayerDirection::Down;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_seconds();
        } else {
            player.current_direction = PlayerDirection::Idle;
        }
    }
}

pub fn animate_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut TextureAtlas, &mut AnimationIndices)>,
) {
    for (mut player, mut atlas, mut indices) in &mut player_query {
        player.timer.tick(time.delta());

        if player.timer.just_finished() {
            let direction_indices = match player.current_direction {
                PlayerDirection::Down => indices.down.clone(),
                PlayerDirection::Up => indices.up.clone(),
                PlayerDirection::Left => indices.left.clone(),
                PlayerDirection::Right => indices.right.clone(),
                PlayerDirection::Idle => indices.idle.clone(),
            };

            indices.current_index = (indices.current_index + 1) % direction_indices.len();
            atlas.index = direction_indices[indices.current_index];
        }
    }
}
