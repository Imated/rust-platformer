#![allow(unused_parens)]
#![allow(dead_code)]
use glam::Vec2;
use crate::collider::{Collider, Object};

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub size: Vec2
}

impl Transform {
    pub fn new(size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0f32,
            size,
        }
    }

    pub fn new_with_position(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            rotation: 0f32,
            size,
        }
    }

    pub fn set_position_x(&mut self, x: f32) {
        self.position.x = x;
    }

    pub fn set_position_y(&mut self, y: f32) {
        self.position.y = y;
    }

    pub fn set_size_x(&mut self, x: f32) {
        self.size.x = x;
    }

    pub fn set_size_y(&mut self, y: f32) {
        self.size.y = y;
    }

    pub fn add_position_x(&mut self, x: f32) {
        self.position.x += x;
    }

    pub fn add_position_y(&mut self, y: f32) {
        self.position.y += y;
    }

    pub fn add_size_x(&mut self, x: f32) {
        self.size.x += x;
    }

    pub fn add_size_y(&mut self, y: f32) {
        self.size.y += y;
    }

    pub fn add_rotation(&mut self, rotation: f32) {
        self.rotation += rotation;
    }
    pub fn add_position(&mut self, position: Vec2) {
        self.position += position;
    }
}

pub trait Entity {
    fn update(&mut self, delta_time: f32);
    fn render(&self);
    fn update_collision(&mut self, collidables: &Vec<Object>);
}