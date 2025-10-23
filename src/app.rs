use crate::background::Background;
use crate::entity::{Renderable, Updatable};

pub trait App {
    fn new() -> Self;
    fn update(&mut self, delta_time: f32);
    fn render(&self);
}

pub struct Game {
    background: Background
}

impl App for Game {
    fn new() -> Self {
        Self {
            background: Background::new()
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.background.update(delta_time);
    }

    fn render(&self) {
        self.background.render();
    }
}