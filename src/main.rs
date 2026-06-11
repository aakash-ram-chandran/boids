mod boid;
mod flock;

use flock::Flock;
use macroquad::prelude::*;

/// Window title and size handed to macroquad on startup.
fn window() -> Conf {
    Conf {
        window_title: "Boids".to_owned(),
        window_width: 1000,
        window_height: 700,
        ..Default::default()
    }
}

/// Game loop: update the flock by the frame's elapsed time, then draw it.
#[macroquad::main(window)]
async fn main() {
    let mut bounds = vec2(screen_width(), screen_height());
    let mut flock = Flock::new(1000, bounds);

    loop {
        bounds = vec2(screen_width(), screen_height());
        flock.update(get_frame_time(), bounds);

        clear_background(Color::from_rgba(12, 14, 22, 255));
        for boid in &flock.boids {
            boid.draw();
        }

        draw_text(&format!("{} boids", flock.boids.len()), 12.0, 24.0, 22.0, GRAY);
        next_frame().await;
    }
}
