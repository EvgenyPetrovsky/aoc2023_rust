use regex::Regex;

type P = Vec<u32>;

pub struct DaySolution(P);

impl DaySolution {

    fn parse_one_line_1(line: &str) -> u32 {
        let re = Regex::new(r#"\d{1}"#).unwrap();
        let d1 =
        re
            .captures_iter(line)
            .nth(0)
            .map(|cap| {
                cap.get(0).unwrap().as_str().parse::<u32>().unwrap()
            })
            .unwrap();
        let d0 =
        re
            .captures_iter(line)
            .last()
            .map(|cap| {
                cap.get(0).unwrap().as_str().parse::<u32>().unwrap()
            })
            .unwrap();
        d1 * 10 + d0
    }

    fn parse_one_line_2(line: &str) -> u32 {

        fn extract_digit(x: &str) -> u32 {
            match x {
                "0" | "zero"  => 0,
                "1" | "one"   => 1,
                "2" | "two"   => 2,
                "3" | "three" => 3,
                "4" | "four"  => 4,
                "5" | "five"  => 5,
                "6" | "six"   => 6,
                "7" | "seven" => 7,
                "8" | "eight" => 8,
                "9" | "nine"  => 9,
                other => panic!("unrecognized digit {}", other),
            }
        }
        // take first occurence
        let re_l = Regex::new(r#"\d|zero|one|two|three|four|five|six|seven|eight|nine"#).unwrap();
        // be greedy and take it up to last occurence
        let re_r = Regex::new(r#"\w*(\d|zero|one|two|three|four|five|six|seven|eight|nine)"#).unwrap();
        let d1 =
        re_l
            .captures_iter(line)
            .nth(0)
            .map(|cap| cap.get(0).unwrap().as_str())
            .map(extract_digit)
            .unwrap();
        let d0 =
        re_r
            .captures_iter(line)
            .last()
            .map(|cap| cap.get(1).unwrap().as_str())
            .map(extract_digit)
            .unwrap();
        d1 * 10 + d0
    }

}

impl super::Solution for DaySolution {


    type Answer = Option<u32>;
    type Problem = P;

    const DAY_NUMBER: u8 = 1;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input.lines().map(DaySolution::parse_one_line_1).collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        text_input.lines().map(DaySolution::parse_one_line_2).collect()
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem.iter().sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = problem.iter().sum();
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}

#[cfg(test)]
mod tests {

    //use super::{Room, DaySolution};

    #[test]
    fn parse_one_line_1() {
        assert_eq!(super::DaySolution::parse_one_line_1("1abc2"), 12)
    }

    #[test]
    fn parse_one_line_2() {
        assert_eq!(super::DaySolution::parse_one_line_2("two1nine"), 29);
        assert_eq!(super::DaySolution::parse_one_line_2("eightwothree"), 83);
        assert_eq!(super::DaySolution::parse_one_line_2("abcone2threexyz"), 13);
        assert_eq!(super::DaySolution::parse_one_line_2("xtwone3four"), 24);
        assert_eq!(super::DaySolution::parse_one_line_2("4nineeightseven2"), 42);
        assert_eq!(super::DaySolution::parse_one_line_2("zoneight234"), 14);
        assert_eq!(super::DaySolution::parse_one_line_2("7pqrstsixteen"), 76);
        assert_eq!(super::DaySolution::parse_one_line_2("eightwo"), 82);
        assert_eq!(super::DaySolution::parse_one_line_2("twocsfzd1eight7eightwovm"), 22);

    }

}
