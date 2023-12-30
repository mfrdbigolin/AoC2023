// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

use std::{
    ops::{Index, IndexMut, Not},
    str::FromStr,
};

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
pub struct Matrix<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
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

    pub fn get_row(&self, i: usize) -> Vec<T> {
        let mut row = vec![];

        for j in 0..self.cols {
            row.push(self[(i, j)].clone());
        }

        row
    }

    pub fn get_rows(&self) -> Vec<Vec<T>> {
        let mut rows = vec![];

        for i in 0..self.rows {
            rows.push(self.get_row(i));
        }

        rows
    }

    pub fn get_col(&self, j: usize) -> Vec<T> {
        let mut col = vec![];

        for i in 0..self.rows {
            col.push(self[(i, j)].clone());
        }

        col
    }

    pub fn get_cols(&self) -> Vec<Vec<T>> {
        let mut cols = vec![];

        for j in 0..self.cols {
            cols.push(self.get_col(j));
        }

        cols
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: Point) -> &Self::Output {
        &self.data[idx.0 * self.cols + idx.1]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, idx: Point) -> &mut Self::Output {
        &mut self.data[idx.0 * self.cols + idx.1]
    }
}

impl<T: Default + Clone + FromStr> FromStr for Matrix<T> {
    type Err = ();

    fn from_str(matrix_str: &str) -> Result<Matrix<T>, Self::Err> {
        let rows = 1 + matrix_str.chars().filter(|ch| *ch == '\n').count();
        let cols = matrix_str.chars().take_while(|ch| *ch != '\n').count();

        let mut matrix = Matrix::<T>::new(rows, cols);

        for (i, line) in matrix_str.lines().enumerate() {
            for (j, elem) in line.chars().enumerate() {
                matrix[(i, j)] = match T::from_str(elem.to_string().as_str()) {
                    Ok(val) => val,
                    Err(_) => return Err(()),
                };
            }
        }

        Ok(matrix)
    }
}
