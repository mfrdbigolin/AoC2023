// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Sixteen, The Floor Will Be Lava.

use std::{cmp::max, collections::HashSet, str::FromStr};

use crate::utils::{Direction, Matrix, Point};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]

struct Mirror {
    reflect: fn(Direction) -> Vec<Direction>,
}

impl Default for Mirror {
    fn default() -> Self {
        Mirror {
            reflect: |dir: Direction| vec![!dir],
        }
    }
}

impl FromStr for Mirror {
    type Err = ();

    fn from_str(tile_str: &str) -> Result<Mirror, Self::Err> {
        match tile_str.to_uppercase().as_str() {
            "." => Ok(Mirror {
                reflect: |dir: Direction| vec![dir],
            }),
            "|" => Ok(Mirror {
                reflect: |dir: Direction| match dir {
                    Direction::South | Direction::North => vec![dir],
                    _ => vec![Direction::South, Direction::North],
                },
            }),
            "-" => Ok(Mirror {
                reflect: |dir: Direction| match dir {
                    Direction::East | Direction::West => vec![dir],
                    _ => vec![Direction::East, Direction::West],
                },
            }),
            "/" => Ok(Mirror {
                reflect: |dir: Direction| match dir {
                    Direction::South => vec![Direction::West],
                    Direction::North => vec![Direction::East],
                    Direction::East => vec![Direction::North],
                    Direction::West => vec![Direction::South],
                },
            }),
            "\\" => Ok(Mirror {
                reflect: |dir: Direction| match dir {
                    Direction::South => vec![Direction::East],
                    Direction::North => vec![Direction::West],
                    Direction::East => vec![Direction::South],
                    Direction::West => vec![Direction::North],
                },
            }),
            _ => Err(()),
        }
    }
}

fn get_energized_positions(
    mirror_matrix: &Matrix<Mirror>,
    starting_position: (Direction, Point),
) -> HashSet<(Direction, Point)> {
    let mut visited: HashSet<(Direction, Point)> = HashSet::from([starting_position]);
    let mut stack: Vec<(Direction, Point)> = vec![starting_position];

    while !stack.is_empty() {
        let (dir, point) = stack.pop().expect("the stack should not be empty");

        let next_dirs = (mirror_matrix[point].reflect)(dir);

        for pos in mirror_matrix
            .get_neighbors(point)
            .into_iter()
            .filter(|(cur_dir, _)| next_dirs.contains(cur_dir))
        {
            if !visited.contains(&pos) {
                stack.push(pos);
            }

            visited.insert(pos);
        }
    }

    visited
}

fn solve1(mirror_matrix: &Matrix<Mirror>, starting_position: (Direction, Point)) -> usize {
    let energized_positions = get_energized_positions(mirror_matrix, starting_position);

    let energized_points: HashSet<Point> = energized_positions
        .into_iter()
        .map(|(_, point)| point)
        .collect();

    energized_points.len()
}

// The only optimization used is the one that exclude edge points that were previously visited
// (due to http://clb.confined.space/aoc2023). But, still quite slow (1.3 secs);
// maybe someday I will try to optimize more.
fn solve2(mirror_matrix: &Matrix<Mirror>) -> usize {
    let mut max_energy = 0;

    let edge_points = {
        let south: Vec<Point> = (0..(mirror_matrix.cols - 1))
            .map(|col| (mirror_matrix.rows - 1, col))
            .collect();
        let north: Vec<Point> = (0..(mirror_matrix.cols - 1)).map(|col| (0, col)).collect();
        let east: Vec<Point> = (1..(mirror_matrix.rows - 2))
            .map(|row| (row, mirror_matrix.cols - 1))
            .collect();
        let west: Vec<Point> = (1..(mirror_matrix.rows - 2)).map(|row| (row, 0)).collect();

        vec![south, north, east, west].concat()
    };

    let mut visited_edges = HashSet::new();

    for point in edge_points {
        for dir in [
            Direction::South,
            Direction::North,
            Direction::East,
            Direction::West,
        ] {
            if visited_edges.contains(&(dir, point)) {
                continue;
            }

            let visited_positions = get_energized_positions(mirror_matrix, (dir, point));

            let energized_points: HashSet<Point> =
                visited_positions.iter().map(|&(_, point)| point).collect();

            max_energy = max(max_energy, energized_points.len());

            visited_edges.extend(visited_positions.into_iter().filter(|&(_, (i, j))| {
                i == 0 || i == mirror_matrix.rows - 1 || j == 0 || j == mirror_matrix.cols - 1
            }));
        }
    }

    max_energy
}

pub fn day16(input_data: &str) {
    let mirror_matrix =
        Matrix::<Mirror>::from_str(input_data).expect("the mirror matrix should be valid");

    let sol1 = solve1(&mirror_matrix, (Direction::East, (0, 0)));
    let sol2 = solve2(&mirror_matrix);

    println!("{sol1}");
    println!("{sol2}");
}
