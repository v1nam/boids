use macroquad::prelude::*;
use std::io;
mod boids2d;
mod boids3d;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_width: 1280,
        window_height: 760,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut mode = String::new();
    println!("Enter mode (2d, 3d): ");
    io::stdin().read_line(&mut mode).expect("¯\\_(ツ)_/¯");
    mode.make_ascii_lowercase();
    let mode = mode.trim();
    if mode == "2d" {
        boids2d::run().await;
    } else if mode == "3d" {
        boids3d::run().await;
    }
}
