use amethyst::prelude::*;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, MaterialTextureSet,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use amethyst::core::cgmath::Vector3;
use rand::prelude::*;
use std::sync::Mutex;

use components;
use utils;

lazy_static! {
    pub static ref RNG: Mutex<SmallRng> = Mutex::new(SmallRng::from_rng(thread_rng()).unwrap());
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub fn spawn_paddle(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut player_paddle = Transform::default();
    let x = ARENA_WIDTH / 2.0;
    player_paddle.translation = Vector3::new(x, 0.0 + 2.5, 0.0);
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };
    
    world.create_entity()
        .with(sprite_render)
        .with(components::Paddle::new(components::Side::Bottom))
        .with(player_paddle)
        .build();
}

// TODO: make variable by type and velocity
pub fn spawn_drop(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut drop_obj = Transform::default();
    let y = ARENA_HEIGHT - 4.0;
    drop_obj.translation = Vector3::new(RNG.lock().unwrap().gen_range(4.0, 80.0), 80.0, 0.0);
    //drop_obj.translation = Vector3::new(30.0, 50.0, 0.0);
    
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 1,
        flip_horizontal: false,
        flip_vertical: false,
    };
    
    world.create_entity()
        .with(sprite_render)
        .with(components::DropValue::new(components::DropType::Water))
        .with(drop_obj)
        .build();
}


pub fn spawn_sun(world: &mut World, sprite_sheet: SpriteSheetHandle, health: i32) {
    let mut sun = Transform::default();
    //sun.translation = Vector3::new(ARENA_WIDTH - 16.0, ARENA_HEIGHT - 16.0, 0.0);
    sun.translation = Vector3::new(85.0, 85.0, 0.0);
    let health_state = utils::get_health_state(health);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: utils::sun_sprite_mapping(&health_state),
        flip_horizontal: false,
        flip_vertical: false,
    };
    
    world.create_entity()
        .with(sprite_render)
        .with(components::Sun::new(health_state))
        .with(sun)
        .build();
}

pub fn initialise_camera(world: &mut World) {
    // represents camera's position. It's in 3d space
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let texture_id = 0;
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_id, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )

}

