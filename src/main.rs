extern crate core;

mod engine;

use std::borrow::BorrowMut;
use std::fmt::format;
use std::mem;
use std::sync::Mutex;
use tetra::{Context, ContextBuilder, graphics, State, time};
use tetra::graphics::{Color, text};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use crate::engine::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::engine::tiles::tile::Tile;
use engine::textures::texture_library::TextureLibrary;
use crate::engine::context::game_context::GameContext;
use crate::engine::engine::Engine;
use crate::engine::entities::player::Player;
use crate::engine::tiles::tileset::{TileRow, Tileset};

struct GameState {
    texture_lib: TextureLibrary,
    engine: Engine,
    fps_text: Text,
}

impl GameState {
    /// create a new instance of game state
    fn new(tctx: &mut Context) -> tetra::Result<GameState> {
        // top level components
        let mut engine = Engine::new(tctx)?;

        // texture loading
        let texture_lib = TextureLibrary::new(tctx)?;

        let fps_text = Text::new(
            "FPS: 0.0",
            Font::vector(tctx, "resources/font/DejaVuSansMono.ttf", 16.0)?,
        );

        // instances
        let player = Player::new(engine.ctx(), texture_lib.get_texture("chick"), Vec2::zero());
        let tileset = Tileset::load(&texture_lib, "resources/maps/dev.csv", Vec2::zero());

        engine.add_object(Box::new(tileset));
        engine.add_object(Box::new(player));

        let game_state = GameState {
            // library of our textures
            texture_lib,
            engine,
            fps_text
        };

        Ok(game_state)
    }
}

/// Add tetra state hooks to our game state
impl State for GameState {
    /// game physics / logic processing done here
    fn update(&mut self, tctx: &mut Context) -> Result<(), tetra::TetraError> {
        self.engine.update(tctx);
        Ok(())
    }

    /// rendering done here
    fn draw(&mut self, tctx: &mut Context) -> Result<(), tetra::TetraError> {
        graphics::clear(tctx, Color::rgb(0.392, 0.584, 0.929));
        self.engine.render(tctx);
        self.fps_text.set_content(format!("FPS: {}", time::get_fps(tctx) as u32));
        self.fps_text.draw(tctx, Vec2::zero());
        Ok(())
    }
}

/// Create the game window.
fn main() -> tetra::Result {
    ContextBuilder::new("CarotCards", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .vsync(false)
        .build()?
        .run(GameState::new)
}
