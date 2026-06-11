mod boid;
mod flock;
mod grid;

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
    let mut flock = Flock::new(10000, bounds);

    // How many boids to add/remove per frame while a key is held.
    let step = 50;

    loop {
        bounds = vec2(screen_width(), screen_height());

        // Hold Up to spawn more boids, Down to remove them.
        if is_key_down(KeyCode::Up) {
            flock.add(step, bounds);
        }
        if is_key_down(KeyCode::Down) {
            flock.remove(step);
        }

        flock.update(get_frame_time(), bounds);

        clear_background(Color::from_rgba(12, 14, 22, 255));
        for boid in &flock.boids {
            boid.draw();
        }

        let info = format!("{} fps   {} boids   [Up/Down to add/remove]", get_fps(), flock.boids.len());
        draw_text(&info, 12.0, 24.0, 22.0, GRAY);
        next_frame().await;
    }
}
