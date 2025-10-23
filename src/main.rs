use macroquad::miniquad::window::order_quit;
use macroquad::prelude::*;
use crate::app::{App, Game};

mod app;
mod background;
mod entity;
mod constants;
mod player;

#[macroquad::main("Game")]
async fn main() {
    let mut app = Game::new();
    loop {
        //update
        app.update(get_frame_time());

        if is_key_down(KeyCode::Escape) {
            order_quit();
        }

        //render
        app.render();
        next_frame().await;
    }
}
