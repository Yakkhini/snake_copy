use bevy::prelude::*;

use crate::{snake_segments, food, snake_head};

pub struct GameOverEvent;

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<snake_segments::SnakeSegments>,
    food: Query<Entity, With<food::Food>>,
    segments: Query<Entity, With<snake_segments::SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        //let mut i = 1;
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
            //println!("The {} time tring despawn {:?}...", i, ent);
            //i += 1;
        }
        snake_head::spawn_snake(commands, segments_res);
    }
}