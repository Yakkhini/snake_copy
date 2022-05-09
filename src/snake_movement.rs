use crate::{snake_head, snake_segments, publiclogic, event_gameover};
use bevy::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum  Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub fn snake_movement(
    segments: ResMut<snake_segments::SnakeSegments>,
    mut heads: Query<(Entity, &snake_head::SnakeHead)>,
    mut positions: Query<&mut publiclogic::Position>,
    mut last_tail_position: ResMut<snake_segments::LastTailPosition>,
    mut game_over_writer: EventWriter<event_gameover::GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<publiclogic::Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {head_pos.x -= 1;}
            Direction::Right => {head_pos.x += 1;}
            Direction::Up => {head_pos.y += 1;}
            Direction::Down => {head_pos.y -= 1;}
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= publiclogic::ARENA_WIDTH
            || head_pos.y as u32 >= publiclogic::ARENA_HEIGHT {
            println!("Game Over by hitting.");
            game_over_writer.send(event_gameover::GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            println!("Game Over by self-eating.");
            game_over_writer.send(event_gameover::GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = snake_segments::LastTailPosition(Some(*segment_positions.last().unwrap()));
        
    };
}

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut snake_head::SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) | keyboard_input.pressed(KeyCode::A) {
            Direction::Left

        } else if keyboard_input.pressed(KeyCode::Right) | keyboard_input.pressed(KeyCode::D) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Up) | keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) | keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}