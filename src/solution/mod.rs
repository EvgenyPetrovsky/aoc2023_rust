use std::fs;

pub trait Solution {
    const DAY_NUMBER: u8;

    type Answer;
    type Problem;

    // Solution Trait
    fn get_filepath(mode: super::Mode) -> String {
        let day_no = Self::DAY_NUMBER;

        let folder = match mode {
            super::Mode::Test => "input_test",
            super::Mode::Real => "input",
        };
        // use formatting to construct the name of the file
        format!("./{f}/day_{n:0>2}.txt", f = folder, n = day_no)
    }

    fn load_file_input(mode: super::Mode) -> String {
        let path = Self::get_filepath(mode);
        fs::read_to_string(path).expect("Couldn't read file")
    }

    fn run(part: super::Part, mode: super::Mode, stdin: &Option<String>) -> String {
        let input = match stdin {
            Some(data) => data.clone(),
            _ => Self::load_file_input(mode),
        };
        let problem = match part {
            super::Part::One => Self::parse_input_part_1(input),
            super::Part::Two => Self::parse_input_part_2(input),
        };
        let answer = match part {
            super::Part::One => Self::solve_part_1(problem),
            super::Part::Two => Self::solve_part_2(problem),
        };
        Self::show_answer(answer)
    }

    fn parse_input_part_1(text_input: String) -> Self::Problem;

    fn parse_input_part_2(text_input: String) -> Self::Problem;

    fn solve_part_1(problem: Self::Problem) -> Self::Answer;

    fn solve_part_2(problem: Self::Problem) -> Self::Answer;

    fn show_answer(answer: Self::Answer) -> String;
}

// template module
mod day_xx;
// real modules
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;
