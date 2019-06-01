use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const CATCHER_GUY_WIDTH: f32 = 40.0;
pub const CATCHER_GUY_HEIGHT: f32 = 20.0;
pub const APPLE_WIDTH: f32 = 6.0;
pub const APPLE_HEIGHT: f32 = 6.0;
pub const APPLE_INITIAL_SPEED: f32 = 20.0;

pub struct Catcher;

impl SimpleState for Catcher {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<CatcherGuy>();
        world.register::<Apple>();

        let spritesheet_handle_catcher = load_catcher_spritesheet(world);
        let spritesheet_handle_apple = load_apple_spritesheet(world);
        initialise_camera(world);
        initialise_catcher(world, spritesheet_handle_catcher);
        spawn_apple(world, spritesheet_handle_apple);
    }
}

// catch_guy
pub struct CatcherGuy {
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl CatcherGuy {
    fn new(speed: f32) -> Self {
        CatcherGuy {
            speed,
            width: CATCHER_GUY_WIDTH,
            height: CATCHER_GUY_HEIGHT,
        }
    }
}

impl Component for CatcherGuy {
    type Storage = DenseVecStorage<Self>;
}

// apple
pub struct Apple {
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl Apple {
    fn new(speed: f32) -> Self {
        Apple {
            speed,
            width: APPLE_WIDTH,
            height: APPLE_HEIGHT,
        }
    }
}

impl Component for Apple {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_catcher(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut catcher_transform = Transform::default();
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 5.0;
    catcher_transform.set_xyz(x, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(CatcherGuy::new(1.0))
        .with(catcher_transform)
        .build();
}

pub fn spawn_apple(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut apple_transform = Transform::default();
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT * 0.9;
    apple_transform.set_xyz(x, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Apple::new(APPLE_INITIAL_SPEED))
        .with(apple_transform)
        .build();
}

fn load_catcher_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/catcher_guy.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/catcher_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn load_apple_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/apple.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/apple_spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
