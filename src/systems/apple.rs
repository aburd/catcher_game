use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::catcher::{Apple, ARENA_HEIGHT, APPLE_HEIGHT};

pub struct AppleSystem;

impl<'s> System<'s> for AppleSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Apple>,
  );

  fn run(&mut self, (mut transforms, apples): Self::SystemData) {
    let apple_min = ARENA_HEIGHT * 0.9;
    let apple_max = APPLE_HEIGHT * 0.5;

    for (apple, transform) in (&apples, &mut transforms).join() {
      let apple_y = transform.translation().y;
      let new_pos_y = apple_y - apple.speed;
      
      let pos_y = {
        if new_pos_y <= apple_max {
          ARENA_HEIGHT * 0.9
        } else {
          new_pos_y
        }
      };
      transform.set_y(
        (pos_y)
          .min(apple_min)
          .max(apple_max),
      );
    }
  }
}
