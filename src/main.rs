extern crate core;

mod textures;
mod engine;

use std::collections::HashMap;
use tetra::{Context, ContextBuilder, graphics, State};
use tetra::graphics::{Color, Texture};
use tetra::input::Key::O;
use tetra::math::Vec2;
use crate::engine::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::textures::texture_library::TextureLibrary;

struct GameState {
    texture_lib: TextureLibrary
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let game_state = GameState {
            texture_lib: TextureLibrary::new(ctx)?
        };

        Ok(game_state)
    }
}

impl State for GameState {
    // game physics / logic processing done here
    fn update(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        Ok(())
    }

    // rendering done here
    fn draw(&mut self, ctx: &mut Context) -> Result<(), tetra::TetraError> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.texture_lib.DIRT.draw(ctx, Vec2::new(0.0, 0.0));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("CarotCards", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
