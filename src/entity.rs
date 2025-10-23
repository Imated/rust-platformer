#![allow(unused_parens)]
#![allow(dead_code)]
use glam::Vec2;

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0f32,
            scale: Vec2::ONE,
        } 
    }

    pub fn set_position_x(&mut self, x: f32) {
        self.position.x = x;
    }

    pub fn set_position_y(&mut self, y: f32) {
        self.position.y = y;
    }

    pub fn set_scale_x(&mut self, x: f32) {
        self.scale.x = x;
    }

    pub fn set_scale_y(&mut self, y: f32) {
        self.scale.y = y;
    }

    pub fn add_position_x(&mut self, x: f32) {
        self.position.x += x;
    }

    pub fn add_position_y(&mut self, y: f32) {
        self.position.y += y;
    }

    pub fn add_scale_x(&mut self, x: f32) {
        self.scale.x += x;
    }

    pub fn add_scale_y(&mut self, y: f32) {
        self.scale.y += y;
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
}