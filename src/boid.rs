//! A single agent in the flock: where it is and where it's heading.

use macroquad::prelude::*;

/// One boid. `pos` is in pixels, `vel` in pixels per second.
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    /// Creates a boid at `pos` travelling with velocity `vel`.
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel }
    }

    /// Draws the boid as a triangle pointing in its direction of travel,
    /// colored by heading so different directions get different hues.
    pub fn draw(&self) {
        let dir = self.vel.normalize_or_zero();
        let perp = vec2(-dir.y, dir.x);

        let tip = self.pos + dir * 9.0;
        let left = self.pos - dir * 5.0 + perp * 4.0;
        let right = self.pos - dir * 5.0 - perp * 4.0;

        let hue = (dir.y.atan2(dir.x) / std::f32::consts::TAU) + 0.5;
        draw_triangle(tip, left, right, macroquad::color::hsl_to_rgb(hue, 0.8, 0.6));
    }
}
