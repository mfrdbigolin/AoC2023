// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Ten, Pipe Maze.

use crate::utils::{Direction, Matrix, Point};

use std::ops::{Index, IndexMut};
use std::str::FromStr;

/// Pipe with cardinal directions representing connections. The default value
/// of the Pipe type is a Pipe with no connections (i.e., the ground tile).
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
struct Pipe {
    south: bool,
    north: bool,
    east: bool,
    west: bool,
}

impl Pipe {
    pub fn is_connected(&self, other: Pipe, dir: Direction) -> bool {
        self[dir] && other[!dir]
    }
}

impl Index<Direction> for Pipe {
    type Output = bool;

    /// Verify whether a connection exists in the pipe.
    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Direction::South => &self.south,
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::West => &self.west,
        }
    }
}

impl IndexMut<Direction> for Pipe {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        match dir {
            Direction::South => &mut self.south,
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::West => &mut self.west,
        }
    }
}

impl FromStr for Pipe {
    type Err = ();

    fn from_str(tile_str: &str) -> Result<Pipe, Self::Err> {
        match tile_str.to_uppercase().as_str() {
            "." => Ok(Pipe {
                ..Default::default()
            }),
            "|" => Ok(Pipe {
                south: true,
                north: true,
                ..Default::default()
            }),
            "-" => Ok(Pipe {
                east: true,
                west: true,
                ..Default::default()
            }),
            "F" => Ok(Pipe {
                south: true,
                east: true,
                ..Default::default()
            }),
            "7" => Ok(Pipe {
                south: true,
                west: true,
                ..Default::default()
            }),
            "L" => Ok(Pipe {
                north: true,
                east: true,
                ..Default::default()
            }),
            "J" => Ok(Pipe {
                north: true,
                west: true,
                ..Default::default()
            }),
            _ => Err(()),
        }
    }
}

fn get_main_loop_points(starting_point: Point, pipe_matrix: &Matrix<Pipe>) -> Vec<Point> {
    // Start in a neighboring tile of the starting point.
    let (mut cur_dir, mut cur) = pipe_matrix.get_neighbors(starting_point)[0];

    let mut visited = vec![cur];

    while cur != starting_point {
        // Gets the next point in the loop, cur_dir is used to prevent going
        // back to a previously visited point.
        (cur_dir, cur) = pipe_matrix
            .get_neighbors(cur)
            .iter()
            .copied()
            .filter(|(dir, point)| {
                *dir != !cur_dir && pipe_matrix[cur].is_connected(pipe_matrix[*point], *dir)
            })
            .next()
            .expect("the main loop should be continuous");

        visited.push(cur);
    }

    visited
}

fn solve1(starting_point: Point, pipe_matrix: &Matrix<Pipe>) -> usize {
    let main_loop = get_main_loop_points(starting_point, pipe_matrix);

    // Considering that there will always be an even number of points that form
    // the main loop, the farthest distance is halfway through the loop.
    main_loop.len() / 2
}

fn solve2(starting_point: Point, pipe_matrix: &Matrix<Pipe>) -> usize {
    let mut main_loop = get_main_loop_points(starting_point, pipe_matrix);

    // The shoelace formula requires that the initial point also appears at the end.
    main_loop.push(main_loop[0]);

    let mut loop_area: i32 = 0;

    for point_pair in main_loop.windows(2) {
        let [point1, point2] = [point_pair[0], point_pair[1]];

        // The total area is incremented by the determinant of the points in the pair.
        loop_area += (point1.0 * point2.1) as i32 - (point1.1 * point2.0) as i32;
    }

    loop_area /= 2;

    // Use Pick’s theorem to find the number of inclosed points.
    (loop_area.abs() as usize) - main_loop.len() / 2 + 1
}

fn parse_input(input_data: &str) -> (Point, Matrix<Pipe>) {
    let rows = 1 + input_data.chars().filter(|ch| *ch == '\n').count();
    let cols = input_data.chars().take_while(|ch| *ch != '\n').count();

    let mut starting_point: Option<Point> = None;
    let mut pipe_matrix = Matrix::new(rows, cols);

    for (i, line) in input_data.lines().enumerate() {
        for (j, tile_str) in line.chars().enumerate() {
            // The starting tile pipe will be set later.
            if tile_str != 'S' {
                pipe_matrix[(i, j)] = Pipe::from_str(tile_str.to_string().as_str())
                    .expect("there should only be valid tiles in the sketch");
            }

            if tile_str == 'S' {
                if starting_point.is_some() {
                    panic!("there should be only one starting tile in the sketch");
                }

                starting_point = Some((i, j));
            }
        }
    }

    let starting_point = starting_point.expect("there should be one starting tile in the sketch");

    // Find the right connections for the starting point.
    for (dir, neighbor) in pipe_matrix.get_neighbors(starting_point) {
        if pipe_matrix[neighbor][!dir] {
            pipe_matrix[starting_point][dir] = true;
        }
    }

    (starting_point, pipe_matrix)
}

pub fn day10(input_data: &str) {
    let (starting_point, pipe_matrix) = parse_input(input_data);

    let sol1 = solve1(starting_point, &pipe_matrix);
    let sol2 = solve2(starting_point, &pipe_matrix);

    println!("{sol1}");
    println!("{sol2}");
}
