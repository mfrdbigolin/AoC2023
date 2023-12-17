// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Six, Wait For It.

/// Multiply the number of ways to win each race described in `document`.
/// Essentially, for each race, solves by finding the roots of
/// -x² + xt - d > 0, in which (t, d) ∈ `document`, with the quadratic formula.
fn solve(document: (Vec<u64>, Vec<u64>)) -> u64 {
    let races = std::iter::zip(document.0, document.1);

    let num_ways: Vec<u64> = races
        .map(|(t, d)| {
            let a = (t as f64) / 2.0;
            let b = ((t * t - 4 * d) as f64).sqrt() / 2.0;

            let lower_bound = (a - b + 1.0).floor() as u64;
            let upper_bound = (a + b - 1.0).ceil() as u64;

            upper_bound - lower_bound + 1
        })
        .collect();

    num_ways.iter().product()
}

fn parse_input1(input_data: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input_data.lines().map(|line| {
        line.split(":")
            .nth(1)
            .expect("a list of space-separated values should follow")
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().expect("this should be an integer"))
            .collect()
    });

    let times = lines
        .next()
        .expect("there should be a line with the allowed times of each race");

    let distances = lines
        .next()
        .expect("there should be a line with the best distances of each race");

    (times, distances)
}

fn parse_input2(input_data: &str) -> (Vec<u64>, Vec<u64>) {
    let mut lines = input_data.lines().map(|line| {
        line.split(":")
            .nth(1)
            .expect("a list of space-separated values should follow")
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .expect("this should be an integer")
    });

    let time = lines
        .next()
        .expect("there should be a line with the allowed time of the race");

    let distance = lines
        .next()
        .expect("there should be a line with the best distance of the race");

    (vec![time], vec![distance])
}

pub fn day06(input_data: &str) {
    let document1 = parse_input1(input_data);

    let sol1 = solve(document1);

    let document2 = parse_input2(input_data);

    let sol2 = solve(document2);

    println!("{sol1}");
    println!("{sol2}");
}
