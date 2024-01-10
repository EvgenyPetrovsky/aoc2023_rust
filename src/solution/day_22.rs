use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brick {
    beg: Location,
    end: Location,
}
type Location = (u32, u32, u32);

type P = Vec<Brick>;

impl Brick {
    // parse the brick
    fn from(line: &str) -> Self {
        fn to_u32(str: &str) -> u32 {
            str.parse().unwrap()
        }
        let re = Regex::new(r#"(-?\d+),(-?\d+),(-?\d+)~(-?\d+),(-?\d+),(-?\d+)"#).unwrap();
        re.captures(line)
            .map(|c| {
                let (_, [x0, y0, z0, x1, y1, z1]) = c.extract();
                Brick {
                    beg: (to_u32(x0), to_u32(y0), to_u32(z0)),
                    end: (to_u32(x1), to_u32(y1), to_u32(z1)),
                }
            })
            .unwrap()
    }
    // validate if brick is on the ground
    fn on_the_ground(&self) -> bool {
        let (z, _) = Self::order(self.beg.2, self.end.2);
        z == 1
    }
    // check if brick is laying on another brick
    fn brick_overlap(&self, other: &Self) -> bool {
        let x_segments = ((self.beg.0, self.end.0), (other.beg.0, other.end.0));
        let y_segments = ((self.beg.1, self.end.1), (other.beg.1, other.end.1));
        let z_segments = ((self.beg.2, self.end.2), (other.beg.2, other.end.2));
        [x_segments, y_segments, z_segments]
            .iter()
            .all(|(s1, s2)| Self::segment_overlap(s1, s2))
    }
    // order values
    fn order<T>(v1: T, v2: T) -> (T, T)
    where
        T: Ord,
    {
        match v1.cmp(&v2) {
            Ordering::Greater => (v2, v1),
            _ => (v1, v2),
        }
    }
    // check if segments overlap
    fn segment_overlap(s1: &(u32, u32), s2: &(u32, u32)) -> bool {
        let s1 = Self::order(s1.0, s1.1);
        let s2 = Self::order(s2.0, s2.1);
        s2.0 <= s1.0 && s1.0 <= s2.1
            || s2.0 <= s1.1 && s1.1 <= s2.1
            || s1.0 <= s2.0 && s2.0 <= s1.1
            || s1.0 <= s2.1 && s2.1 <= s1.1
    }
    // move brick down by 1
    fn move_down(&self) -> Self {
        let mut new = self.clone();
        new.beg.2 -= 1;
        new.end.2 -= 1;
        new
    }
}

impl DaySolution {
    // let all bricks move down until they lay on the ground of yeach other
    // function returns new state of bricks and number of moves they did
    fn let_bricks_fall(bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
        fn iterate(bricks: Vec<Brick>, moves: usize) -> (Vec<Brick>, usize) {
            let new = bricks.iter().enumerate().map(|(index, brick)| {
                let on_the_ground = brick.on_the_ground();
                let new_place = brick.move_down();
                let any_overlap = bricks
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != index)
                    .any(|(_, other)| new_place.brick_overlap(other));

                if !on_the_ground && !any_overlap {
                    (new_place, 1)
                } else {
                    (brick.clone(), 0_usize)
                }
            });
            //.collect::<Vec<(Brick, usize)>>();
            let (new_bricks, new_moves): (Vec<Brick>, Vec<usize>) = new.unzip();
            let new_moves: usize = new_moves.iter().sum();
            if new_moves > 0 {
                iterate(new_bricks, new_moves + moves)
            } else {
                (bricks, moves)
            }
        }
        iterate(bricks, 0)
    }
    fn remove_falling_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
        let init_count = bricks.len();
        let new: Vec<Brick> = bricks
            .iter()
            .enumerate()
            .filter(|(index, brick)| {
                let on_the_ground = brick.on_the_ground();
                let new_place = brick.move_down();
                let any_overlap = bricks
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| idx != index)
                    .any(|(_, other)| new_place.brick_overlap(other));
                on_the_ground || any_overlap
            })
            .map(|(_, brick)| brick.clone())
            .collect();
        let new_count = new.len();
        if new_count < init_count {
            Self::remove_falling_bricks(new)
        } else {
            bricks
        }
    }
}

pub struct DaySolution(P);

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 22;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input.lines().map(Brick::from).collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        /*
        Approach:
        first let all bricks settle down
        then move along bricks in cycle,
        remove brick and see if bricks will make any moves, count all such cases
        */
        let (bricks, _) = DaySolution::let_bricks_fall(problem);
        let answer = bricks
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                let mut temp_bricks = bricks.clone();
                temp_bricks.remove(*index);
                let (_, moves) = DaySolution::let_bricks_fall(temp_bricks);
                moves == 0
            })
            .count();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        /*
        Approach:
        first let all bricks settle down
        then move along bricks in cycle,
        remove brick and
          see if bricks will make any moves,
          remove all bricks that move
          count difference between initial number and remaining number
        */
        let (bricks, _) = DaySolution::let_bricks_fall(problem);
        let answer = bricks
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let mut temp_bricks = bricks.clone();
                temp_bricks.remove(index);
                let temp_len = temp_bricks.len();
                let remaining_bricks = DaySolution::remove_falling_bricks(temp_bricks);
                let rem_len = remaining_bricks.len();
                temp_len - rem_len
            })
            .sum();
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
