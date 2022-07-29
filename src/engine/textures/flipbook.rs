use macroquad::prelude::{Texture2D, get_frame_time};

pub trait Flipbook {
    fn update(&mut self);

    fn get_current_texture(&self) -> Texture2D;
}

pub struct TimedFlipbook {
    textures : Vec<Texture2D>,
    frame_duration : f32,
    frame_index : usize,
    remaining_frame_time : f32
}

impl TimedFlipbook {
    pub fn new(textures : Vec<Texture2D>, frame_duration : f32) -> TimedFlipbook{
        TimedFlipbook { 
            textures,
            frame_duration,
            frame_index : 0,
            remaining_frame_time : frame_duration
        }
    }
}

impl Flipbook for TimedFlipbook {
    fn update(&mut self) {
        self.remaining_frame_time -= get_frame_time();

        if self.remaining_frame_time <= 0.0 {
            self.frame_index += 1;

            if self.frame_index == self.textures.len() {
                self.frame_index = 0;
            }

            self.remaining_frame_time = self.frame_duration;
        }

    }

    fn get_current_texture(&self) -> Texture2D {
        self.textures[self.frame_index]
    }
}