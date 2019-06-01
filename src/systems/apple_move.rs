use amethyst::{
    core::timing::Time,
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::catcher::Apple;

pub struct AppleMoveSystem;

impl<'s> System<'s> for AppleMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Apple>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, apples, time): Self::SystemData) {
        for (apple, transform) in (&apples, &mut transforms).join() {
            let apple_y = transform.translation().y;
            let pos_y = apple_y - (apple.speed * time.delta_seconds());

            transform.set_y(pos_y);
        }
    }
}
