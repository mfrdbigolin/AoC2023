// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Twelve, Hot Springs.

use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl FromStr for Spring {
    type Err = ();

    fn from_str(spring_str: &str) -> Result<Spring, Self::Err> {
        match spring_str {
            "." => Ok(Spring::Operational),
            "#" => Ok(Spring::Damaged),
            "?" => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}

type SpringRecord = (Vec<Spring>, Vec<u64>);

// My previous recursive solution with memoization. It is roughly five times
// slower than the dynamic programming one.
#[allow(dead_code)]
fn solve_rec(
    springs: &Vec<Spring>,
    groups: &Vec<u64>,
    memo: &mut Vec<Vec<Vec<Option<u64>>>>,
    i: usize,
    j: usize,
    k: usize,
) -> u64 {
    if let Some(num) = memo[i][j][k] {
        return num;
    }

    if j == groups.len() {
        // For this combination to be valid, there should be no damaged tiles ahead.
        return if !springs[i..].contains(&Spring::Damaged) {
            1
        } else {
            0
        };
    }

    if i == springs.len() {
        return if j == groups.len() - 1 && (k as u64) == groups[j] {
            1
        } else {
            0
        };
    }

    let mut ans = 0;

    let damaged = springs[i] == Spring::Damaged || springs[i] == Spring::Unknown;
    let operational = springs[i] == Spring::Operational || springs[i] == Spring::Unknown;

    if (k as u64) != groups[j] {
        if operational && k == 0 {
            ans = solve_rec(springs, groups, memo, i + 1, j, k);
        }

        if damaged {
            ans += solve_rec(springs, groups, memo, i + 1, j, k + 1);
        }
    } else if operational {
        // We have completed a group of damaged tiles.
        ans = solve_rec(springs, groups, memo, i + 1, j + 1, 0);
    }

    memo[i][j][k] = Some(ans);
    ans
}

// Solution inspired from u/Nithramir from Reddit. However, it only keeps the
// last two rows of the table in each iteration to save memory.
fn solve_dp(springs: &Vec<Spring>, groups: &Vec<u64>) -> u64 {
    // Pad springs.
    let springs = vec![
        vec![Spring::Operational],
        springs.clone(),
        vec![Spring::Operational],
    ]
    .concat();

    // Pad groups.
    let groups: Vec<bool> = groups
        .iter()
        .copied()
        .map(|group| vec![true; group as usize])
        .flat_map(|group| vec![group, vec![false]].concat())
        .collect();

    let groups = vec![vec![false], groups].concat();

    let mut dp = vec![0 as u64; groups.len() + 1];
    dp[groups.len()] = 1;

    for &spring in springs.iter().rev() {
        let mut cur_dp = vec![0; groups.len() + 1];

        for (i, &group) in groups.iter().enumerate().rev() {
            let damaged = spring == Spring::Damaged || spring == Spring::Unknown;
            let operational = spring == Spring::Operational || spring == Spring::Unknown;

            if damaged && group {
                cur_dp[i] = dp[i + 1];
            } else if operational && !group {
                cur_dp[i] = dp[i + 1] + dp[i];
            }
        }

        dp = cur_dp;
    }

    dp[0]
}

fn solve(records: &Vec<SpringRecord>) -> u64 {
    let mut acc = 0;

    for (springs, groups) in records {
        /* // Setup for the recursive solution:
         * let biggest_group = groups
         *     .iter()
         *     .copied()
         *     .max()
         *     .expect("the spring group should not be empty") as usize;
         *
         * let mut memo: Vec<Vec<Vec<Option<u64>>>> =
         *     vec![vec![vec![None; biggest_group + 1]; groups.len() + 1]; springs.len() + 1];
         *
         * acc += solve_rec(&springs, &groups, &mut memo, 0, 0, 0);
         */

        acc += solve_dp(&springs, &groups);
    }

    acc
}

fn parse_input(input_data: &str, unfolding_factor: u64) -> Vec<SpringRecord> {
    let mut records = vec![];

    for line in input_data.lines() {
        let [springs_str, groups_str]: [&str; 2] = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .expect("there should be a list of springs and a list of groups in each line");

        let unfolded_springs = vec![springs_str.to_string(); unfolding_factor as usize].join("?");

        let springs = unfolded_springs
            .chars()
            .map(|spring| {
                Spring::from_str(spring.to_string().as_str())
                    .expect("the spring condition should be valid")
            })
            .collect();

        let unfolded_groups = vec![groups_str.to_string(); unfolding_factor as usize].join(",");

        let groups: Vec<u64> = unfolded_groups
            .split(',')
            .map(|group| {
                group
                    .parse()
                    .expect("the group size should be a positive integer")
            })
            .collect();

        records.push((springs, groups));
    }

    records
}

pub fn day12(input_data: &str) {
    let spring_records1 = parse_input(input_data, 1);

    let sol1 = solve(&spring_records1);

    let spring_records2 = parse_input(input_data, 5);

    let sol2 = solve(&spring_records2);

    println!("{sol1}");
    println!("{sol2}");
}
