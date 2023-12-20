// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Eight, Haunted Wasteland.

use num::integer;
use regex::Regex;
use std::str::FromStr;

// For ease of development, we will only consider uppercase letters for the alphabet.
const NODE_ALPHABET_SIZE: usize = 'Z' as usize - 'A' as usize + 1;
const NODE_IDENTIFIER_SIZE: usize = 3;
/// Number of all the possible nodes in the network.
const NETWORK_SIZE: usize = NODE_ALPHABET_SIZE.pow(NODE_IDENTIFIER_SIZE as u32);

enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(instr_str: &str) -> Result<Instruction, Self::Err> {
        match instr_str.to_uppercase().as_str() {
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            _ => Err(()),
        }
    }
}

/// Given a node string identifier, return the node’s index in the network.
fn get_node_index(node: &str) -> usize {
    node.chars()
        .enumerate()
        .map(|(i, ch)| {
            (ch.to_ascii_uppercase() as usize - 'A' as usize)
                * NODE_ALPHABET_SIZE.pow((node.len() - i - 1) as u32)
        })
        .sum()
}

type Network = ([Option<usize>; NETWORK_SIZE], [Option<usize>; NETWORK_SIZE]);

fn solve1(
    (left, right): &Network,
    instructions: &Vec<Instruction>,
    start_node: &str,
    end_node: &str,
) -> u64 {
    let mut count = 0;

    let mut cur = get_node_index(start_node);
    let end = get_node_index(end_node);

    while cur != end {
        cur = match instructions[count % instructions.len()] {
            Instruction::Left => left[cur],
            Instruction::Right => right[cur],
        }
        .expect("node path should not lead to nonexistent node");

        count += 1;
    }

    count as u64
}

// This solution is not general; it makes several assumptions that are true in the given input,
// such as: predicting that there will be a cycle and that each cycle has the same period, but
// that are not universally applicable.
//
// Considering I’m kinda late this year and that the general solution is quite more involved,
// for now I will leave this as it is.
fn solve2(
    (left, right): &Network,
    instructions: &Vec<Instruction>,
    start_nodes_ending_with: char,
    end_nodes_ending_with: char,
) -> u64 {
    let start_nodes_idx = start_nodes_ending_with.to_ascii_uppercase() as usize - 'A' as usize;
    let end_nodes_idx = end_nodes_ending_with.to_ascii_uppercase() as usize - 'A' as usize;

    let mut starting_nodes: Vec<usize> = vec![];

    for (i, node) in left
        .iter()
        .copied()
        .skip(start_nodes_idx)
        .step_by(NODE_ALPHABET_SIZE)
        .enumerate()
    {
        if !node.is_none() {
            starting_nodes.push(NODE_ALPHABET_SIZE * i + start_nodes_idx);
        }
    }

    let mut counts: Vec<u64> = vec![];

    for node in starting_nodes {
        let mut count = 0;

        let mut cur = node;

        while cur % NODE_ALPHABET_SIZE != end_nodes_idx {
            cur = match instructions[count % instructions.len()] {
                Instruction::Left => left[cur],
                Instruction::Right => right[cur],
            }
            .expect("node path should not lead to nonexistent node");

            count += 1;
        }

        counts.push(count as u64);
    }

    counts
        .into_iter()
        .reduce(|multiple, cnt| integer::lcm(multiple, cnt))
        .expect("there should be at least one starting node")
}

fn parse_input(input_data: &str) -> (Vec<Instruction>, Network) {
    let mut left_vec: Vec<Option<usize>> = vec![None; NETWORK_SIZE];
    let mut right_vec: Vec<Option<usize>> = vec![None; NETWORK_SIZE];

    if input_data.find(|ch: char| ch.is_numeric()).is_some() {
        panic!("the input should not have numeric characters")
    }

    let mut lines = input_data.lines();

    let instructions: Vec<_> = lines
        .next()
        .expect("there should be a line with instructions")
        .chars()
        .map(|ins| {
            Instruction::from_str(ins.to_string().as_str())
                .expect("each instruction should be left or right only")
        })
        .collect();

    lines.next();

    let node_regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)")
        .expect("hardcoded regex should be valid");

    let nodes_str: String = lines.collect::<Vec<_>>().join("\n");

    for (_, [node, left_node, right_node]) in node_regex
        .captures_iter(nodes_str.as_str())
        .map(|c| c.extract())
    {
        left_vec[get_node_index(node)] = Some(get_node_index(left_node));
        right_vec[get_node_index(node)] = Some(get_node_index(right_node));
    }

    let left = left_vec
        .try_into()
        .expect("vector length should be equal to NETWORK_SIZE");

    let right = right_vec
        .try_into()
        .expect("vector length should be equal to NETWORK_SIZE");

    (instructions, (left, right))
}

pub fn day08(input_data: &str) {
    let (instructions, network) = parse_input(input_data);

    let sol1 = solve1(&network, &instructions, "AAA", "ZZZ");
    let sol2 = solve2(&network, &instructions, 'A', 'Z');

    println!("{sol1}");
    println!("{sol2}");
}
