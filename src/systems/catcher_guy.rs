use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use crate::catcher::{CatcherGuy, ARENA_WIDTH, CATCHER_GUY_WIDTH};

pub struct CatcherGuySystem;

impl<'s> System<'s> for CatcherGuySystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    WriteStorage<'s, SpriteRender>,
    ReadStorage<'s, CatcherGuy>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, mut sprite_renders, catcher_guys, input): Self::SystemData) {
    for (catcher_guy, transform, sprite_render) in (&catcher_guys, &mut transforms, &mut sprite_renders).join() {
      let movement = input.axis_value("catcher_guy");
      if let Some(mv_amount) = movement {
        let scaled_amount = 1.2 * mv_amount as f32;
        let catcher_x = transform.translation().x;
        transform.set_x(
          (catcher_x + scaled_amount)
            .min(ARENA_WIDTH - CATCHER_GUY_WIDTH * 0.5)
            .max(CATCHER_GUY_WIDTH * 0.5),
        );
        if sprite_render.sprite_number < 4 {
          sprite_render.sprite_number += 1;
        } else {
          sprite_render.sprite_number = 0;
        }
      }
    }
  }
}
