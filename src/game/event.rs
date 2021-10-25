use crate::game::context::Context;

pub trait EventHandler {
    fn update(&mut self, ctx: &Context);

    fn draw(&mut self);
}
