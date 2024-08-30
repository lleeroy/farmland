use crate::components::player::{AnimationIndices, Player, PlayerDirection};
use bevy::prelude::*;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut player_query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            player.current_direction = PlayerDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            player.current_direction = PlayerDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            player.current_direction = PlayerDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
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
            let animation_len = match player.current_direction {
                PlayerDirection::Down => &indices.down.len(),
                PlayerDirection::Up => &indices.up.len(),
                PlayerDirection::Left => &indices.left.len(),
                PlayerDirection::Right => &indices.right.len(),
                PlayerDirection::Idle => &indices.idle.len(),
            };

            indices.current_index = (indices.current_index + 1) % animation_len;

            atlas.index = match player.current_direction {
                PlayerDirection::Down => indices.down[indices.current_index],
                PlayerDirection::Up => indices.up[indices.current_index],
                PlayerDirection::Left => indices.left[indices.current_index],
                PlayerDirection::Right => indices.right[indices.current_index],
                PlayerDirection::Idle => indices.idle[indices.current_index],
            };
        }
    }
}
