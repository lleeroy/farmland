use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;

use crate::{
    components::player::{AnimationIndices, Player},
    resources::assets::GameAssets,
};

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map.ldtk"),
        ..Default::default()
    });
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
