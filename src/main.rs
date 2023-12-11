/*
use clap::Parser;

#[derive(Parser)]
struct{
    // day number to process
    day: u8,
    // run in test mode
    test: bool
}
*/

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
    let day = 11;
    let mode = Mode::Real;
    let answer_1 = get_solution(day, Part::One, mode);
    let answer_2 = get_solution(day, Part::Two, mode);
    println!("The part 1 answer is: {}", answer_1);
    println!("The part 2 answer is: {}", answer_2);
}

fn get_solution(day: Day, part: Part, mode: Mode) -> String {
    let answer = match day {
        01 => solution::day_01::DaySolution::run(part, mode),
        02 => solution::day_02::DaySolution::run(part, mode),
        03 => solution::day_03::DaySolution::run(part, mode),
        04 => solution::day_04::DaySolution::run(part, mode),
        05 => solution::day_05::DaySolution::run(part, mode),
        06 => solution::day_06::DaySolution::run(part, mode),
        07 => solution::day_07::DaySolution::run(part, mode),
        08 => solution::day_08::Day::run(part, mode),
        09 => solution::day_09::DaySolution::run(part, mode),
        10 => solution::day_10::DaySolution::run(part, mode),
        11 => solution::day_11::DaySolution::run(part, mode),
        12 => solution::day_12::DaySolution::run(part, mode),
        13 => solution::day_13::DaySolution::run(part, mode),
        14 => solution::day_14::DaySolution::run(part, mode),
        15 => solution::day_15::DaySolution::run(part, mode),
        16 => solution::day_16::DaySolution::run(part, mode),
        17 => solution::day_17::DaySolution::run(part, mode),
        18 => solution::day_18::DaySolution::run(part, mode),
        19 => solution::day_19::DaySolution::run(part, mode),
        20 => solution::day_20::DaySolution::run(part, mode),
        21 => solution::day_21::DaySolution::run(part, mode),
        22 => solution::day_22::DaySolution::run(part, mode),
        23 => solution::day_23::DaySolution::run(part, mode),
        24 => solution::day_24::DaySolution::run(part, mode),
        25 => solution::day_25::DaySolution::run(part, mode),
        _ => panic!("unrecognized day '{day}', it must be number between 1 and 25")
    };
    answer
}
