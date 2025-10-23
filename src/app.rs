use glam::Vec2;
use macroquad::window::screen_width;
use crate::background::Background;
use crate::collider::Object;
use crate::entity::{Entity};
use crate::player::Player;

pub trait App {
    fn new() -> Self;
    fn update(&mut self, delta_time: f32);
    fn render(&self);
}

pub struct Game {
    entities: Vec<Box<dyn Entity>>,
    objects: Vec<Object>
}

impl App for Game {
    fn new() -> Self {
        Self {
            entities: vec![Box::new(Background::new()), Box::new(Player::new())],
            objects: vec![Object::new(Vec2::new(0.0, 400.0), Vec2::new(screen_width(), 40.0))]
        }
    }

    fn update(&mut self, delta_time: f32) {
        for entity in &mut self.entities {
            entity.update(delta_time);
            entity.update_collision(&self.objects);
        }
    }

    fn render(&self) {
        for entity in &self.entities {
            entity.render();
        }

        for object in &self.objects {
            object.render();
        }
    }
}