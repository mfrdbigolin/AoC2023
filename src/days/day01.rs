// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day One, Trebuchet?!

/// Return the sum of the digits selected by a selector function in each line of a document.
fn digit_sum(document: std::str::Lines<'_>, digit_selector: impl Fn(&str) -> u32) -> u32 {
    document.fold(0, |acc, line| acc + digit_selector(line))
}

/// Convert a digit stored inside a string to an integer.
fn digit_str_to_num(digit_str: &str) -> u32 {
    (digit_str.as_bytes()[0] as char)
        .to_digit(10)
        .expect("the character matched should be a digit")
}

fn get_first_digit(s: &str) -> u32 {
    let mut digit_matches = s.matches(|ch: char| ch.is_digit(10));

    let digit_str = digit_matches
        .next()
        .expect("there should be a digit in this line");

    digit_str_to_num(digit_str)
}

fn get_last_digit(s: &str) -> u32 {
    let mut digit_matches = s.matches(|ch: char| ch.is_digit(10));

    let digit_str = digit_matches
        .next_back()
        .expect("there should be a digit in this line");

    digit_str_to_num(digit_str)
}

fn solve1(document: &str) -> u32 {
    let forward_sum = digit_sum(document.lines(), get_first_digit);
    let backward_sum = digit_sum(document.lines(), get_last_digit);

    10 * forward_sum + backward_sum
}

fn solve2(document: &str) -> u32 {
    // Ambiguous cases that have to be dealt with separately.
    let overlapping_patterns = vec![
        "oneight",
        "twone",
        "threeight",
        "fiveight",
        "sevenine",
        "eightwo",
        "eighthree",
        "nineight",
    ];

    let patterns = vec![
        overlapping_patterns,
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ],
    ]
    .concat();

    let digit_replacements = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    // Replacements used when searching for the first digit.
    let forward_replacements = vec![
        vec!["1", "2", "3", "5", "7", "8", "8", "9"],
        digit_replacements.clone(),
    ]
    .concat();

    // Replacements used when searching for the last digit.
    let backward_replacements = vec![
        vec!["8", "1", "8", "8", "9", "2", "3", "8"],
        digit_replacements.clone(),
    ]
    .concat();

    let ac_automaton = aho_corasick::AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::LeftmostLongest)
        .build(patterns)
        .unwrap();

    let forward_doc = ac_automaton.replace_all(document, &forward_replacements);
    let backward_doc = ac_automaton.replace_all(document, &backward_replacements);

    let forward_sum = digit_sum(forward_doc.lines(), get_first_digit);
    let backward_sum = digit_sum(backward_doc.lines(), get_last_digit);

    10 * forward_sum + backward_sum
}

pub fn day01(input_data: &str) {
    let sol1 = solve1(input_data);
    let sol2 = solve2(input_data);

    println!("{sol1}");
    println!("{sol2}");
}
