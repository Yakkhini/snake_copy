use crate::publiclogic;
use bevy::prelude::*;

const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(
    pub Vec<Entity>
);

#[derive(Default)]
pub struct LastTailPosition(
    pub Option<publiclogic::Position>
);

pub fn spawn_segment(mut commands: Commands, position: publiclogic::Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle{
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(publiclogic::Size::square(0.65))
        .id()
}