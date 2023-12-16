use crate::{
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    SCREEN_TILE_SIZE_Y,
};

pub const STATUS_SIZE_X: usize = GRID_SIZE_X + 1;
pub const STATUS_SIZE_Y: usize = SCREEN_TILE_SIZE_Y - GRID_SIZE_Y;
