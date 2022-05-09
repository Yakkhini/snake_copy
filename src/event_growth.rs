use bevy::prelude::*;

use crate::{publiclogic, snake_head, food, snake_segments};

pub struct GrowthEvent;

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &publiclogic::Position), With<food::Food>>,
    head_positions: Query<&publiclogic::Position, With<snake_head::SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if head_pos == food_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<snake_segments::LastTailPosition>,
    mut segments: ResMut<snake_segments::SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(snake_segments::spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}