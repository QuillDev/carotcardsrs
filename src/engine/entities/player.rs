use std::ops::{Add, Mul};
use tetra::{Context, input, time};
use tetra::graphics::Texture;
use tetra::input::Key;
use uuid::Uuid;
use crate::{GameContext, Vec2};
use crate::engine::constants::PLAYER_TILES_PER_SECOND;
use crate::engine::entities::game_object::GameObject;

pub struct Player {
    pos: Vec2<f32>,
    texture: Texture,
    uuid: Uuid,
    dir_vec: Vec2<f32>
}

impl Player {
    pub fn new(ctx: &mut GameContext, texture: Texture, pos: Vec2<f32>) -> Player {
        return Player {
            pos,
            texture,
            uuid: Uuid::new_v4(),
            dir_vec: Vec2::zero()
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, ctx: &mut GameContext, tctx: &mut Context) {
        let mut vec = Vec2::zero();
        if input::is_key_down(tctx, Key::W) {
            vec.y -= PLAYER_TILES_PER_SECOND;
        }
        if input::is_key_down(tctx, Key::S) {
            vec.y += PLAYER_TILES_PER_SECOND;
        }
        if input::is_key_down(tctx, Key::A) {
            vec.x -= PLAYER_TILES_PER_SECOND;
        }
        if input::is_key_down(tctx, Key::D) {
            vec.x += PLAYER_TILES_PER_SECOND;
        }
        self.dir_vec = vec;
    }

    fn render(&mut self, ctx: &mut Context) {
        let delta = time::get_delta_time(ctx).as_secs_f32();
        self.pos.x = self.pos.x + (self.dir_vec.x * delta);
        self.pos.y = self.pos.y + (self.dir_vec.y * delta);

        self.texture.draw(ctx, self.pos);

    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}