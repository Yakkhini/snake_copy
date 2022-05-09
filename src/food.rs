use bevy::prelude::*;
use rand::random;

use crate::publiclogic;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(publiclogic::Position {
            x: (random::<f32>() * publiclogic::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * publiclogic::ARENA_HEIGHT as f32) as i32,
        })
        .insert(publiclogic::Size::square(0.8));
}