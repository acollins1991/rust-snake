pub static MAP_TILE_SIZE: f32 = 25.0;
pub static MAP_HEIGHT: f32 = MAP_TILE_SIZE * 17.;
pub static MAP_WIDTH: f32 = MAP_TILE_SIZE * 17.;
pub static MAP_CENTER_VALUE: f32 = MAP_HEIGHT / 2.;

fn create_map_grid() -> Vec<(i32, i32)> {
    let mut map_tiles: Vec<(i32, i32)> = vec![];
    for (index, tile) in (1..=17).map(|x| x).enumerate() {
        let this_position = index as i32 + 1;
        // create base tuple: (1,1), (2,1) etc
        let tuple = (this_position, 1);
        map_tiles.push(tuple);
        // internal loop to create additional tuples: (1,2), (1,3) etc.
        // do not include the first as already created above
        for (internal_index, tile) in (1..=17).map(|x| x).enumerate() {
            if internal_index != 0 {
                let new_tuple = (this_position, (internal_index as i32 + 1));
                map_tiles.push(new_tuple);
            }
        }
    }

    return map_tiles;
}

pub const MAP_GRID: Vec<(i32, i32)> = create_map_grid();
