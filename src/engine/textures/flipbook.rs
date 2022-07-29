use macroquad::prelude::Texture2D;

pub trait Flipbook {
    fn update(&mut self);

    fn get_current_texture(&self) -> Texture2D;
}