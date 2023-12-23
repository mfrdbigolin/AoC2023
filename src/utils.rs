// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

use std::ops::{Index, IndexMut, Not};

/// A Point in the Cartesian plane.
pub type Point = (usize, usize);

/// The cardinal Directions (South, North, East, and West).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    South,
    North,
    East,
    West,
}

impl Not for Direction {
    type Output = Self;

    /// Return the opposite Direction in a compass rose.
    fn not(self) -> Self::Output {
        match self {
            Direction::South => Direction::North,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

/// A two-dimensional Matrix.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Matrix<T: Default + Clone> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![Default::default(); rows * cols];

        Matrix { data, rows, cols }
    }

    /// For each Direction, return, in a vector, the neighbor point with its
    /// associated Direction relative to this point.
    pub fn get_neighbors(&self, (i, j): Point) -> Vec<(Direction, Point)> {
        /// All (Δi, Δj) to cover all possible neighbors (horizontal and vertical) of a tile.
        const DIRECTIONS: [(Direction, (i32, i32)); 4] = [
            (Direction::South, (1, 0)),
            (Direction::North, (-1, 0)),
            (Direction::East, (0, 1)),
            (Direction::West, (0, -1)),
        ];

        let m = self.rows as i32;
        let n = self.cols as i32;

        let mut neighbors = vec![];

        for (dir, (di, dj)) in DIRECTIONS {
            let ni = (i as i32) + di;
            let nj = (j as i32) + dj;

            if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
                continue;
            }

            neighbors.push((dir, (ni as usize, nj as usize)));
        }

        neighbors
    }
}

impl<T: Default + Clone> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: Point) -> &Self::Output {
        &self.data[idx.0 * self.cols + idx.1]
    }
}

impl<T: Default + Clone> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        &mut self.data[idx.0 * self.cols + idx.1]
    }
}
