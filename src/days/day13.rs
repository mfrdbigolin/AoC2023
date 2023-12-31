// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Thirteen, Point of Incidence.

use std::{cmp::min, iter::zip, str::FromStr};

use crate::utils::Matrix;

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, Hash)]
enum Terrain {
    #[default]
    Ash,
    Rock,
}

impl FromStr for Terrain {
    type Err = ();

    fn from_str(terrain_str: &str) -> Result<Terrain, Self::Err> {
        match terrain_str {
            "." => Ok(Terrain::Ash),
            "#" => Ok(Terrain::Rock),
            _ => Err(()),
        }
    }
}

fn find_reflection_line(lines: &Vec<Vec<Terrain>>, req_num_smudges: usize) -> Option<usize> {
    for i in 1..lines.len() {
        let num_lines = min(i, lines.len() - i);

        let num_smudges: usize = (0..num_lines)
            .filter(|&j| lines[i - j - 1] != lines[i + j])
            .map(|j| {
                zip(&lines[i - j - 1], &lines[i + j])
                    .filter(|&(a, b)| a != b)
                    .count()
            })
            .sum();

        if num_smudges == req_num_smudges {
            return Some(i);
        }
    }

    None
}

fn solve(landscape_matrices: &Vec<Matrix<Terrain>>, req_num_smudges: usize) -> usize {
    landscape_matrices.iter().fold(0, |acc, landscape_matrix| {
        let rows = landscape_matrix.get_rows();
        let cols = landscape_matrix.get_cols();

        let reflection_row = find_reflection_line(&rows, req_num_smudges);
        let reflection_col = find_reflection_line(&cols, req_num_smudges);

        acc + 100 * reflection_row.unwrap_or(0) + reflection_col.unwrap_or(0)
    })
}

fn parse_input(input_data: &str) -> Vec<Matrix<Terrain>> {
    input_data
        .split("\n\n")
        .map(|matrix_str| {
            Matrix::from_str(matrix_str)
                .expect("a landscape matrix should only contain ash ('.') and rocks ('#')")
        })
        .collect()
}

pub fn day13(input_data: &str) {
    let landscape_matrices = parse_input(input_data);

    let sol1 = solve(&landscape_matrices, 0);
    let sol2 = solve(&landscape_matrices, 1);

    println!("{sol1}");
    println!("{sol2}");
}
