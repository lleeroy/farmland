use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
const PLAYER_DEFAULT_SPEED: f32 = 100.0;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub timer: Timer,
    pub current_direction: PlayerDirection,
}

#[derive(Default, Clone, Copy)]
pub enum PlayerDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
    Idle,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub idle: Vec<usize>,
    pub down: Vec<usize>,
    pub up: Vec<usize>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub current_index: usize,
}

impl Default for AnimationIndices {
    fn default() -> Self {
        Self {
            idle: vec![0, 1, 2, 3, 4, 5],
            down: vec![18, 19, 20, 21, 22, 23],
            up: vec![30, 31, 32, 33, 34, 35],
            left: vec![36, 37, 38, 39, 40, 41],
            right: vec![24, 25, 26, 27, 28, 29],
            current_index: 0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: PLAYER_DEFAULT_SPEED,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            current_direction: PlayerDirection::Idle,
        }
    }
}
