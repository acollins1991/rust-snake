use bevy::{prelude::*, utils::tracing::Instrument};

/*
A SnakePiece must have;
is_head: boolean
current_position: Vec(x, y)
next_position: getter for the next SnakePiece current_position
*/

/**
 * Sname component contains vector of tuples, representing segments at position x, y. The last segment is the head.
 */

#[derive(Component)]
struct Snake(Vec<(u8, u8)>);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
struct CurrentDirection(Direction);

fn init_snake(mut commands: Commands) {
    commands.spawn(Snake(vec![(7, 9), (8, 9), (9, 9), (10, 9)]));
    commands.spawn(CurrentDirection(Direction::Right));
}

fn move_snake_segments(mut query: Query<&mut Snake>, direction_query: Query<&CurrentDirection>) {
    let mut snake = query.single_mut();
    let snake_segments = snake.0.clone();
    let mut updated_segments = snake_segments.clone();

    for segment in snake.0.clone() {
        println!("{:?}", segment);
    }

    // set position for last section (the head) based on direction
    let first_segment = snake_segments.last().unwrap();
    let current_direction = &direction_query.single().0;
    match current_direction {
        Direction::Up => updated_segments[snake_segments.len() - 1].1 = first_segment.1 + 1,
        Direction::Right => updated_segments[snake_segments.len() - 1].0 = first_segment.0 + 1,
        Direction::Down => updated_segments[snake_segments.len() - 1].1 = first_segment.1 - 1,
        Direction::Left => updated_segments[snake_segments.len() - 1].0 = first_segment.0 - 1,
    };

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

    snake.0 = updated_segments;

    for segment in snake.0.clone() {
        println!("{:?}", segment);
    }
}

fn change_direction(mut query: Query<&mut CurrentDirection>, keys: Res<Input<KeyCode>>) {
    let mut current_direction = query.single_mut();
    if keys.just_pressed(KeyCode::W) {
        current_direction.0 = Direction::Up;
        println!("{:?}", current_direction.0);
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
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_snake))
        .add_systems(Update, change_direction)
        .add_systems(Update, (move_snake_segments,))
        .run();
}
