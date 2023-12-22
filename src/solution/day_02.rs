use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallSet {
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    sets: Vec<BallSet>,
}

type P = Vec<Game>;

pub struct DaySolution(P);

impl DaySolution {
    const ZERO_SET: BallSet = BallSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    fn sum_ball_sets(a: BallSet, b: BallSet) -> BallSet {
        BallSet {
            red: a.red + b.red,
            green: a.green + b.green,
            blue: a.blue + b.blue,
        }
    }

    fn max_ball_sets(a: BallSet, b: BallSet) -> BallSet {
        BallSet {
            red: a.red.max(b.red),
            green: a.green.max(b.green),
            blue: a.blue.max(b.blue),
        }
    }

    fn parse_one_set(set_of_balls: &str) -> BallSet {
        let re_balls = Regex::new(r#"(\d+) (\w+)"#).unwrap();
        re_balls
            .captures_iter(set_of_balls)
            .map(|c| {
                let n: u8 = c.get(1).unwrap().as_str().parse().unwrap();
                let c: &str = c.get(2).unwrap().as_str();
                match c {
                    "red" => BallSet {
                        red: n,
                        ..Self::ZERO_SET
                    },
                    "green" => BallSet {
                        green: n,
                        ..Self::ZERO_SET
                    },
                    "blue" => BallSet {
                        blue: n,
                        ..Self::ZERO_SET
                    },
                    _ => panic!("unrecognized color: '{}'", c),
                }
            })
            .fold(Self::ZERO_SET, Self::sum_ball_sets)
    }

    fn parse_one_line(line: &str) -> Game {
        let re_game = Regex::new(r#"^Game (\d+):(.*)$"#).unwrap();
        let (game_id, game_sets): (u32, &str) = re_game
            .captures(line)
            .map(|c| {
                let id: u32 = c.get(1).unwrap().as_str().parse().unwrap();
                let gs = c.get(2).unwrap().as_str();
                (id, gs)
            })
            .unwrap();
        let re_set = Regex::new(r#"[\d a-z,]+"#).unwrap();
        let game_sets = re_set
            .captures_iter(game_sets)
            .map(|c| c.get(0).unwrap().as_str())
            .map(Self::parse_one_set)
            .collect();
        Game {
            id: game_id,
            sets: game_sets,
        }
    }
}

impl super::Solution for DaySolution {
    type Answer = Option<u32>;
    type Problem = P;

    const DAY_NUMBER: u8 = 2;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(DaySolution::parse_one_line)
            .collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let (max_r, max_g, max_b) = (12_u8, 13_u8, 14_u8);
        let valid_id_sum: u32 = problem
            .iter()
            .map(|g| {
                let folded_set = g.sets.iter().fold(DaySolution::ZERO_SET, |z, &x| {
                    DaySolution::max_ball_sets(z, x)
                });
                (g.id, folded_set)
            })
            .filter(|(_id, s)| {
                let condition = s.red <= max_r && s.green <= max_g && s.blue <= max_b;
                //println!("{} Game {:>3}: red: {:>2}, green: {:>2}, blue: {:>2}", if condition {'*'} else {' '}, _id, s.red, s.green, s.blue);
                condition
            })
            .fold(0, |z, x| z + x.0);
        Some(valid_id_sum)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let power_sum: u32 = problem
            .iter()
            .map(|g| {
                let folded_set = g.sets.iter().fold(DaySolution::ZERO_SET, |z, &x| {
                    DaySolution::max_ball_sets(z, x)
                });
                folded_set
            })
            .map(|s| s.red as u32 * s.green as u32 * s.blue as u32)
            .sum();
        Some(power_sum)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}

#[cfg(test)]
mod tests {

    //use super::{Room, DaySolution};

    use crate::solution::day_02::{BallSet, DaySolution, Game};

    #[test]
    fn parse_one_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        //BallSet { red: 4, green: 0, blue: 3 },
        //BallSet { red: 1, green: 2, blue: 6 },
        //BallSet { red: 0, green: 2, blue: 0 }
        assert_eq!(
            DaySolution::parse_one_line(line),
            Game {
                id: 1,
                sets: vec![
                    BallSet {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    BallSet {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    BallSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        )
    }
}

//
