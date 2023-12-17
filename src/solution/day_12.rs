use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum SpringState {
    Working,
    Broken,
    Unknown,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    springs: Vec<SpringState>,
    brokens: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum S {
    W,
    B,
    U,
}

type Springs = Vec<S>;
struct Record {
    springs: Vec<S>,
    record: Vec<u8>,
}

type P = Vec<Record>;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Record {
        let springs: Vec<SpringState> = Regex::new(r#"[?#\.]+"#)
            .unwrap()
            .captures(line)
            .map(|c| c.get(0).unwrap().as_str())
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => SpringState::Working,
                '#' => SpringState::Broken,
                '?' => SpringState::Unknown,
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

    fn fast_permutations(broken: usize, total: usize) -> Vec<Vec<SpringState>> {
        fn iter_fun(broken: usize, total: usize) -> Vec<Vec<SpringState>> {
            if total > 0 {
                let mut v1: Vec<Vec<SpringState>> = vec![vec![]];
                let mut v2: Vec<Vec<SpringState>> = vec![vec![]];
                if broken > 0 {
                    v1 = iter_fun(broken - 1, total - 1);
                    for idx in 0..v1.len() {
                        v1[idx].push(SpringState::Broken);
                    }
                } else {
                    v1.clear();
                };
                if total - broken > 0 {
                    v2 = iter_fun(broken, total - 1);
                    for idx in 0..v2.len() {
                        v2[idx].push(SpringState::Working);
                    }
                } else {
                    v2.clear()
                }
                let v: Vec<Vec<SpringState>> =
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

    fn _slow_permutations(num: usize, total: usize) -> Vec<Vec<SpringState>> {
        let springs: Vec<SpringState> = (0..total)
            .map(|x| {
                if x < num {
                    SpringState::Broken
                } else {
                    SpringState::Working
                }
            })
            .collect();

        springs
            .iter()
            .permutations(springs.len())
            .unique()
            .map(|v| v.iter().map(|&&s| s).collect::<Vec<SpringState>>())
            .collect()
    }

    fn enumerate_unknowns(springs: &Vec<SpringState>) -> Vec<Option<usize>> {
        springs
            .iter()
            .map(|&s| s == SpringState::Unknown)
            .scan((None::<Option<SpringState>>, 0_usize), |(_, n), x| {
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
        springs: &Vec<SpringState>,
        replacement_springs: &Vec<SpringState>,
    ) -> Vec<SpringState> {
        let idx_unknowns = Self::enumerate_unknowns(springs);
        springs
            .iter()
            .enumerate()
            .map(|(idx, s)| match s {
                SpringState::Unknown => replacement_springs[idx_unknowns[idx].unwrap()],
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

    fn process_one_record(record: &Record) -> usize {
        let springs = &(record.springs);
        let unknown_count: usize = springs
            .iter()
            .filter(|&&s| s == SpringState::Unknown)
            .count();
        let known_broken_count: usize = springs
            .iter()
            .filter(|&&s| s == SpringState::Broken)
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
                    .map(|&s| if s == SpringState::Broken { 'B' } else { 'W' })
                    .collect::<String>()
            })
            .filter(|s| re.is_match(s))
            .count()
    }

    // use all reversed values to easier reason about the end (0)
    fn rev_calculate(
        rev_springs: &Springs,
        rev_records: &Vec<u8>,
        rev_pos: usize,
        rev_rec_idx: usize,
        rem_brk: u8,
    ) -> usize {
        let (spr, pos, rec, idx, rem) = (rev_springs, rev_pos, rev_records, rev_rec_idx, rem_brk);

        //if remaining broken springs = 0 and rev_rec_idx = 0 and we reached the end of springs, then = 1
        if pos == 0 && idx == 0 && rem == 1 && (spr[pos] == S::B || spr[pos] == S::U) {
            1
        } else if pos == 0 && idx == 0 && rem == 0 && (spr[pos] == S::W || spr[pos] == S::U) {
            1
        }
        //if remaining broken springs > 0 and re reached end of springs
        else if pos == 0 && (idx > 0 || rem > 0) {
            0
        }
        //if remaining broken springs = 0 and we find B-spring, then = 0
        else if rem == 0 && spr[pos] == S::B {
            0
        }
        //if remaining broken springs > 0 and we find W-spring, then = 0
        else if rem > 0 && spr[pos] == S::W {
            0
        }
        //if remaining broken springs > 0 and we find broken or unknown spring then continue
        else if pos > 0 && rem > 0 && (spr[pos] == S::B || spr[pos] == S::U) {
            Self::rev_calculate(spr, rec, pos - 1, idx, rem - 1)
        }
        //if remaining broken springs = 0 and rev_rec_idx = 0 and we didn't reach the end of spings, then
        //    - move on to next spring and search for brokens
        //    - move on to next spring and postpone search
        else if pos > 0 && idx > 0 && rem == 0 && (spr[pos] == S::W || spr[pos] == S::U) {
            Self::rev_calculate(spr, rec, pos - 1, idx, rem)
                + Self::rev_calculate(spr, rec, pos - 1, idx - 1, rec[idx - 1])
        }
        // if remaining broken springs = 0 and we find working or unknown spring then continue
        else if pos > 0 && idx == 0 && rem == 0 && (spr[pos] == S::W || spr[pos] == S::U) {
            Self::rev_calculate(spr, rec, pos - 1, idx, rem)
        } else {
            panic!(
                "rev_pos: {pos}, rev_rec_idx {idx}, rem_brk: {rem}, pos_val: {:?}",
                spr[pos]
            )
        }
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
        Self::parse_input_part_1(text_input)
            .iter()
            .map(|r| {
                let s: Vec<SpringState> =
                    repeat_n(r.springs.clone(), 5)
                    .intersperse(vec![SpringState::Unknown])
                    .flatten()
                    .collect();
                let b: Vec<usize> =
                    repeat_n(r.brokens.clone(), 5)
                    .flatten()
                    .collect();

                Record { springs: s, brokens: b }
            })
            .collect()
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .enumerate()
            .map(|(idx, record)| {
                //if idx % 1 == 0 {
                    println!("processing part 1, record {:>3}", idx);
                //};
                DaySolution::process_one_record(record)
            })
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        Self::solve_part_1(problem)
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
    use super::{DaySolution as DS, Record, SpringState as SS};
    #[test]
    fn generate_brokens_regexp() {
        assert_eq!(
            DS::generate_brokens_regexp(&Record {
                springs: vec![],
                brokens: vec![3]
            }),
            String::from("BBB")
        );
        assert_eq!(
            DS::generate_brokens_regexp(&Record {
                springs: vec![],
                brokens: vec![1, 1]
            }),
            String::from("BW+B")
        );
        assert_eq!(
            DS::generate_brokens_regexp(&Record {
                springs: vec![],
                brokens: vec![1, 2, 1]
            }),
            String::from("BW+BBW+B")
        );
    }
    #[test]
    fn parse_one_line() {
        assert_eq!(
            DS::parse_one_line("#.#.### 1,1,3"),
            Record {
                springs: vec![
                    SS::Broken,
                    SS::Working,
                    SS::Broken,
                    SS::Working,
                    SS::Broken,
                    SS::Broken,
                    SS::Broken
                ],
                brokens: vec![1, 1, 3]
            }
        );
        assert_eq!(
            DS::parse_one_line("?#? 1,3,1,6"),
            Record {
                springs: vec![SS::Unknown, SS::Broken, SS::Unknown],
                brokens: vec![1, 3, 1, 6]
            }
        );
    }
    #[test]
    fn substitute_unknowns() {
        assert_eq!(
            DS::substitute_unknowns(
                &vec![SS::Working, SS::Broken, SS::Unknown, SS::Unknown],
                &vec![SS::Broken, SS::Working]
            ),
            vec![SS::Working, SS::Broken, SS::Broken, SS::Working]
        );
    }
    #[test]
    fn fast_permutations() {
        //assert_eq!(DS::fast_permutations(0, 1).len(), 1);
        //assert_eq!(DS::fast_permutations(0, 2).len(), 1);
        //assert_eq!(DS::fast_permutations(1, 2).len(), 2);
        //assert_eq!(DS::fast_permutations(2, 2).len(), 1);
        assert_eq!(DS::fast_permutations(1, 3).len(), 3);
        assert_eq!(DS::fast_permutations(2, 3).len(), 3);
        assert_eq!(DS::fast_permutations(2, 4).len(), 6);
        assert_eq!(DS::fast_permutations(11, 17).len(), 12376);
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
