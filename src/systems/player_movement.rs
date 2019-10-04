use amethyst::core::timing::Time;
use amethyst::ecs::prelude::*;
use amethyst::input::{StringBindings, InputHandler};
use amethyst::ui::UiTransform;

use crate::components::Player;

const MOVEMENT_SPEED: f32 = 150.0;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiTransform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );
    
    fn run(&mut self, (players, mut transforms, input, time): Self::SystemData) {
        for (_player, transform) in (&players, &mut transforms).join() {
            transform.local_x += input.axis_value("horizontal").unwrap_or(0.0) * MOVEMENT_SPEED * time.delta_seconds();
            transform.local_y += input.axis_value("vertical").unwrap_or(0.0) * MOVEMENT_SPEED * time.delta_seconds();
        }
    }
}
