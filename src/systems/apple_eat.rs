use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System},
};

use crate::catcher::{Apple, CatcherGuy};

pub struct AppleEatSystem;

impl<'s> System<'s> for AppleEatSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Apple>,
        ReadStorage<'s, CatcherGuy>,
        Entities<'s>,
    );

    fn run(&mut self, (transforms, apples, catcher_guys, entities): Self::SystemData) {
        for (e, apple, a_transform) in (&*entities, &apples, &transforms).join() {
            for (catcher_guy, c_transform) in (&catcher_guys, &transforms).join() {
                let catcher_rect = Rectangle {
                    top: c_transform.translation().y + (catcher_guy.height / 2.0),
                    bottom: c_transform.translation().y - (catcher_guy.height / 2.0),
                    right: c_transform.translation().x + (catcher_guy.width / 2.0),
                    left: c_transform.translation().x - (catcher_guy.width / 2.0),
                };
                // check if apple has fallen to the bottom (lose case)
                let apple_y = a_transform.translation().y;
                let apple_x = a_transform.translation().x;
                if apple_y <= apple.max {
                    println!("Lose!");
                };
                // check if an apple has been eaten
                if catcher_rect.point_in(apple_x, apple_y) {
                    // a_transform.set_y(apple.min);
                    entities.delete(e);
                }
            }
        }
    }
}

struct Rectangle {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

impl Rectangle {
    fn point_in(&self, x: f32, y: f32) -> bool {
        if y < self.top && y > self.bottom && x < self.right && x > self.left {
            true
        } else {
            false
        }
    }
}
