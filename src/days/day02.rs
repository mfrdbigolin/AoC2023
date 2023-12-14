// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Two, Cube Conundrum.

type Game = Vec<(u32, u32, u32)>;

fn solve1(games: &Vec<Game>, bag: (u32, u32, u32)) -> usize {
    let mut valid_id_sum = 0;

    for (id, game) in games.iter().enumerate() {
        let is_valid = game
            .iter()
            .all(|subset| (subset.0 <= bag.0) && (subset.1 <= bag.1) && (subset.2 <= bag.2));

        if is_valid {
            valid_id_sum += id + 1;
        }
    }

    valid_id_sum
}

fn solve2(games: &Vec<Game>) -> u32 {
    let mut power_sum = 0;

    for game in games {
        let min_set = game.iter().fold(game[0], |acc, rev| {
            (
                std::cmp::max(acc.0, rev.0),
                std::cmp::max(acc.1, rev.1),
                std::cmp::max(acc.2, rev.2),
            )
        });

        power_sum += min_set.0 * min_set.1 * min_set.2;
    }

    power_sum
}

fn parse_input(records: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for record in records.lines() {
        let subsets: Vec<&str> = record
            .split(": ")
            .nth(1)
            .expect("the game record is ill-formed")
            .split("; ")
            .collect();

        let mut game = Game::new();

        for subset in subsets {
            let cubes: Vec<&str> = subset.split(", ").collect();

            let mut cube_quantities: (u32, u32, u32) = (0, 0, 0);

            for cube_str in cubes {
                let mut cube = cube_str.split_whitespace();

                let quantity: u32 = cube
                    .next()
                    .expect("the cube description should not be empty")
                    .parse()
                    .expect("the quantity should be a positive integer");

                let cube_color = cube
                    .next()
                    .expect("there should be a color name following the quantity")
                    .to_lowercase();

                match cube_color.as_str() {
                    "red" => cube_quantities.0 = quantity,
                    "green" => cube_quantities.1 = quantity,
                    "blue" => cube_quantities.2 = quantity,
                    _ => panic!("unexpected color: '{cube_color}'"),
                };
            }

            game.push(cube_quantities);
        }

        games.push(game);
    }

    games
}

pub fn day02(input_data: &str) {
    let games = parse_input(input_data);

    let sol1 = solve1(&games, (12, 13, 14));
    let sol2 = solve2(&games);

    println!("{sol1}");
    println!("{sol2}");
}
