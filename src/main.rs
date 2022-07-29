extern crate core;

use std::error::Error;

use macroquad::prelude::*;

use engine::textures::texture_library::TextureLibrary;

use crate::engine::constants::{UPDATE_RATE, WINDOW_HEIGHT, WINDOW_WIDTH, WORLD_UNIT};
use crate::engine::context::game_context::GameContext;
use crate::engine::engine::Engine;
use crate::engine::entities::player::Player;
use crate::engine::tiles::tile::Tile;
use crate::engine::tiles::tileset::Tileset;

mod engine;


struct State {
    elapsed: f32
}

/// Create the game window.
#[macroquad::main("CarotCards")]
async fn main() -> Result<(), Box<dyn Error>>{

    let mut state = State{ elapsed: 0.0 };
    // texture loading
    let texture_lib = TextureLibrary::new().await?;
    // top level components
    let mut engine = Engine::new(texture_lib)?;

    let tileset = Tileset::load(
        &engine.texture_lib,
        "resources/maps/dev.csv",
        Vec2::new(
            WINDOW_WIDTH / 2.0 - WORLD_UNIT * 10.0 / 2.0,
            WINDOW_HEIGHT / 2.0 - WORLD_UNIT * 10.0 / 2.0,
        ),
    );

    engine.add_object(Box::new(tileset));

    loop {
        state.elapsed += get_frame_time();

        while state.elapsed > UPDATE_RATE {
            engine.update();
            state.elapsed -= UPDATE_RATE;
        }

        clear_background(SKYBLUE);
        engine.render();
        next_frame().await;
    }
}
