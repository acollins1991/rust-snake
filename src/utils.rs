use crate::constants::*;
use crate::map::Position;

pub fn calculate_map_position(x: i32, y: i32) -> Position {
    let x_pos = MAP_TILE_SIZE * x as f32 - MAP_CENTER_VALUE;
    let y_pos = MAP_TILE_SIZE * y as f32 - MAP_CENTER_VALUE;

    return Position { x: x_pos, y: y_pos };
}
