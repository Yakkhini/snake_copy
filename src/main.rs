use bevy::{prelude::*, core::FixedTimestep};

mod event_gameover;
mod event_growth;
mod publiclogic;
mod food;
mod snake_head;
mod snake_segments;
mod snake_movement;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(snake_segments::SnakeSegments::default())
        .insert_resource(snake_segments::LastTailPosition::default())
        .add_event::<event_growth::GrowthEvent>()
        .add_system(snake_movement::snake_movement_input.before(snake_movement::snake_movement))
        .add_startup_system(publiclogic::setup_camera)
        .add_startup_system(snake_head::spawn_snake)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food::food_spawner),
        )
        .add_event::<event_gameover::GameOverEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.3))
                .with_system(snake_movement::snake_movement)
                .with_system(event_growth::snake_eating.after(snake_movement::snake_movement))
                .with_system(event_growth::snake_growth.after(event_growth::snake_eating))
        )
        .add_system(event_gameover::game_over.after(snake_movement::snake_movement))
        .add_system_set_to_stage(
            CoreStage::PostUpdate, 
            SystemSet::new()
                .with_system(publiclogic::position_translation)
                .with_system(publiclogic::size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
