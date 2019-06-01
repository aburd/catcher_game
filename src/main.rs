extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    Blend, BlendChannel, ColorMask, DisplayConfig, DrawFlat2D, Equation, Event, Factor, Pipeline,
    RenderBundle, Stage, VirtualKeyCode,
};
use amethyst::utils::application_root_dir;

mod catcher;
use crate::catcher::Catcher;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let path = format!("{}/resources/display_config.ron", application_root_dir(),);

    let config = DisplayConfig::load(&path);

    let color_blend_channel = BlendChannel {
        equation: Equation::Add,
        source: Factor::One,
        destination: Factor::Zero,
    };
    let alpha_blend_channel = BlendChannel {
        equation: Equation::Add,
        source: Factor::One,
        destination: Factor::Zero,
    };
    let blend = Blend {
        color: color_blend_channel,
        alpha: alpha_blend_channel,
    };
    let pass = DrawFlat2D::new().with_transparency(ColorMask::all(), blend, None);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(pass),
    );

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::CatcherGuySystem,
            "catcher_guy_system",
            &["input_system"],
        )
        .with(systems::AppleMoveSystem, "apple_system", &[]);

    let mut game = Application::new("./", Catcher, game_data)?;
    game.run();

    Ok(())
}
