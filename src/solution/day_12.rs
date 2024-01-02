use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

const S_W: u8 = b'.';
const S_B: u8 = b'#';
const S_U: u8 = b'?';

type Springs = Vec<u8>;
// memory is a cache for current positions, number of rem indexes and associated number of solutions
type Memory = HashMap<(usize, usize), usize>;

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    springs: Springs,
    brokens: Vec<usize>,
}

type P = Vec<Record>;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Record {
        let springs: Springs = Regex::new(r#"[?#\.]+"#)
            .unwrap()
            .captures(line)
            .map(|c| c.get(0).unwrap().as_str())
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => S_W,
                '#' => S_B,
                '?' => S_U,
                _ => panic!("Couldn't recognize char '{}'", c),
            })
            .collect();

        let brokens = Regex::new(r#"\d+"#)
            .unwrap()
            .captures_iter(line)
            .map(|c| c.get(0).unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        Record { springs, brokens }
    }

    fn parse_one_line_part_2(line: &str) -> Record {
        let splits = String::from(line)
            .split(" ")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let s5 = vec![splits[0].clone(); 5].join("?");
        let b5 = vec![splits[1].clone(); 5].join(",");
        Self::parse_one_line(&(vec![s5, b5].join(" ")))
    }

    // use all reversed values to easier reason about the end (0)
    fn rev_calculate(
        springs: &Springs,
        brokens: &Vec<usize>,
        pos: usize,
        brk_idx: usize,
        rem_brk: usize,
        min_req_len: usize,
        memory: Memory,
    ) -> (usize, Memory) {
        let (spr, pos, brk, idx, rem, req) = (springs, pos, brokens, brk_idx, rem_brk, min_req_len);
        let init_brk = brk[idx];
        //let debug = false;
        let spr_at_pos = spr[pos];
        //if remaining broken springs = 0 and rev_brk_idx = 0 and we reached the end of springs, then = 1
        if pos + 1 < req {
            (0, memory)
        } else if pos == 0 && idx == 0 && rem == 1 && (spr_at_pos == S_B || spr_at_pos == S_U) {
            //if debug {println!("pos == 0 && idx == 0 && rem == 1 && (spr_at_pos == S_B || spr_at_pos == S_U)")};
            (1, memory)
        } else if pos == 0 && idx == 0 && rem == 0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("pos == 0 && idx == 0 && rem == 0 && (spr_at_pos == S_W || spr_at_pos == S_U)")};
            (1, memory)
        }
        //if remaining broken springs > 0 and re reached end of springs
        else if pos == 0 && (idx > 0 || rem > 0) {
            //if debug {println!("pos == 0 && (idx > 0 || rem > 0)")};
            (0, memory)
        }
        //if remaining broken springs = 0 and we find B-spring, then = 0
        else if rem == 0 && spr_at_pos == S_B {
            //if debug {println!("rem == 0 && spr_at_pos == S_B")};
            (0, memory)
        } else if pos == 0 {
            panic!("non-handled pos == 0 condition")
        }
        //if remaining broken springs > 0 and series of broken springs has not yet started remaining broken = init_broken
        //    - move on to next spring and search for brokens
        //    - move on to next spring and postpone search
        else if rem == init_brk && spr_at_pos == S_U {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_U")};
            if let Some(n) = memory.get(&(pos, idx)) {
                //println!("** use cache entry [pos:{}, idx:{}] => {}", pos, idx, n);
                (*n, memory)
            } else {
                let (n1, m1) =
                    Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1, req - 1, memory);
                let (n2, mut m2) =
                    Self::rev_calculate(spr, brk, pos - 1, idx, rem, req, m1);
                m2.insert((pos, idx), n1 + n2);
                //println!(">> add cache entry [pos:{}, idx:{}] => {}", pos, idx, n1+n2);
                (n1 + n2, m2)
            }
        }
        //if remaining broken springs > 0 and we find W-spring, then = 0
        else if rem == init_brk && spr_at_pos == S_W {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_W")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem, req, memory)
        }
        //if remaining broken springs > 0 and we find B-spring, then
        //    - move on
        else if rem == init_brk && spr_at_pos == S_B {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_B")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1, req - 1, memory)
        } else if rem == init_brk {
            panic!("non handled rem == init_brk condition")
        }
        // if remaining broken in the index is 0 but next spring is broken then = 0
        else if rem == 0 && spr_at_pos == S_B {
            //if debug {println!("if rem == 0 && spr_at_pos == S_B")};
            (0, memory)
        }
        // if remaining broken in the index is 0 but next spring is unknown or working then
        //    - move on by decreasing idx and
        else if rem == 0 && idx > 0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("rem == 0 && idx >  0 && ( spr_at_pos == S_W || spr_at_pos == S_U )")};
            Self::rev_calculate(spr, brk, pos - 1, idx - 1, brk[idx - 1], req - 1, memory)
        }
        // if remaining broken = 0 and no indexes and current spring is W or U, move on to next
        else if rem == 0 && idx == 0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("rem == 0 && idx == 0 && (spr_at_pos == S_W || spr_at_pos == S_U)")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem, req, memory)
        } else if rem == 0 {
            panic!("non handled rem == 0 condition")
        }
        //if remaining broken springs > 0 and we find W-spring, then = 0
        else if rem > 0 && spr_at_pos == S_W {
            //if debug {println!("rem > 0 && rem < init_brk && spr_at_pos == S_W")};
            (0, memory)
        }
        //if remaining broken springs > 0 and we find broken or unknown spring then continue
        else if rem > 0 && (spr_at_pos == S_B || spr_at_pos == S_U) {
            //if debug {println!("pos > 0 && rem > 0 && rem != init_brk && (spr_at_pos == S_B || spr_at_pos == S_U)")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1, req - 1, memory)
        } else if rem > 0 {
            panic!("non handled rem > 0 condition")
        } else {
            panic!(
                "Undefined case! pos: {pos}, brk_idx {idx}, rem: {rem}, pos_val: {:?}",
                spr_at_pos
            )
        }
    }

    fn process_one_record(record: &Record) -> usize {
        let init_pos = record.springs.len() - 1;
        let init_brk_idx = record.brokens.len() - 1;
        let rem_brk = record.brokens[init_brk_idx];

        // minimum required len for all broken springs and spaces between
        let min_req_len: usize = record.brokens.iter().fold(0_usize, |z, x| z + x) + init_brk_idx;
        let empty_memory: Memory = HashMap::new();

        let (num_combinations, _) = Self::rev_calculate(
            &record.springs,
            &record.brokens,
            init_pos,
            init_brk_idx,
            rem_brk,
            min_req_len,
            empty_memory,
        );
        num_combinations
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 12;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(DaySolution::parse_one_line)
            .collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(DaySolution::parse_one_line_part_2)
            .collect()
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(DaySolution::process_one_record)
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .par_iter()
            .map(DaySolution::process_one_record)
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

#[cfg(test)]
mod tests {
    use super::{DaySolution as DS, Record as Rec, S_B, S_U, S_W};

    #[test]
    fn parse_one_line() {
        assert_eq!(
            DS::parse_one_line(".#? 1,1"),
            Rec {
                springs: vec![S_W, S_B, S_U],
                brokens: vec![1, 1]
            }
        )
    }
    #[test]
    fn parse_one_line_part_2() {
        assert_eq!(
            DS::parse_one_line_part_2(".# 1,2"),
            Rec {
                springs: vec![S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B],
                brokens: vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]
            }
        )
    }

    #[test]
    fn process_one_simple_record() {
        /*
            assert_eq!(
                DS::process_one_record(&DS::parse_one_line("?? 1")),
                2
            );
            assert_eq!(
                DS::process_one_record(&DS::parse_one_line("??? 1,1")),
                1
            );
            assert_eq!(
                DS::process_one_record(&DS::parse_one_line("??#?? 2")),
                2
            );
            assert_eq!(
                DS::process_one_record(&DS::parse_one_line("?.?? 1,2")),
                0
            );
        */
        assert_eq!(DS::process_one_record(&DS::parse_one_line("...# 1")), 1);
    }

    #[test]
    fn process_one_heavy_record() {
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("?#.??????#??#?#?#?#? 1,1,15")),
            1_usize
        );
    }

    #[test]
    fn process_one_heavy_record_part_2() {
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line_part_2(".?????????? 2,2")),
            111063614
        );
    }

    #[test]
    fn process_one_record() {
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("???.### 1,1,3")),
            1_usize
        );
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line(".??..??...?##. 1,1,3")),
            4_usize
        );
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("?#?#?#?#?#?#?#? 1,3,1,6")),
            1_usize
        );
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("????.#...#... 4,1,1")),
            1_usize
        );
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("????.######..#####. 1,6,5")),
            4_usize
        );
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("?###???????? 3,2,1")),
            10_usize
        );
    }
}
