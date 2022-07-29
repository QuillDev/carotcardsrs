use tetra::Context;
use uuid::Uuid;
use crate::GameContext;

pub trait GameObject {
    fn update(&mut self, ctx: &mut GameContext, tctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
    fn uuid(&self) -> Uuid;
}