use tetra::Context;
use tetra::graphics::Texture;
use uuid::Uuid;
use crate::{GameContext, TextureLibrary, Vec2};
use crate::engine::entities::game_object::GameObject;

#[derive(Clone)]
pub struct Tile {
    texture: Texture,
    pub pos: Vec2<f32>,
    tags: Vec<String>,
    uuid: Uuid,
}

impl Tile {
    pub fn positioned(texture: Texture, pos: Vec2<f32>) -> Tile {
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

    pub fn new(texture: Texture) -> Tile {
        return Tile::positioned(texture, Vec2::zero());
    }
}

impl GameObject for Tile {
    fn update(&mut self, _: &mut GameContext, _: &mut Context) {}

    fn render(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, self.pos);
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}