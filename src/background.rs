use macroquad::prelude::*;
use crate::entity::{Transform, Renderable, Updatable};

pub struct Background {
    transform: Transform,
    slide_timer: f32,
    speed: f32
}

impl Background {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            slide_timer: 0f32,
            speed: 0.5,
        }
    }

    fn draw_stripe(&self, x : i32, slant: f32, thickness: f32) {
        draw_affine_parallelogram(vec3(x as f32, 0.0, 0.0), vec3(slant, screen_height(), 0.0), vec3(thickness, 0.0, 0.0), None, BLUE);
    }
}

impl Updatable for Background {
    fn update(&mut self, delta_time: f32) {
        self.transform.add_position_y(self.speed);
    }
}

impl Renderable for Background {
    fn render(&self) {
        self.draw_stripe(0, -85.0, 85.0);
    }
}