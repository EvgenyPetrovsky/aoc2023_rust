use regex::Regex;
use rayon::prelude::*;

const S_W: u8 = b'.';
const S_B: u8 = b'#';
const S_U: u8 = b'?';


#[derive(Debug, Clone, PartialEq, Eq)]
enum S {
    W,
    B,
    U,
}

type Springs = Vec<u8>;

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

    /*
        fn fast_permutations(broken: usize, total: usize) -> Vec<Springs> {
            fn iter_fun(broken: usize, total: usize) -> Vec<Springs> {
                if total > 0 {
                    let mut v1: Vec<Springs> = vec![vec![]];
                    let mut v2: Vec<Springs> = vec![vec![]];
                    if broken > 0 {
                        v1 = iter_fun(broken - 1, total - 1);
                        for idx in 0..v1.len() {
                            v1[idx].push(S_B);
                        }
                    } else {
                        v1.clear();
                    };
                    if total - broken > 0 {
                        v2 = iter_fun(broken, total - 1);
                        for idx in 0..v2.len() {
                            v2[idx].push(S_W);
                        }
                    } else {
                        v2.clear()
                    }
                    let v: Vec<Springs> =
                        v1.iter().chain(v2.iter()).map(|v| v.clone()).collect();
                    v
                } else {
                    vec![vec![]]
                }
            }
            let result = iter_fun(broken, total);
            //result.iter().for_each(|v| println!("Broken {:>2}, Total {:>2}: {:?}", broken, total, v));
            result
        }

        fn enumerate_unknowns(springs: &Springs) -> Vec<Option<usize>> {
            springs
                .iter()
                .map(|&s| s == S_U)
                .scan((None::<Option<S>>, 0_usize), |(_, n), x| {
                    if x {
                        *n += 1;
                        Some((Some(*n - 1), *n))
                    } else {
                        Some((None, *n))
                    }
                })
                .map(|(x, _)| x)
                .collect()
        }

        fn substitute_unknowns(
            springs: &Springs,
            replacement_springs: &Springs,
        ) -> Springs {
            let idx_unknowns = Self::enumerate_unknowns(springs);
            springs
                .iter()
                .enumerate()
                .map(|(idx, s)| match s {
                    S_U => replacement_springs[idx_unknowns[idx].unwrap()],
                    _ => *s,
                })
                .collect()
        }

        fn generate_brokens_regexp(record: &Record) -> String {
            record
                .brokens
                .iter()
                .map(|&x| format!("{:B<1$}", "", x))
                .collect::<Vec<String>>()
                .join("W+")
        }

        fn process_one_record_old(record: &Record) -> usize {
            let springs = &(record.springs);
            let unknown_count: usize = springs
                .iter()
                .filter(|&&s| s == S_U)
                .count();
            let known_broken_count: usize = springs
                .iter()
                .filter(|&&s| s == S_B)
                .count();
            let total_broken_count: usize = record.brokens.iter().sum();
            //println!("total_broken_count {}, known_broken_count: {}, unknown_count: {}", total_broken_count, known_broken_count, unknown_count);
            let replacements =
                Self::fast_permutations(total_broken_count - known_broken_count, unknown_count);

            let brokens_as_regex_str = Self::generate_brokens_regexp(record);
            let re = Regex::new(brokens_as_regex_str.as_str()).unwrap();
            replacements
                .iter()
                .map(|replacement_springs| Self::substitute_unknowns(&springs, replacement_springs))
                .map(|v| {
                    v.iter()
                        .map(|&s| if s == S_B { 'B' } else { 'W' })
                        .collect::<String>()
                })
                .filter(|s| re.is_match(s))
                .count()
        }
    */
    // use all reversed values to easier reason about the end (0)
    fn rev_calculate(
        springs: &Springs,
        brokens: &Vec<usize>,
        pos: usize,
        brk_idx: usize,
        rem_brk: usize,
    ) -> usize {
        let (spr, pos, brk, idx, rem) = (springs, pos, brokens, brk_idx, rem_brk);
        let init_brk = brk[idx];
        //let debug = false;
        let spr_at_pos = spr[pos];
        //if debug {println!("spr: {:?}, pos: {}, brk: {:?}, idx: {}, rem: {}, init_brk: {}", &spr, pos, &brk, idx, rem, init_brk)};
        //if remaining broken springs = 0 and rev_brk_idx = 0 and we reached the end of springs, then = 1

        //let must_be_working = rem == 0 || rem == init_brk;
        //let must_be_broken = rem > 0 || rem < init_brk;
        //let must_go_on = pos > 0;

        if pos == 0 && idx == 0 && rem == 1 && (spr_at_pos == S_B || spr_at_pos == S_U) {
            //if debug {println!("pos == 0 && idx == 0 && rem == 1 && (spr_at_pos == S_B || spr_at_pos == S_U)")};
            1
        } else if pos == 0 && idx == 0 && rem == 0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("pos == 0 && idx == 0 && rem == 0 && (spr_at_pos == S_W || spr_at_pos == S_U)")};
            1
        }
        //if remaining broken springs > 0 and re reached end of springs
        else if pos == 0 && (idx > 0 || rem > 0) {
            //if debug {println!("pos == 0 && (idx > 0 || rem > 0)")};
            0
        }
        //if remaining broken springs = 0 and we find B-spring, then = 0
        else if rem == 0 && spr_at_pos == S_B {
            //if debug {println!("rem == 0 && spr_at_pos == S_B")};
            0
        }
        else if pos == 0 {
            panic!("non-handled pos == 0 condition")
        }
        //if remaining broken springs > 0 and series of broken springs has not yet started remaining broken = init_broken
        //    - move on to next spring and search for brokens
        //    - move on to next spring and postpone search
        else if rem == init_brk && spr_at_pos == S_U {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_U")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1)
            + Self::rev_calculate(spr, brk, pos - 1, idx, rem)
        }
        //if remaining broken springs > 0 and we find W-spring, then = 0
        else if rem == init_brk && spr_at_pos == S_W {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_W")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem)
        }
        //if remaining broken springs > 0 and we find B-spring, then
        //    - move on
        else if rem == init_brk && spr_at_pos == S_B {
            //if debug {println!("rem > 0 && rem == init_brk && spr_at_pos == S_B")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1)
        }
        else if rem == init_brk {
            panic!("non handled rem == init_brk condition")
        }
        // if remaining broken in the index is 0 but next spring is broken then = 0
        else if rem == 0 && spr_at_pos == S_B {
            //if debug {println!("if rem == 0 && spr_at_pos == S_B")};
            0
        }
        // if remaining broken in the index is 0 but next spring is unknown or working then
        //    - move on by decreasing idx and
        else if rem == 0 && idx >  0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("rem == 0 && idx >  0 && ( spr_at_pos == S_W || spr_at_pos == S_U )")};
            Self::rev_calculate(spr, brk, pos - 1, idx - 1, brk[idx-1])
        }
        // if remaining broken = 0 and no indexes and current spring is W or U, move on to next
        else if rem == 0 && idx == 0 && (spr_at_pos == S_W || spr_at_pos == S_U) {
            //if debug {println!("rem == 0 && idx == 0 && (spr_at_pos == S_W || spr_at_pos == S_U)")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem)
        }
        else if rem == 0 {
            panic!("non handled rem == 0 condition")
        }
        //if remaining broken springs > 0 and we find W-spring, then = 0
        else if rem > 0 && spr_at_pos == S_W {
            //if debug {println!("rem > 0 && rem < init_brk && spr_at_pos == S_W")};
            0
        }
        //if remaining broken springs > 0 and we find broken or unknown spring then continue
        else if rem > 0 && (spr_at_pos == S_B || spr_at_pos == S_U) {
            //if debug {println!("pos > 0 && rem > 0 && rem != init_brk && (spr_at_pos == S_B || spr_at_pos == S_U)")};
            Self::rev_calculate(spr, brk, pos - 1, idx, rem - 1)
        }
        else if rem > 0 {
            panic!("non handled rem > 0 condition")
        } else {
            panic!(
                "Undefined case! pos: {pos}, brk_idx {idx}, rem: {rem}, pos_val: {:?}", spr_at_pos
            )
        }
    }

    fn process_one_record(record: &Record) -> usize {
        //let rev_springs: Springs = record.springs.clone().into_iter().rev().collect();
        //let rev_brokens: Vec<usize> = record.brokens.clone().into_iter().rev().collect();
        let init_pos = record.springs.len() - 1;
        let init_brk_idx = record.brokens.len() - 1;
        let rem_brk = record.brokens[init_brk_idx];

        //Self::rev_calculate(&rev_springs, &rev_brokens, init_pos, init_brk_idx, rem_brk)
        Self::rev_calculate(&record.springs, &record.brokens, init_pos, init_brk_idx, rem_brk)
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
            .enumerate()
            .map(|(idx, record)| {
                if idx % 50 == 0 {
                println!("processing part 1, record {:>3}", idx);
                };
                DaySolution::process_one_record(record)
            })
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .par_iter()
            .enumerate()
            .map(|(idx, record)| {
                let v = DaySolution::process_one_record(record);
                if idx % 1 == 0 {
                println!("processing part 2, record: {:>3}, count: {:>13}", idx, v);
                };
                v
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

#[cfg(test)]
mod tests {
    use super::{DaySolution as DS, Record as Rec, S_B, S_W, S_U};

    #[test]
    fn parse_one_line() {
        assert_eq!(DS::parse_one_line(".#? 1,1"), Rec { springs: vec![S_W, S_B, S_U], brokens: vec![1,1]})
    }
    #[test]
    fn parse_one_line_part_2() {
        assert_eq!(
            DS::parse_one_line_part_2(".# 1,2"),
            Rec {
                springs: vec![S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B, S_U, S_W, S_B],
                brokens: vec![1,2,1,2,1,2,1,2,1,2]
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
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("...# 1")),
            1
        );
    }

    #[test]
    fn process_one_heavy_record() {
        assert_eq!(
            DS::process_one_record(&DS::parse_one_line("?#.??????#??#?#?#?#? 1,1,15")),
            1_usize
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
