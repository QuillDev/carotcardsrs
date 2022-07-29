use std::collections::HashSet;
use std::error::Error;

use macroquad::prelude::camera::mouse::Camera;
use macroquad::prelude::Vec2;
use uuid::Uuid;

use crate::{GameContext, Player, TextureLibrary, WINDOW_HEIGHT, WINDOW_WIDTH, WORLD_UNIT};
use crate::engine::constants::PLAYER_TILES_PER_SECOND;
use crate::engine::entities::game_object::GameObject;

pub struct Engine {
    ctx: GameContext,
    pub texture_lib: TextureLibrary,
    objects: Vec<Box<dyn GameObject>>,
    player: Player,
}

// TODO: Handle render order, Handle removing in batches
impl Engine {
    /// Create a new engine with default state
    pub fn new(texture_lib: TextureLibrary) -> Result<Engine, Box<dyn Error>> {
        let ctx = GameContext::new();
        // create the player_idle
        let player_texture = texture_lib.get_texture("chick");
        let player_pos = Vec2::new(
            WINDOW_WIDTH / 2.0 - player_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - player_texture.height() as f32 / 2.0,
        );

        let player = Player::new(&texture_lib, player_pos);

        let engine = Engine {
            ctx,
            objects: Vec::new(),
            player,
            texture_lib,
        };

        Ok(engine)
    }

    /// Render the engine
    pub fn render(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.render();
        }
        self.player.render();
    }

    /// Update the engine
    pub fn update(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.update();
        }
        self.player.update();
    }

    pub fn ctx(&mut self) -> &mut GameContext {
        return &mut self.ctx;
    }

    /// Add an object to the engine
    pub fn add_object(&mut self, obj: Box<dyn GameObject>) {
        self.objects.push(obj);
    }
}
