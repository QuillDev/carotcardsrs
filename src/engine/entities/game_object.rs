use uuid::Uuid;
use crate::GameContext;

pub trait GameObject {
    fn update(&mut self);
    fn render(&mut self);
    fn uuid(&self) -> Uuid;
}