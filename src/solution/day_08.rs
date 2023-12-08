use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Letter = u8;

type Location = [Letter; 3];
type Network = HashMap<Location, (Location, Location)>;

#[derive(Debug, PartialEq, Eq)]
enum Turn { Left, Right }


#[derive(Debug, PartialEq, Eq)]
pub struct P { instructions: Vec<Turn>, network: Network }

pub struct Day(P);

impl Day {

    const START : [u8; 3] = [b'A'; 3];
    const FINISH: [u8; 3] = [b'Z'; 3];

    fn parse_instructions(line: &str) -> Vec<Turn> {
        line.chars().map(|c| match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
             _  => panic!("Could not recognize instruction '{}'", c),
        })
        .collect()
    }
    fn parse_network(text_input: &str) -> Network {
        let text_input = text_input.clone();
        Regex::new(r#"([\d\w]{3}) = \(([\d\w]{3}), ([\d\w]{3})\)"#)
        .unwrap()
        .captures_iter(text_input)
        .map(|c| {
            let (_, [p, l, r]) = c.extract();
            let p: [u8; 3] = [0,1,2].map(|i| p.as_bytes()[i as usize]);
            let l: [u8; 3] = [0,1,2].map(|i| l.as_bytes()[i as usize]);
            let r: [u8; 3] = [0,1,2].map(|i| r.as_bytes()[i as usize]);
            (p, (l, r))
        }).collect()
    }

    fn go_to_new_location(location: &Location, instruction: &Turn, network: &Network) -> Location {
        match instruction {
            Turn::Left  => network[location].0,
            Turn::Right => network[location].1,
        }
}
    fn run_network_once(start: Location, instructions: &Vec<Turn>, network: &Network) -> Location {
        instructions
        .iter()
        .fold(start, |location, instruction| {
            Self::go_to_new_location(&location, &instruction, network)
        })
    }

}

impl super::Solution for Day {

    const DAY_NUMBER: u8 = 8;

    type Answer = Option<u32>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        Regex::new(r#"([LR]+)\n\n([\S\s]+)"#).unwrap()
        .captures(&text_input)
        .map(|c| {
            let (_, [text_instructions, text_network]) = c.extract();
            Self::Problem{
                instructions: Day::parse_instructions(text_instructions),
                network: Day::parse_network(text_network),
            }
        })
        .unwrap()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let P { instructions, network } = problem;
        let run_length = instructions.len() as u32;
        let number_of_runs: u32 =
            (0..)
            .scan(Day::START, |position, _| {
                // make one run trhough network and update the position
                *position = Day::run_network_once(*position, &instructions, &network);
                match *position {
                    Day::FINISH => None,
                    _ => Some(*position)
                }
            })
            .count() as u32;
        let answer = (number_of_runs + 1) * run_length;
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let P { instructions, network } = problem;
        let start: Vec<Location> = network.keys().filter(|l| l[2] == b'A').map(|l| *l).collect();
        let answer =
            instructions.iter().cycle()
            .scan(start, |locations, instruction| {
                *locations =
                    locations
                    .iter()
                    .map(|l| Day::go_to_new_location(l, instruction, &network))
                    .collect();
                //println!("locations: {:?}", &locations);
                match locations.iter().all(|l| l[2] == b'Z') {
                    false => Some(locations.clone()),
                    true  => None,
                }
            })
            .take(1_000_000_000)
            .count() as u32;
        Some(answer + 1)
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
    use super::*;
    #[test]
    fn parse_network() {
        assert_eq!(
            Day::parse_network("AAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)"),
            HashMap::from([
                ([b'Z';3], ([b'Z';3], [b'Z';3])),
                ([b'B';3], ([b'A';3], [b'Z';3])),
                ([b'A';3], ([b'B';3], [b'B';3])),
            ])
        );
        assert_eq!(
            Day::parse_network("ABC = (DEF, HIJ)\nBBB = (AAA, ZZZ)"),
            HashMap::from([
                ([b'B';3], ([b'A';3], [b'Z';3])),
                ([b'A',b'B',b'C'], ([b'D',b'E',b'F'], [b'H',b'I',b'J'])),
            ])
        );
    }
    #[test]
    fn go_to_new_location() {
        let (aaa, bbb, zzz) = ([b'A';3], [b'B';3], [b'Z';3]);
        let network = Day::parse_network("AAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)");
        assert_eq!(Day::go_to_new_location(&aaa, &Turn::Left , &network), bbb);
        assert_eq!(Day::go_to_new_location(&aaa, &Turn::Right, &network), bbb);
        assert_eq!(Day::go_to_new_location(&bbb, &Turn::Left , &network), aaa);
        assert_eq!(Day::go_to_new_location(&bbb, &Turn::Right, &network), zzz);
        assert_eq!(Day::go_to_new_location(&zzz, &Turn::Right, &network), zzz);
        assert_eq!(Day::go_to_new_location(&zzz, &Turn::Left , &network), zzz);
    }

}