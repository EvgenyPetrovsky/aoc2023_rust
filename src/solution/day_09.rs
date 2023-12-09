use regex::Regex;

type Number = i64;
type Series = Vec<Number>;

type Report = Vec<Series>;
type P = Report;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Series {
        Regex::new(r#"-?\d+"#)
            .unwrap()
            .captures_iter(line)
            .map(|c| {
                let num = c.get(0).unwrap().as_str();
                num.parse::<Number>()
                    .expect(format!("Couldn't parse value '{}' into i32", num).as_str())
            })
            .collect()
    }

    // all elements of series is 0
    fn is_zero_series(series: &Series) -> bool {
        series.iter().all(|x| *x == 0)
    }
    // take last number
    fn series_last_num(series: &Series) -> Number {
        *(series.last().unwrap())
    }
    // take first number
    fn series_head_num(series: &Series) -> Number {
        series[0]
    }
    // differentiate
    fn diff_series(series: &Series) -> Series {
        series
            .iter()
            .skip(1)
            .zip(series.iter())
            .map(|(n2, n1)| n2 - n1)
            .collect()
    }
    // find next number in series
    fn find_next_number(acc: Number, series: &Series) -> Number {
        /*
        take differentiate the series
          if it has only 0 then return 3
          if it is not only 0 series then
            return last elem of original series
            plus a number given by application of this function recursively to differential
          */
        if Self::is_zero_series(&series) {
            acc
        } else {
            let last = Self::series_last_num(&series);
            let diff = Self::diff_series(&series);
            Self::find_next_number(acc + last, &diff)
        }
    }

    fn find_prev_number(acc: Number, series: &Series) -> Number {
        if Self::is_zero_series(&series) {
            acc
        } else {
            let head = Self::series_head_num(&series);
            let diff = Self::diff_series(&series);
            head - Self::find_prev_number(acc, &diff)
        }
    }
}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 9;

    type Answer = Option<Number>;
    type Problem = P;

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
        let answer =
            problem
            .iter()
            .map(|series| DaySolution::find_next_number(0, series))
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer =
            problem
            .iter()
            .map(|series| DaySolution::find_prev_number(0, series))
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
    fn parse_one_line() {
        assert_eq!(DS::parse_one_line("1 2 3 -3"), vec![1, 2, 3, -3]);
        assert_eq!(DS::parse_one_line("-21 2 23"), vec![-21, 2, 23]);
    }

    #[test]
    fn diff_series() {
        assert_eq!(
            DS::diff_series(&vec![10, 13, 16, 21, 30, 45]),
            vec![3, 3, 5, 9, 15]
        );
        assert_eq!(DS::diff_series(&vec![3, 3, 5, 9, 15]), vec![0, 2, 4, 6]);
        assert_eq!(DS::diff_series(&vec![0, 2, 4, 6]), vec![2, 2, 2]);
        assert_eq!(DS::diff_series(&vec![2, 2, 2]), vec![0, 0]);
    }

    #[test]
    fn find_next_number() {
        assert_eq!(DS::find_next_number(0, &vec![2, 2, 2]), 2);
        assert_eq!(DS::find_next_number(0, &vec![0, 2, 4, 6]), 8);
        assert_eq!(DS::find_next_number(0, &vec![3, 3, 5, 9, 15]), 23);
        assert_eq!(DS::find_next_number(0, &vec![10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(DS::find_next_number(0, &vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(DS::find_next_number(0, &vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn find_prev_number() {
        assert_eq!(DS::find_prev_number(0, &vec![2, 2, 2]), 2);
        assert_eq!(DS::find_prev_number(0, &vec![0, 2, 4, 6]), -2);
        assert_eq!(DS::find_prev_number(0, &vec![3, 3, 5, 9, 15]), 5);
        assert_eq!(DS::find_prev_number(0, &vec![10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(DS::find_prev_number(0, &vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(DS::find_prev_number(0, &vec![0, 3, 6, 9, 12, 15]), -3);
    }
}
