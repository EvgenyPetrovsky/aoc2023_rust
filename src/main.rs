use clap::Parser;
use std::io;

/// Advent of Code 2023 launcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day number to solve
    #[arg(short, long)]
    day: u8,
    /// Read data from standard input
    #[arg(short, long)]
    stdin: bool,
    /// Solve problem in test mode
    #[arg(short, long)]
    test: bool,
}


use solution::Solution;

mod solution;

pub enum Part {
    One,
    Two,
}

type Day = u8;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Test,
    Real,
}

fn main() {
    // cli interface
    let args = Cli::parse();
    let day = args.day;
    let mode = if args.test { Mode::Test } else { Mode::Real };
    // standard input
    let stdin: Option<String> =
        if args.stdin {
            io::read_to_string(io::stdin()).ok()
        } else {
            None
        };
    // redefine mode
    let answer_1 = get_solution(day, Part::One, mode, &stdin);
    let answer_2 = get_solution(day, Part::Two, mode, &stdin);
    println!("The part 1 answer is: {}", answer_1);
    println!("The part 2 answer is: {}", answer_2);

}

fn get_solution(day: Day, part: Part, mode: Mode, stdin: &Option<String>) -> String {
    let answer = match day {
        01 => solution::day_01::DaySolution::run(part, mode, stdin),
        02 => solution::day_02::DaySolution::run(part, mode, stdin),
        03 => solution::day_03::DaySolution::run(part, mode, stdin),
        04 => solution::day_04::DaySolution::run(part, mode, stdin),
        05 => solution::day_05::DaySolution::run(part, mode, stdin),
        06 => solution::day_06::DaySolution::run(part, mode, stdin),
        07 => solution::day_07::DaySolution::run(part, mode, stdin),
        08 => solution::day_08::Day::run(part, mode, stdin),
        09 => solution::day_09::DaySolution::run(part, mode, stdin),
        10 => solution::day_10::DaySolution::run(part, mode, stdin),
        11 => solution::day_11::DaySolution::run(part, mode, stdin),
        12 => solution::day_12::DaySolution::run(part, mode, stdin),
        13 => solution::day_13::DaySolution::run(part, mode, stdin),
        14 => solution::day_14::DaySolution::run(part, mode, stdin),
        15 => solution::day_15::DaySolution::run(part, mode, stdin),
        16 => solution::day_16::DaySolution::run(part, mode, stdin),
        17 => solution::day_17::DaySolution::run(part, mode, stdin),
        18 => solution::day_18::DaySolution::run(part, mode, stdin),
        19 => solution::day_19::DaySolution::run(part, mode, stdin),
        20 => solution::day_20::DaySolution::run(part, mode, stdin),
        21 => solution::day_21::DaySolution::run(part, mode, stdin),
        22 => solution::day_22::DaySolution::run(part, mode, stdin),
        23 => solution::day_23::DaySolution::run(part, mode, stdin),
        24 => solution::day_24::DaySolution::run(part, mode, stdin),
        25 => solution::day_25::DaySolution::run(part, mode, stdin),
        _ => panic!("unrecognized day '{day}', it must be number between 1 and 25"),
    };
    answer
}
