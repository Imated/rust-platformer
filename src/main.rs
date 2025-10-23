use macroquad::prelude::get_frame_time;
use macroquad::window::next_frame;
use crate::app::{App, Game};

mod app;
mod background;
mod entity;

#[macroquad::main("Game")]
async fn main() {
    let mut app = Game::new();
    loop {
        app.update(get_frame_time());
        app.render();
        next_frame().await;
    }
}
