use clap::Parser;
use std::io;

const YEAR: i32 = 2023;

/// Advent of Code 2023 launcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day number to solve
    #[arg(short, long)]
    day: u8,
    /// get input data for day problem from site
    /*
    For this option to work, SESSION file is required. Session file must contain the 'session' variable of cookie for advent of code site.
    More details on how to obtain session can be found here:
    https://www.reddit.com/r/adventofcode/comments/a2vonl/how_to_download_inputs_with_a_script/
    */
    #[arg(short, long)]
    get_input: bool,
    /// Read data from standard input
    #[arg(short, long)]
    stdin: bool,
    /// Solve problem in test mode
    #[arg(short, long)]
    test: bool,
    /// Define the level of logging output when running the solution
    #[arg(short, long)]
    debug: bool,
}

use solution::Solution;

mod solution;
mod utils;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Logging {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

fn main() {
    // cli interface
    let args = Cli::parse();
    let day = args.day;
    let mode = if args.test { Mode::Test } else { Mode::Real };
    let dlin = args.get_input;
    // standard input
    let stdin: Option<String> = if args.stdin {
        io::read_to_string(io::stdin()).ok()
    } else {
        None
    };
    if dlin {
        utils::download_input(YEAR, day as u32).unwrap();
    } else {
        let answer_1 = get_solution(day, Part::One, mode, &stdin);
        let answer_2 = get_solution(day, Part::Two, mode, &stdin);
        println!("The part 1 answer is: {}", answer_1);
        println!("The part 2 answer is: {}", answer_2);
    }
}

fn get_solution(day: Day, part: Part, mode: Mode, stdin: &Option<String>) -> String {
    match day {
        1 => solution::day_01::DaySolution::run(part, mode, stdin),
        2 => solution::day_02::DaySolution::run(part, mode, stdin),
        3 => solution::day_03::DaySolution::run(part, mode, stdin),
        4 => solution::day_04::DaySolution::run(part, mode, stdin),
        5 => solution::day_05::DaySolution::run(part, mode, stdin),
        6 => solution::day_06::DaySolution::run(part, mode, stdin),
        7 => solution::day_07::DaySolution::run(part, mode, stdin),
        8 => solution::day_08::Day::run(part, mode, stdin),
        9 => solution::day_09::DaySolution::run(part, mode, stdin),
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
    }
}
