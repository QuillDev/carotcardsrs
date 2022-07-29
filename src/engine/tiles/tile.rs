use macroquad::color::WHITE;
use macroquad::prelude::*;
use uuid::Uuid;
use crate::{GameContext, TextureLibrary, Vec2};
use crate::engine::entities::game_object::GameObject;

#[derive(Clone)]
pub struct Tile {
    texture: Texture2D,
    pub pos: Vec2,
    tags: Vec<String>,
    uuid: Uuid,
}

impl Tile {
    pub fn positioned(texture: Texture2D, pos: Vec2) -> Tile {
        return Tile {
            texture,
            pos,
            tags: Vec::new(),
            uuid: Uuid::new_v4()
        }
    }

    pub fn new_tex(lib: &TextureLibrary, name: &str) -> Tile {
        return Tile::new(lib.get_texture(name));
    }

    pub fn new(texture: Texture2D) -> Tile {
        return Tile::positioned(texture, Vec2::zero());
    }
}

impl GameObject for Tile {
    fn update(&mut self) {}

    fn render(&mut self) {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}