extern crate amethyst;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use amethyst::{
    prelude::*,
    core::transform::{TransformBundle},
    renderer::{DisplayConfig, DrawSprite, Event, Pipeline, RenderBundle, Stage, VirtualKeyCode},
    utils::application_root_dir,
};

mod components;
mod resources;
mod entities;
mod utils;
mod pong;

use resources::{Score};
use pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    // path to settings, window creation
    let app_root = application_root_dir();
    let path = format!(
        "{}/examples/pong_tutorial_01/resources/display_config.ron",
        app_root
    );
    let config = DisplayConfig::load(&path);

    // rendering
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );
    
    // actual run
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    Ok(())
}
