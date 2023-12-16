// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Four, Scratchcards.

use std::collections::HashSet;

type Card = (HashSet<u32>, HashSet<u32>);

fn solve1(cards: &Vec<Card>) -> u32 {
    let mut total_points = 0;

    for (winning_set, owned_set) in cards {
        let num_winning_cards_owned = winning_set
            .iter()
            .filter(|card| owned_set.contains(card))
            .count();

        total_points += if num_winning_cards_owned != 0 {
            1 << (num_winning_cards_owned - 1)
        } else {
            0
        };
    }

    total_points
}

fn solve2(cards: &Vec<Card>) -> u32 {
    let n = cards.len();

    // dp[i] stores how many copies were generated by the (i + 1)th card after the end of the
    // process plus one (for the original card). At the start, only the original cards exist.
    let mut dp = vec![1; n];

    // Start from the last card (that can’t generate any copies) and build up to the first
    // card. This technique is called bottom-up dynamic programming (hence the name `dp`).
    for (i, (winning_set, owned_set)) in cards.iter().enumerate().rev() {
        let mut num_winning_owned = 0;

        for card in winning_set {
            if owned_set.contains(&card) {
                num_winning_owned += 1;
            }
        }

        // Use the results of the previous subproblems to compute the current result.
        dp[i] += dp[(i + 1)..(i + 1 + num_winning_owned)].iter().sum::<u32>();
    }

    dp.iter().sum()
}

fn parse_input(input_data: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input_data.lines() {
        let mut lists = line
            .split(": ")
            .nth(1)
            .expect("there should be two lists of numbers after the card name")
            .split(" | ");

        let winning_nums = lists
            .next()
            .expect("there should be a list of winning numbers")
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .expect("this list should contain only numbers")
            });

        let owned_nums = lists
            .next()
            .expect("there should be a list of owned numbers")
            .split_whitespace()
            .map(|n| {
                n.parse::<u32>()
                    .expect("this list should contain only numbers")
            });

        let winning_set: HashSet<u32> = winning_nums.collect();
        let owned_set: HashSet<u32> = owned_nums.collect();

        let card = (winning_set, owned_set);
        cards.push(card);
    }

    cards
}

pub fn day04(input_data: &str) {
    let cards = parse_input(input_data);

    let sol1 = solve1(&cards);
    let sol2 = solve2(&cards);

    println!("{sol1}");
    println!("{sol2}");
}