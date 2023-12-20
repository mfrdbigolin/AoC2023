// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Nine, Mirage Maintenance.

fn solve(histories: Vec<Vec<i32>>) -> i32 {
    let mut history_sum = 0;

    for history in histories {
        let mut next_value = 0;

        let mut diffs = history;

        // While the adjacent differences are non-constant.
        while diffs.windows(2).any(|val| val[0] != val[1]) {
            next_value += diffs[diffs.len() - 1];
            diffs = diffs.windows(2).map(|val| val[1] - val[0]).collect();
        }
        next_value += diffs[diffs.len() - 1];

        history_sum += next_value;
    }

    history_sum
}

fn parse_input(input_data: &str) -> Vec<Vec<i32>> {
    let mut histories: Vec<Vec<i32>> = vec![];

    for line in input_data.lines() {
        let history: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("each history value should be an integer"))
            .collect();

        histories.push(history);
    }

    histories
}

pub fn day09(input_data: &str) {
    let histories = parse_input(input_data);

    // Each history’s value in reverse, used for part two.
    let histories_rev = histories
        .iter()
        .cloned()
        .map(|mut lol| {
            lol.reverse();
            lol
        })
        .collect();

    let sol1 = solve(histories);
    let sol2 = solve(histories_rev);

    println!("{sol1}");
    println!("{sol2}");
}
