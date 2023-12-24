// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Eleven, Cosmic Expansion.

use std::str::FromStr;

use crate::utils::{Matrix, Point};

fn solve(points: &Vec<Point>) -> u64 {
    let mut dist_sum = 0;

    for (i, point1) in points.iter().copied().enumerate() {
        for point2 in points[i + 1..].iter().copied() {
            let dx = (point2.0 as i64 - point1.0 as i64).abs() as u64;
            let dy = (point2.1 as i64 - point1.1 as i64).abs() as u64;

            // Manhattan distance.
            dist_sum += dx + dy;
        }
    }

    dist_sum
}

fn get_empty_lines(intergalactic_image: &Matrix<char>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows: Vec<_> = intergalactic_image
        .get_rows()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(i, _)| i)
        .rev()
        .collect();

    let empty_cols: Vec<_> = intergalactic_image
        .get_cols()
        .into_iter()
        .enumerate()
        .filter(|(_, col)| !col.contains(&'#'))
        .map(|(j, _)| j)
        .rev()
        .collect();

    (empty_rows, empty_cols)
}

fn expand_image(
    intergalactic_image: &Matrix<char>,
    (empty_rows, empty_cols): &(Vec<usize>, Vec<usize>),
    expansion_factor: u32,
) -> Vec<Point> {
    let mut points = vec![];

    let mut empty_rows = empty_rows.clone();
    let mut cur_missing_row = empty_rows.pop();

    let mut row_offset = 0;

    for (i, line) in intergalactic_image.get_rows().into_iter().enumerate() {
        if Some(i) == cur_missing_row {
            row_offset += (expansion_factor - 1) as usize;
            cur_missing_row = empty_rows.pop();
            continue;
        }

        let mut empty_cols = empty_cols.clone();
        let mut cur_missing_col = empty_cols.pop();

        let mut col_offset = 0;

        for (j, el) in line.into_iter().enumerate() {
            if Some(j) == cur_missing_col {
                col_offset += (expansion_factor - 1) as usize;
                cur_missing_col = empty_cols.pop();
                continue;
            }

            if el != '#' {
                continue;
            }

            points.push((i + row_offset, j + col_offset));
        }
    }

    points
}

pub fn day11(input_data: &str) {
    let intergalactic_image = Matrix::<char>::from_str(input_data).unwrap();

    let empty_lines = get_empty_lines(&intergalactic_image);

    let galaxy_points1 = expand_image(&intergalactic_image, &empty_lines, 2);

    let sol1 = solve(&galaxy_points1);

    let galaxy_points2 = expand_image(&intergalactic_image, &empty_lines, 1000000);

    let sol2 = solve(&galaxy_points2);

    println!("{sol1}");
    println!("{sol2}");
}
