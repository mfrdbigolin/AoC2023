// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Seven, Camel Cards.

use core::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Card {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Jack,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(card_str: &str) -> Result<Card, Self::Err> {
        match card_str {
            "★" => Ok(Card::Joker),
            "2" => Ok(Card::N2),
            "3" => Ok(Card::N3),
            "4" => Ok(Card::N4),
            "5" => Ok(Card::N5),
            "6" => Ok(Card::N6),
            "7" => Ok(Card::N7),
            "8" => Ok(Card::N8),
            "9" => Ok(Card::N9),
            "T" => Ok(Card::T),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

fn calculate_hand_type(cards: &[Card; 5]) -> HandType {
    let mut freq_map = cards.iter().copied().fold(HashMap::new(), |mut map, card| {
        map.entry(card).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    let num_jokers = freq_map.remove(&Card::Joker).unwrap_or(0);

    // All the cards were Jokers.
    if freq_map.is_empty() {
        return HandType::FiveKind;
    }

    let most_freq_card = freq_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(card, _)| card)
        .expect("the iterator should not be empty");

    // The strategy is to treat the Jokers as the most frequent non-Joker card.
    freq_map
        .entry(*most_freq_card)
        .and_modify(|freq| *freq += num_jokers)
        .or_insert(num_jokers);

    let mut frequencies: Vec<i32> = freq_map.values().copied().collect();

    frequencies.sort();

    match frequencies[..] {
        [5] => HandType::FiveKind,
        [1, 4] => HandType::FourKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::High,
        _ => panic!("this frequency is impossible"),
    }
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let hand_type = calculate_hand_type(&cards);

        Hand { cards, hand_type }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(cards_str: &str) -> Result<Hand, Self::Err> {
        let mut cards_vec: Vec<Card> = vec![];

        for card_str in cards_str.chars() {
            cards_vec.push(Card::from_str(card_str.to_string().as_str())?);
        }

        let cards = cards_vec
            .try_into()
            .expect("the hand should have exactly five cards");

        Ok(Hand::new(cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            unequal => unequal,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(plays: &Vec<(Hand, u32)>) -> u32 {
    plays
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum()
}

fn parse_input(input_data: &str, j_is_joker: bool) -> Vec<(Hand, u32)> {
    let mut plays: Vec<(Hand, u32)> = vec![];

    if input_data.contains('★') {
        panic!("the input should not contain the Joker card symbol");
    }

    let plays_str = match j_is_joker {
        true => input_data.replace('J', "★"),
        false => input_data.to_owned(),
    };

    for line in plays_str.lines() {
        let [hand_str, bid_str]: [&str; 2] = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .try_into()
            .expect("the line should have a hand and a bid");

        let hand = Hand::from_str(hand_str).expect("this should be a well-formed hand");

        let bid = bid_str
            .parse::<u32>()
            .expect("the bid should be a positive integer");

        plays.push((hand, bid));
    }

    plays.sort();

    plays
}

pub fn day07(input_data: &str) {
    let plays_with_jack = parse_input(input_data, false);

    let sol1 = solve(&plays_with_jack);

    let plays_with_joker = parse_input(input_data, true);

    let sol2 = solve(&plays_with_joker);

    println!("{sol1}");
    println!("{sol2}");
}
