// Copyright (C) 2023 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

mod days;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 || args.len() > 3 {
        eprintln!("Usage: advent <DAY> [<INPUT_EXTENSION>]");

        return std::process::ExitCode::FAILURE;
    }

    {
        let program_path = std::env::current_exe();
        if program_path.is_ok() {
            let path = program_path.unwrap();
            let root_path = path.ancestors().nth(3).unwrap();
            let _ = std::env::set_current_dir(root_path);
        }
    }

    let day_number: i64 = match args.get(1).unwrap().parse() {
        Ok(val) => val,
        Err(e) => {
            match e.kind() {
                std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow => {
                    eprintln!("advent: The day typed is too large.");
                }
                _ => {
                    eprintln!("advent: Please type a whole number as the day.");
                }
            };

            return std::process::ExitCode::FAILURE;
        }
    };

    if day_number < 0 {
        eprintln!("advent: Please type a whole number as the day.");

        return std::process::ExitCode::FAILURE;
    }

    let day_str = if day_number < 10 {
        format!("0{day_number}")
    } else {
        format!("{day_number}")
    };

    let input_extension = match args.get(2) {
        Some(val) => format!(".{val}"),
        None => String::from(""),
    };

    let input_filename = format!("day{day_str}{input_extension}.txt");

    let path = std::env::current_dir()
        .unwrap()
        .join("inputs")
        .join(&input_filename);

    let input_data = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("advent: {input_filename} was not found.");
                }
                _ => {
                    eprintln!("advent: Could not open the input file.");
                }
            };

            return std::process::ExitCode::FAILURE;
        }
    };

    if days::DAYS.len() < (day_number as usize) {
        eprintln!("advent: The day typed currently does not exist.");

        return std::process::ExitCode::FAILURE;
    }

    days::DAYS[(day_number - 1) as usize](input_data.as_str());

    std::process::ExitCode::SUCCESS
}
