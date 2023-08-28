use bevy::prelude::*;

use crate::utils::*;

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
struct SnakeSegment {
    x: i32,
    y: i32,
}

pub fn init_snake(mut commands: Commands) {
    let snake_entity = commands.spawn(Snake).id();

    for segement in vec![(7, 9), (8, 9), (9, 9), (10, 9)] {
        let segment_entity = commands
            .spawn(SnakeSegment {
                x: segement.0,
                y: segement.1,
            })
            .id();

        commands.entity(snake_entity).add_child(segment_entity);
    }
}

pub fn init_snake_renderer(snake_query: Query<&Snake>, mut commands: Commands) {
    let snake = snake_query.single();

    for segment in snake.segments.iter() {
        let position = calculate_map_position(segment.0 as i32, segment.1 as i32);
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

#[test]
fn init_snake_registers_one_snake() {
    let mut app = App::new();

    //
    app.add_systems(Startup, init_snake);

    // run systems
    app.update();

    // assert creates 1 Snake entity
    let mut snake_query_binding = app.world.query::<&Snake>();
    let snake_query_iter = snake_query_binding.iter(&app.world);
    assert_eq!(snake_query_iter.count(), 1);
}

#[test]
fn init_snake_registers_four_snake_segments() {
    let mut app = App::new();

    //
    app.add_systems(Startup, init_snake);

    // run systems
    app.update();

    // asset creates four SnakeSegment entities
    let mut query = app.world.query::<(&Snake, &Children)>();
    let mut segments_vector = vec![];

    for (_, children) in query.iter(&app.world) {
        for child in children.iter() {
            segments_vector.push(child);
        }
    }

    assert_eq!(segments_vector.len(), 4);
}
