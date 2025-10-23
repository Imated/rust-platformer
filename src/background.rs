use macroquad::math::i32;
use macroquad::prelude::*;
use crate::constants::*;
use crate::entity::{Transform, Entity};

pub struct Background {
    transform: Transform,
    speed: f32
}

impl Background {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            speed: 25.0,
        }
    }

    fn draw_stripe(&self, x : f32, slant: f32, thickness: f32, color: Color) {
        draw_affine_parallelogram(vec3(x, 0.0, 0.0), vec3(slant, screen_height(), 0.0), vec3(thickness, 0.0, 0.0), None, color);
    }
}

impl Entity for Background {
    fn update(&mut self, delta_time: f32) {
        self.transform.add_position_x(self.speed * delta_time);
        let period = BACKGROUND_STRIPE_OFFSET * 2.0;
        if self.transform.position.x >= period {
            self.transform.set_position_x(0.0);
        }
    }

    fn render(&self) {
        let start_offset = -2;
        let num_stripes: i32 = ((screen_width() / BACKGROUND_STRIPE_OFFSET).ceil() as i32) + 1;
        for i in start_offset..num_stripes {
            let color = if i % 2 == 0 {
                Color::from_hex(BACKGROUND_LIGHT)
            } else {
                Color::from_hex(BACKGROUND_DARK)
            };
            let x_pos = (i as f32) * BACKGROUND_STRIPE_OFFSET + self.transform.position.x;
            self.draw_stripe(x_pos, -BACKGROUND_STRIPE_OFFSET, BACKGROUND_STRIPE_OFFSET, color);
        }
    }
}