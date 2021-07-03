use macroquad::prelude::*;
use macroquad::ui::root_ui;
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
    let mut mode = "2d";
    loop {
        clear_background(GRAY);
        if root_ui().button(None, "2D Visualization") {
            mode = "2d";
            break;
        } else if root_ui().button(None, "3D Visualization") {
            mode = "3d";
            break;
        }
        next_frame().await;
    }
    match mode {
        "2d" => boids2d::run().await,
        "3d" => boids3d::run().await,
        _ => {},
    }
}
