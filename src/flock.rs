//! The flock and the three steering rules that drive it:
//! separation, alignment, and cohesion.

use macroquad::prelude::*;

use crate::boid::Boid;

/// Tunable weights and ranges that shape how the flock behaves.
pub struct Settings {
    pub neighbor_radius: f32,
    pub separation_radius: f32,
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub max_speed: f32,
    pub max_force: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            neighbor_radius: 60.0,
            separation_radius: 24.0,
            separation: 1.6,
            alignment: 1.0,
            cohesion: 0.9,
            max_speed: 220.0,
            max_force: 240.0,
        }
    }
}

/// A collection of boids plus the settings they all obey.
pub struct Flock {
    pub boids: Vec<Boid>,
    pub settings: Settings,
}

impl Flock {
    /// Spawns `count` boids at random positions and headings inside `bounds`.
    pub fn new(count: usize, bounds: Vec2) -> Self {
        let boids = (0..count)
            .map(|_| {
                let pos = vec2(rand::gen_range(0.0, bounds.x), rand::gen_range(0.0, bounds.y));
                let angle = rand::gen_range(0.0, std::f32::consts::TAU);
                Boid::new(pos, Vec2::from_angle(angle) * 120.0)
            })
            .collect();

        Self {
            boids,
            settings: Settings::default(),
        }
    }

    /// Advances every boid by `dt` seconds: steer, cap speed, move, wrap.
    ///
    /// Steering is computed for all boids before any of them move, so each
    /// boid reacts to where its neighbors *were* this frame, not mid-update.
    pub fn update(&mut self, dt: f32, bounds: Vec2) {
        let accelerations: Vec<Vec2> = (0..self.boids.len())
            .map(|i| self.steer(i))
            .collect();

        for (boid, acc) in self.boids.iter_mut().zip(accelerations) {
            boid.vel = clamp_len(boid.vel + acc * dt, self.settings.max_speed);
            boid.pos += boid.vel * dt;
            wrap(&mut boid.pos, bounds);
        }
    }

    /// Combined steering force for one boid from its nearby flockmates.
    ///
    /// Blends separation (avoid crowding), alignment (match heading), and
    /// cohesion (move toward the group). Returns zero if it has no neighbors.
    fn steer(&self, index: usize) -> Vec2 {
        let s = &self.settings;
        let me = &self.boids[index];

        let mut separation = Vec2::ZERO;
        let mut heading = Vec2::ZERO;
        let mut center = Vec2::ZERO;
        let mut neighbors = 0;

        for (j, other) in self.boids.iter().enumerate() {
            if j == index {
                continue;
            }

            let offset = me.pos - other.pos;
            let dist = offset.length();
            if dist > s.neighbor_radius || dist == 0.0 {
                continue;
            }

            if dist < s.separation_radius {
                separation += offset / dist;
            }
            heading += other.vel;
            center += other.pos;
            neighbors += 1;
        }

        if neighbors == 0 {
            return Vec2::ZERO;
        }

        let align = steer_toward(heading / neighbors as f32, me.vel, s.max_speed, s.max_force);
        let cohere = steer_toward(center / neighbors as f32 - me.pos, me.vel, s.max_speed, s.max_force);
        let separate = steer_toward(separation, me.vel, s.max_speed, s.max_force);

        separate * s.separation + align * s.alignment + cohere * s.cohesion
    }
}

/// Turns a desired direction into a turn force, capped by `max_force`.
///
/// Aims at full speed along `desired`, then returns the change from the
/// current velocity so the boid eases into the new heading instead of snapping.
fn steer_toward(desired: Vec2, vel: Vec2, max_speed: f32, max_force: f32) -> Vec2 {
    if desired == Vec2::ZERO {
        return Vec2::ZERO;
    }
    clamp_len(desired.normalize() * max_speed - vel, max_force)
}

/// Shortens `v` to `max` if it's longer, keeping its direction.
fn clamp_len(v: Vec2, max: f32) -> Vec2 {
    if v.length() > max {
        v.normalize() * max
    } else {
        v
    }
}

/// Wraps a position around the screen edges, so boids leaving one side
/// reappear on the opposite one.
fn wrap(pos: &mut Vec2, bounds: Vec2) {
    if pos.x < 0.0 {
        pos.x += bounds.x;
    } else if pos.x > bounds.x {
        pos.x -= bounds.x;
    }
    if pos.y < 0.0 {
        pos.y += bounds.y;
    } else if pos.y > bounds.y {
        pos.y -= bounds.y;
    }
}
