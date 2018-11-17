/**
 * Core game logic
 * */
use amethyst::prelude::*;
use entities;
use components;

pub struct Pong;

impl<'a, 'b> SimpleState<'a, 'b> for Pong {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let sprite_sheet_handle = entities::load_sprite_sheet(world);

        world.register::<components::Paddle>();
        world.register::<components::DropValue>();

        entities::spawn_paddle(world, sprite_sheet_handle.clone());
        entities::spawn_drop(world, sprite_sheet_handle);
        entities::initialise_camera(world);
    }
}

