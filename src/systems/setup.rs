use crate::{
    components::player::{AnimationIndices, Player},
    resources::assets::GameAssets,
};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Player::default(),
        SpriteBundle {
            texture: game_assets.player_image.clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..Default::default()
        },
        AnimationIndices::default(),
        TextureAtlas::from(game_assets.player_atlas.clone_weak()),
    ));
}
