use macroquad::math::i32;
use macroquad::prelude::*;
use crate::constants::*;
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

    fn draw_stripe(&self, x : i32, slant: i8, thickness: i8, color: Color) {
        draw_affine_parallelogram(vec3(x as f32, 0.0, 0.0), vec3(slant as f32, screen_height(), 0.0), vec3(thickness as f32, 0.0, 0.0), None, color);
    }
}

impl Updatable for Background {
    fn update(&mut self, delta_time: f32) {
        self.transform.add_position_y(self.speed);
    }
}

impl Renderable for Background {
    fn render(&self) {
        let num_stripes: i32 = (screen_width() / 85.0).ceil() as i32 + 1;
        for i in 0..num_stripes {
            let mut color = Color::from_hex(BACKGROUND_DARK);
            if (i % 2 == 0) {
                color = Color::from_hex(BACKGROUND_LIGHT);
            }
            self.draw_stripe(i * BACKGROUND_STRIPE_OFFSET as i32, -BACKGROUND_STRIPE_OFFSET, BACKGROUND_STRIPE_OFFSET, color);
        }

    }
}