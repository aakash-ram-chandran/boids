//! A uniform spatial grid for fast neighbor lookups.
//!
//! Boids only interact within `neighbor_radius`, so instead of checking every
//! other boid (O(n^2)), we bucket boids into square cells sized to that radius.
//! Any neighbor in range must then live in the boid's own cell or one of the 8
//! cells touching it -- a 3x3 block -- no matter how many boids exist in total.

use macroquad::prelude::*;

use crate::boid::Boid;

/// Boid indices bucketed into square cells that tile the world bounds.
pub struct Grid {
    cell_size: f32,
    cols: usize,
    rows: usize,
    /// One bucket per cell, stored row-major: `cells[row * cols + col]`.
    /// Each bucket holds the indices of the boids that fall in that cell.
    cells: Vec<Vec<usize>>,
}

impl Grid {
    /// Builds a fresh grid and drops every boid into the cell it sits in.
    ///
    /// `cell_size` should be the flock's `neighbor_radius`: that guarantees a
    /// boid's neighbors are never more than one cell away.
    pub fn build(boids: &[Boid], bounds: Vec2, cell_size: f32) -> Self {
        let cols = (bounds.x / cell_size).ceil().max(1.0) as usize;
        let rows = (bounds.y / cell_size).ceil().max(1.0) as usize;

        let mut grid = Self {
            cell_size,
            cols,
            rows,
            cells: vec![Vec::new(); cols * rows],
        };

        for (i, boid) in boids.iter().enumerate() {
            let (cx, cy) = grid.cell_of(boid.pos);
            grid.cells[cy * grid.cols + cx].push(i);
        }

        grid
    }

    /// The (column, row) of the cell containing `pos`, clamped to the grid so
    /// boids exactly on the edge don't index out of bounds.
    fn cell_of(&self, pos: Vec2) -> (usize, usize) {
        let cx = (pos.x / self.cell_size).floor().clamp(0.0, (self.cols - 1) as f32) as usize;
        let cy = (pos.y / self.cell_size).floor().clamp(0.0, (self.rows - 1) as f32) as usize;
        (cx, cy)
    }

    /// Calls `f` with the index of every boid in the 3x3 block of cells around
    /// `pos` -- that is, every boid that could possibly be within range. The
    /// caller still does the exact distance check; this just shrinks the set
    /// from "all boids" to "the handful nearby".
    pub fn for_each_neighbor(&self, pos: Vec2, mut f: impl FnMut(usize)) {
        let (cx, cy) = self.cell_of(pos);

        let x0 = cx.saturating_sub(1);
        let y0 = cy.saturating_sub(1);
        let x1 = (cx + 1).min(self.cols - 1);
        let y1 = (cy + 1).min(self.rows - 1);

        for gy in y0..=y1 {
            for gx in x0..=x1 {
                for &j in &self.cells[gy * self.cols + gx] {
                    f(j);
                }
            }
        }
    }
}
