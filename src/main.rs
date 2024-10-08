use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use components::player::PlayerBundle;
use resources::assets::GameAssets;
use resources::state::GameState;

use systems::setup::*;

mod components;
mod resources;
mod systems;

const WW: f32 = 1200.0;
const WH: f32 = 900.0;
const BG_COLOR: (u8, u8, u8) = (125, 120, 143);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .insert_resource(LevelSelection::index(0))
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Initializing)
                .load_collection::<GameAssets>(),
        )
        .add_systems(
            OnEnter(GameState::Initializing),
            (setup_camera, setup_player),
        )
        .add_loading_state(
            LoadingState::new(GameState::Initializing).continue_to_state(GameState::Playing),
        )
        .add_systems(
            Update,
            (
                systems::movement::player_movement,
                systems::movement::animate_movement,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
