#![allow(unused)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use resources::assets::GameAssets;
use resources::state::GameState;
use systems::setup::{setup_camera, setup_player};

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
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
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
