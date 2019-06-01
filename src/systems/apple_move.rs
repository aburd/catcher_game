use amethyst::{
    core::timing::Time,
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::catcher::{Apple, APPLE_HEIGHT, ARENA_HEIGHT};

pub struct AppleMoveSystem;

impl<'s> System<'s> for AppleMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Apple>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, apples, time): Self::SystemData) {
        let apple_min = ARENA_HEIGHT * 0.9;
        let apple_max = APPLE_HEIGHT * 0.5;

        for (apple, transform) in (&apples, &mut transforms).join() {
            let apple_y = transform.translation().y;
            let apple_x = transform.translation().x;
            let new_pos_y = apple_y - (apple.speed * time.delta_seconds());

            let pos_y = {
                if new_pos_y <= apple_max {
                    ARENA_HEIGHT * 0.9
                } else {
                    new_pos_y
                }
            };
            transform.set_y((pos_y).min(apple_min).max(apple_max));
        }
    }
}
