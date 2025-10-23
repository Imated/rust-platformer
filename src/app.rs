use crate::background::Background;
use crate::entity::{Entity};

pub trait App {
    fn new() -> Self;
    fn update(&mut self, delta_time: f32);
    fn render(&self);
}

pub struct Game {
    entities: Vec<Box<dyn Entity>>
}

impl App for Game {
    fn new() -> Self {
        Self {
            entities: vec![Box::new(Background::new())]
        }
    }

    fn update(&mut self, delta_time: f32) {
        for mut entity in &mut self.entities {
            entity.update(delta_time);
        }
    }

    fn render(&self) {
        for mut entity in &self.entities {
            entity.render();
        }
    }
}