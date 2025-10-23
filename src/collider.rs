use glam::Vec2;
use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use crate::entity::{Entity, Transform};

pub trait Collider {
    fn collides(&self, this: &Transform, other: &Transform) -> bool {
        let collision_x = this.position.x + this.size.x >= other.position.x &&
            other.position.x + other.size.x >= this.position.x;
        let collision_y = this.position.y + this.size.y >= other.position.y &&
            other.position.y + other.size.y >= this.position.y;

        collision_x && collision_y
    }
}

pub struct Object {
    pub transform: Transform,
}

impl Object {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            transform: Transform::new_with_position(position, size),
        }
    }
}

impl Entity for Object {
    fn update(&mut self, delta_time: f32) {

    }

    fn render(&self) {
        draw_rectangle(self.transform.position.x, self.transform.position.y, self.transform.size.x, self.transform.size.y, Color::from_hex(0x323232))
    }

    fn update_collision(&mut self, collidables: &Vec<Object>) {
        // nothing it doesnt move
    }
}

impl Collider for Object {

}