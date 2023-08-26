use bevy::{audio::CpalSample, prelude::*, transform::commands};

/*
A SnakePiece must have;
is_head: boolean
current_position: Vec(x, y)
next_position: getter for the next SnakePiece current_position
*/

/**
 * Sname component contains vector of tuples, representing segments at position x, y. The last segment is the head.
 * Tile map will be 17x17
 */

// define constants
static MAP_TILE_SIZE: f32 = 25.0;
static MAP_HEIGHT: f32 = MAP_TILE_SIZE * 17.;
static MAP_WIDTH: f32 = MAP_TILE_SIZE * 17.;
static MAP_CENTER_VALUE: f32 = MAP_HEIGHT / 2.;

#[derive(Component)]
struct Snake {
    segments: Vec<(u8, u8)>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
struct CurrentDirection(Direction);

#[derive(Component)]
struct GameOver(bool);

#[derive(Component)]
struct Map;

#[derive(Component, Debug)]
struct MapTile((i32, i32));

fn init_snake(mut commands: Commands) {
    commands.spawn(Snake {
        segments: vec![(7, 9), (8, 9), (9, 9), (10, 9)],
    });
    commands.spawn(CurrentDirection(Direction::Right));
    commands.spawn(GameOver(false));
}

fn init_map(mut commands: Commands) {
    // create map and map tiles
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

    let map_entity = commands.spawn(Map).id();
    let mut map_tile_entities: Vec<Entity> = vec![];
    // loop through tiles to create tile entities
    for tile in map_tiles.clone() {
        let tile_entity = commands.spawn(MapTile(tile)).id();
        map_tile_entities.push(tile_entity);
    }
    // add tile entity children to map entity
    for tile in map_tile_entities {
        commands.entity(map_entity).add_child(tile);
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn calculate_map_position(x: i32, y: i32, map_center_value: f32, map_tile_size: f32) -> Position {
    let x_pos = map_tile_size * x as f32 - map_center_value;
    let y_pos = map_tile_size * y as f32 - map_center_value;

    Position { x: x_pos, y: y_pos }
}

fn init_camera(mut commands: Commands) {
    // setup camera
    commands.spawn(Camera2dBundle::default());
}

fn init_map_renderer(map_tiles_query: Query<&MapTile>, mut commands: Commands) {
    let map_tiles = map_tiles_query.iter();

    // render tiles
    for tile in map_tiles {
        let position =
            calculate_map_position(tile.0 .0, tile.0 .1, MAP_CENTER_VALUE, MAP_TILE_SIZE);
        // calc positions
        let x_pos = position.x;
        let y_pos = position.y;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(MAP_TILE_SIZE, MAP_TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 1.)),
            ..default()
        });
    }
}

// used for quering snake segment sprites
#[derive(Component)]
struct SnakeSegmentSprite(u8, u8);

fn init_snake_renderer(snake_query: Query<&Snake>, mut commands: Commands) {
    let snake = snake_query.single();

    for segment in snake.segments.iter() {
        let position = calculate_map_position(
            segment.0 as i32,
            segment.1 as i32,
            MAP_CENTER_VALUE,
            MAP_TILE_SIZE,
        );
        // calc positions
        let x_pos = position.x;
        let y_pos = position.y;

        commands
            .spawn(SnakeSegmentSprite(segment.0, segment.1))
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 0.8),
                    custom_size: Some(Vec2::new(MAP_TILE_SIZE, MAP_TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 2.)),
                ..default()
            })
            .insert(position);
    }
}

#[derive(Resource)]
struct MovementTimer(Timer);

fn move_snake_segments(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut query: Query<&mut Snake>,
    direction_query: Query<&CurrentDirection>,
    mut game_over_query: Query<&GameOver>,
) {
    // if MovementTimer not finished return early and do nothing
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut snake = query.single_mut();
    let snake_segments = snake.segments.clone();
    let mut updated_segments = snake_segments.clone();
    let game_over = game_over_query.single_mut();

    // set position for last section (the head) based on direction
    let first_segment = snake_segments.last().unwrap();
    let current_direction = &direction_query.single().0;
    match current_direction {
        Direction::Up => updated_segments[snake_segments.len() - 1].1 = first_segment.1 + 1,
        Direction::Right => updated_segments[snake_segments.len() - 1].0 = first_segment.0 + 1,
        Direction::Down => updated_segments[snake_segments.len() - 1].1 = first_segment.1 - 1,
        Direction::Left => updated_segments[snake_segments.len() - 1].0 = first_segment.0 - 1,
    };
    // set position for all but last
    for index in 1..snake_segments.len() {
        // get current segment
        let current_segment = &snake_segments[index];
        // get next position
        let mut updated_segment = updated_segments[index - 1].clone();
        // set the current segment to the new position
        current_segment.clone_into(&mut updated_segment);
        //
        updated_segments[index - 1] = updated_segment;
    }

    snake.segments = updated_segments;
}

// for segments that have changed position, create new sprite
// fn update_snake_segment_rendered_position(
//     segment_query: Query<Entity, (&SnakeSegmentSprite, Changed<Position>)>,
//     commands: Commands,
// ) {
//     let segment_entities = segment_query.iter();

//     for segment_entity in segment_entities {

//         let current_position

//         let position = calculate_map_position(
//             segment_entity.0 as i32,
//             segment_entity.1 as i32,
//             MAP_CENTER_VALUE,
//             MAP_TILE_SIZE,
//         );
//         // calc positions
//         let x_pos = position.x;
//         let y_pos = position.y;

//         commands.entity(segment_entity).insert(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(0.8, 0.8, 0.8),
//                 custom_size: Some(Vec2::new(MAP_TILE_SIZE, MAP_TILE_SIZE)),
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new(x_pos, y_pos, 2.)),
//             ..default()
//         });
//     }
// }

fn detect_map_collision(mut query: Query<&mut Snake>, mut game_over_query: Query<&mut GameOver>) {
    let snake = query.single_mut();
    let snake_segments = snake.segments.clone();
    let head_segment = snake_segments.last().unwrap();
    let mut game_over = game_over_query.single_mut();

    // has hit a square in position 17 or 0 on the x and y axis
    let has_collided =
        head_segment.0 > 16 || head_segment.1 > 16 || head_segment.0 < 1 || head_segment.1 < 1;

    if has_collided {
        game_over.0 = true;
    }
}

fn detect_body_collision(mut query: Query<&mut Snake>, mut game_over_query: Query<&mut GameOver>) {
    let snake = query.single_mut();
    let snake_segments = snake.segments.clone();
    let mut snake_segments_clone = snake.segments.clone();
    let head_segment = snake_segments.last().unwrap();
    let mut game_over = game_over_query.single_mut();

    // remove head segment from segments clone for search
    snake_segments_clone.pop();
    // has hit a square in position 17 or 0 on the x and y axis
    let has_collided = snake_segments_clone
        .iter()
        .position(|&segment| &segment == head_segment)
        .is_some();

    if has_collided {
        game_over.0 = true;
    }
}

fn change_direction(mut query: Query<&mut CurrentDirection>, keys: Res<Input<KeyCode>>) {
    let mut current_direction = query.single_mut();
    if keys.just_pressed(KeyCode::W) {
        current_direction.0 = Direction::Up;
    }
    if keys.just_pressed(KeyCode::D) {
        current_direction.0 = Direction::Right;
    }
    if keys.just_pressed(KeyCode::S) {
        current_direction.0 = Direction::Down;
    }
    if keys.just_pressed(KeyCode::A) {
        current_direction.0 = Direction::Left;
    }
}

fn main() {
    App::new()
        .insert_resource(MovementTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_snake, init_map, init_camera))
        // renderers querying entites must run in a seperate stage as commands always run after stage is finished (Startup in this instance)
        .add_systems(PostStartup, (init_map_renderer, init_snake_renderer))
        .add_systems(Update, (detect_map_collision, detect_body_collision))
        .add_systems(Update, change_direction)
        .add_systems(Update, move_snake_segments)
        .run();
}
