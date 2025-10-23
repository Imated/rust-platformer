use glam::f32::{Vec2};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{Color};
use macroquad::shapes::draw_rectangle;
use crate::collider::{Collider, Object};
use crate::constants::{GRAVITY, PLAYER_COLOR};
use crate::entity::{Entity, Transform};

pub struct Player {
    transform: Transform,
    velocity: Vec2,
    move_speed: f32,
    jump_height: f32,
    grounded: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(Vec2::new(50.0, 50.0)),
            velocity: Vec2::ZERO,
            move_speed: 250.0,
            jump_height: 260.0,
            grounded: true,
        }
    }

    fn jump(&mut self) {
        if self.grounded {
            self.velocity.y = -self.jump_height;
            self.grounded = false;
        }
    }

    fn update_movement(&mut self, input: i32, delta_time: f32) {
        self.velocity.x = input as f32 * self.move_speed;
        if !self.grounded {
            self.velocity.y -= GRAVITY * delta_time;
        }

        self.transform.add_position(self.velocity * delta_time);

        if self.grounded {
            self.velocity.y = 0.0;
        }
    }
}

impl Entity for Player {
    fn update(&mut self, delta_time: f32) {
        let input = (is_key_down(KeyCode::D) as i32) - (is_key_down(KeyCode::A) as i32);
        self.update_movement(input, delta_time);
        if is_key_down(KeyCode::Space) {
            self.jump();
        }
    }

    fn render(&self) {
        draw_rectangle(self.transform.position.x, self.transform.position.y, self.transform.size.x, self.transform.size.y, Color::from_hex(PLAYER_COLOR));
    }

    fn update_collision(&mut self, collidables: &Vec<Object>) {
        for collider in collidables {
            if self.collides(&self.transform, &collider.transform) {
                self.grounded = true;
            }
            else {
                self.grounded = false;
            }
        }
    }
}

impl Collider for Player {
    
}