use amethyst::ecs::prelude::*;
use amethyst::ui::UiTransform;

use crate::player::Player;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiTransform>,
        // get time...
    );
    
    fn run(&mut self, (players, mut transforms): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            transform.local_x += 2.0;
            transform.local_y += 1.0;
        }
    }
}
