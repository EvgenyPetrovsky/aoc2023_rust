use regex::Regex;
type Pattern = Vec<Vec<u8>>;

type P = Vec<Pattern>;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Vec<u8> {
        line.as_bytes().to_vec()
    }

    fn transpose(mx: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let rows = mx.len();
        let cols = mx[0].len();
        (0..cols)
            .map(|c| (0..rows).map(|r| mx[r][c]).collect())
            .collect()
    }

    fn is_perfect_reflection(pattern: &Pattern, line: usize) -> bool {
        let (a, b) = pattern.split_at(line);

        a.iter()
            .rev()
            .zip(b.iter())
            .all(|(aa, bb)| aa.iter().zip(bb.iter()).all(|(l, r)| *l == *r))
    }

    fn has_one_smudge(pattern: &Pattern, line: usize) -> bool {
        let (a, b) = pattern.split_at(line);
        let cnt_smudges: usize = a
            .iter()
            .rev()
            .zip(b.iter())
            .map(|(aa, bb)| {
                aa.iter().zip(bb.iter())
                .filter(|(l, r)| *l != *r)
                .count()
            })
            .sum();
        cnt_smudges == 1
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 13;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let re_pattern = Regex::new(r#"([.#]+(\n[.#]+)+)"#).unwrap();
        re_pattern
            .captures_iter(&text_input)
            .map(|c| {
                c.get(0)
                    .unwrap()
                    .as_str()
                    .lines()
                    .map(|line| DaySolution::parse_one_line(line))
                    .collect()
            })
            .collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|pattern| -> usize {
                let rows = pattern.len();
                let r: usize = (1..rows)
                    .filter(|line| DaySolution::is_perfect_reflection(pattern, *line))
                    .sum();
                let pattern: Vec<Vec<u8>> = DaySolution::transpose(pattern);
                let cols = pattern.len();
                let c: usize = (1..cols)
                    .filter(|line| DaySolution::is_perfect_reflection(&pattern, *line))
                    .sum();
                r * 100 + c
            })
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|pattern| -> usize {
                let rows = pattern.len();
                let r: usize = (1..rows)
                    .filter(|line| DaySolution::has_one_smudge(pattern, *line))
                    .sum();
                let pattern: Vec<Vec<u8>> = DaySolution::transpose(pattern);
                let cols = pattern.len();
                let c: usize = (1..cols)
                    .filter(|line| DaySolution::has_one_smudge(&pattern, *line))
                    .sum();
                r * 100 + c
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
    use super::DaySolution as DS;
    #[test]
    fn transpose() {
        assert_eq!(
            DS::transpose(&vec![vec![1_u8, 2_u8], vec![3_u8, 4_u8]]),
            vec![vec![1_u8, 3_u8], vec![2_u8, 4_u8]]
        );
        assert_eq!(
            DS::transpose(&vec![vec![1_u8, 2, 3], vec![3_u8, 4, 5]]),
            vec![vec![1_u8, 3], vec![2_u8, 4], vec![3_u8, 5]]
        );
    }

    #[test]
    fn is_perfect_reflection() {
        let v: Vec<Vec<u8>> = vec![
            vec![1, 2, 3, 3, 2],
            vec![5, 4, 3, 3, 4],
            vec![1, 0, 1, 1, 0],
        ];
        assert_eq!(DS::is_perfect_reflection(&v, 1), false);
        assert_eq!(DS::is_perfect_reflection(&v, 2), false);

        let t: Vec<Vec<u8>> = vec![
            vec![1, 5, 1],
            vec![2, 4, 0],
            vec![3, 3, 1],
            vec![3, 3, 1],
            vec![2, 4, 0],
        ];
        //let t = DS::transpose(&v);
        assert_eq!(DS::is_perfect_reflection(&t, 1), false);
        assert_eq!(DS::is_perfect_reflection(&t, 2), false);
        assert_eq!(DS::is_perfect_reflection(&t, 3), true);
        //assert_eq!(DS::is_perfect_reflection(&t, 4), false);
    }
}
