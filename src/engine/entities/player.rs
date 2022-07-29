use std::ops::{Add, Mul};
use macroquad::prelude::*;
use macroquad::time;
use uuid::Uuid;
use crate::engine::constants::PLAYER_TILES_PER_SECOND;
use crate::engine::entities::game_object::GameObject;
use crate::KeyCode;

pub struct Player {
    pub pos: Vec2,
    texture: Texture2D,
    uuid: Uuid,
    dir_vec: Vec2
}

impl Player {
    pub fn new(texture: Texture2D, pos: Vec2) -> Player {
        return Player {
            pos,
            texture,
            uuid: Uuid::new_v4(),
            dir_vec: Vec2::default()
        }
    }
}

impl GameObject for Player {
    fn update(&mut self) {
        let mut vec = Vec2::zero();
        if is_key_down(KeyCode::W) {
            vec.y -= PLAYER_TILES_PER_SECOND;
        }
        if is_key_down(KeyCode::S) {
            vec.y += PLAYER_TILES_PER_SECOND;
        }
        if is_key_down(KeyCode::A) {
            vec.x -= PLAYER_TILES_PER_SECOND;
        }
        if is_key_down(KeyCode::D) {
            vec.x += PLAYER_TILES_PER_SECOND;
        }
        self.dir_vec = vec;
    }

    fn render(&mut self) {
        let delta = get_frame_time();
        self.pos.x = self.pos.x + (self.dir_vec.x * delta);
        self.pos.y = self.pos.y + (self.dir_vec.y * delta);

        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}