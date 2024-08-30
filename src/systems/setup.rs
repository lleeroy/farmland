use crate::resources::assets::GameAssets;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_player(mut commands: Commands, game_assets: Res<GameAssets>) {}
