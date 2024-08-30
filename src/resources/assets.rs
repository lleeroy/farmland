use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 7))]
    pub player_atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "player/player.png")]
    pub player_image: Handle<Image>,
}
