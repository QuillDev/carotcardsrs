use std::collections::HashSet;
use tetra::Context;
use uuid::Uuid;
use crate::{GameContext, GameState};
use crate::engine::entities::game_object::GameObject;

pub struct Engine {
    pub ctx: GameContext,
    objects: Vec<Box<dyn GameObject>>,
}

// TODO: Handle render order, Handle removing in batches
impl Engine {

    /// Create a new engine with default state
    pub fn new(tctx: &mut Context) -> tetra::Result<Engine> {
        let ctx = GameContext::new(tctx);
        let engine = Engine {
            ctx,
            objects: Vec::new()
        };

        Ok(engine)
    }

    /// Render the engine
    pub fn render(&mut self, tctx: &mut Context) {
        for obj in self.objects.iter_mut() {
            obj.render(tctx);
        }
    }

    /// Update the engine
    pub fn update(&mut self, tctx: &mut Context) {
        for obj in self.objects.iter_mut() {
            obj.update(&mut self.ctx, tctx);
        }
    }

    pub fn ctx(&mut self) -> &mut GameContext {
        return &mut self.ctx;
    }

    /// Add an object to the engine
    pub fn add_object(&mut self, obj: Box<dyn GameObject>) {
        self.objects.push(obj);
    }
}