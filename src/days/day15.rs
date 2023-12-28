// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Fifteen, Lens Library.

use std::collections::HashMap;

fn compute_hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, ch| 17 * (acc + ch as u32) % 256)
}

fn solve1(steps: &Vec<&str>) -> u32 {
    steps.iter().map(|step| compute_hash(step)).sum()
}

fn solve2(steps: &Vec<(&str, Option<u32>)>) -> u32 {
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for &(label, focal_length) in steps {
        let box_num = compute_hash(label);

        if let Some(focal_length) = focal_length {
            let lenses = boxes.entry(box_num).or_default();

            if let Some(lens_pos) = lenses.iter().position(|&(lab, _)| lab == label) {
                lenses[lens_pos] = (label, focal_length);
            } else {
                lenses.push((label, focal_length));
            }
        } else {
            boxes
                .entry(box_num)
                .and_modify(|lenses| lenses.retain(|&(lab, _)| lab != label));
        }
    }

    boxes
        .into_iter()
        .map(|(box_num, lenses)| {
            (box_num + 1)
                * lenses
                    .into_iter()
                    .enumerate()
                    .map(|(slot, (_, focal_length))| (slot as u32 + 1) * focal_length)
                    .sum::<u32>()
        })
        .sum()
}

fn parse_steps1(input_data: &str) -> Vec<&str> {
    input_data.split(',').collect()
}

fn parse_steps2(input_data: &str) -> Vec<(&str, Option<u32>)> {
    input_data
        .split(',')
        .map(|step| {
            let mut split_step = step.split(|ch| ch == '-' || ch == '=');

            let label = split_step
                .next()
                .expect("there should be a label with every step");

            let focal_length = match split_step
                .next()
                .expect("there should be a focal length with every step")
            {
                "" => None,
                len_str => Some(
                    len_str
                        .parse()
                        .expect("the focal length should be a positive integer"),
                ),
            };

            (label, focal_length)
        })
        .collect()
}

pub fn day15(input_data: &str) {
    let steps1 = parse_steps1(input_data);

    let sol1 = solve1(&steps1);

    let steps2 = parse_steps2(input_data);

    let sol2 = solve2(&steps2);

    println!("{sol1}");
    println!("{sol2}");
}
