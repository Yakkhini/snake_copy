use crate::{snake_segments, publiclogic, snake_movement};
use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);


#[derive(Component)]
pub struct SnakeHead {
    pub direction: snake_movement::Direction,
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<snake_segments::SnakeSegments>) {
    *segments = snake_segments::SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: snake_movement::Direction::Up,
            })
            .insert(snake_segments::SnakeSegment)
            .insert(publiclogic::Position {x: 3, y: 3})
            .insert(publiclogic::Size::square(0.8))
            .id(),
            snake_segments::spawn_segment(commands, publiclogic::Position { x: 3, y: 2 }),
    ]);
}