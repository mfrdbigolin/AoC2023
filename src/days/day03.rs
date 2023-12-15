// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Three, Gear Ratios.

/// Given a `line` string and an index `j` pointing to a digit in a number,
/// extract the complete number being pointed. If there is no digit indexed by
/// `j`, returns zero.
///
/// If the number is successfully extracted, this function will erase all the
/// digits (replacing them with dots) in the `line` string, **mutating** it.
fn extract_number(line: &mut Vec<char>, mut j: usize) -> u32 {
    // Move the index to the rightmost digit of the number.
    while line[j].is_digit(10) && j > 0 && line[j - 1].is_digit(10) {
        j -= 1;
    }

    let mut num = 0;

    while j < line.len() && line[j].is_digit(10) {
        let digit = line[j]
            .to_digit(10)
            .expect("this character should be a digit");

        // Erase the digit.
        line[j] = '.';

        num = num * 10 + digit;

        j += 1;
    }

    num
}

/// All (Δi, Δj) to cover all possible neighbors (horizontal, vertical, and diagonal) of a tile.
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Wrapper for a vector of vectors (2D matrix).
type Matrix<T> = Vec<Vec<T>>;

/// Given a `schematic`, perform an operation on the part numbers adjacent to a
/// symbol specified by `symbol_pred` and accumulate the results of these
/// operations. If `exactly_adjacent_to` is not `None`, only accumulate the
/// results with exactly this amount of part numbers.
fn solve(
    schematic: &Matrix<char>,
    symbol_pred: impl Fn(char) -> bool,
    part_number_op: impl Fn(u32, u32) -> u32,
    exactly_adjacent_to: Option<u32>,
) -> u32 {
    let m = schematic.len() as i32;
    let n = schematic[0].len() as i32;

    let mut schem = schematic.clone();

    let mut acc = 0;

    for (i, line) in schematic.iter().enumerate() {
        for (j, sym) in line.iter().enumerate() {
            if symbol_pred(*sym) && !sym.is_digit(10) {
                let mut partial: Option<u32> = None;
                let mut count = 0;

                for (di, dj) in DIRECTIONS {
                    let ni = (i as i32) + di;
                    let nj = (j as i32) + dj;

                    if !(0..m).contains(&ni) || !(0..n).contains(&nj) {
                        continue;
                    }

                    let part_number = extract_number(&mut schem[ni as usize], nj as usize);

                    if part_number > 0 {
                        partial = if partial.is_none() {
                            Some(part_number)
                        } else {
                            Some(part_number_op(partial.unwrap(), part_number))
                        };
                        count += 1;
                    }
                }

                match exactly_adjacent_to {
                    Some(num) => {
                        if count != num {
                            partial = None
                        }
                    }
                    None => (),
                };

                acc += match partial {
                    Some(num) => num,
                    None => 0,
                };
            }
        }
    }

    acc
}

fn parse_input(input_data: &str) -> Matrix<char> {
    let mut schematic: Matrix<char> = Vec::new();

    for line in input_data.lines() {
        schematic.push(line.chars().collect());
    }

    schematic
}

pub fn day03(input_data: &str) {
    let schematic = parse_input(input_data);

    let sum_op = |acc, n| acc + n;
    let sol1 = solve(&schematic, |ch| ch != '.', sum_op, None);

    let mult_op = |acc, n| acc * n;
    let sol2 = solve(&schematic, |ch| ch == '*', mult_op, Some(2));

    println!("{sol1}");
    println!("{sol2}");
}
