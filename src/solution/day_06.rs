use regex::Regex;

use crate::Part;

type Time = u64;
type Timespan = u64;
type Distance = u64;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Race {
    time: Time,
    record: Distance,
}
pub type P = Vec<Race>;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_input(part: Part, text_input: String) -> P {
        let time_text_ = Regex::new(r#"Time:([ \d]+)"#)
            .unwrap()
            .captures(&text_input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let time_text = match part {
            Part::One => String::from(time_text_),
            Part::Two => time_text_.split_whitespace().collect::<String>(),
        };

        let times: Vec<Time> = Regex::new(r#"\d+"#)
            .unwrap()
            .captures_iter(&time_text)
            .map(|c| c.get(0).unwrap().as_str().parse::<Time>().unwrap())
            .collect();

        let dist_text_ = Regex::new(r#"Distance:([ \d]+)"#)
            .unwrap()
            .captures(&text_input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let dist_text = match part {
            Part::One => String::from(dist_text_),
            Part::Two => dist_text_.split_whitespace().collect::<String>(),
        };

        let dists: Vec<Distance> = Regex::new(r#"\d+"#)
            .unwrap()
            .captures_iter(&dist_text)
            .map(|c| c.get(0).unwrap().as_str().parse::<Time>().unwrap())
            .collect();

        times
            .iter()
            .zip(dists.iter())
            .map(|(&time, &record)| Race { time, record })
            .collect()
    }

    fn _analytic_distance(race_time: &Time, press_time: &Time) -> Distance {
        press_time * (race_time - press_time)
    }
    fn _new_record_time_span(race: Race) -> Timespan {
        let (race_time, record) = (race.time, race.record);
        (0..=race_time)
            .map(|press_time| Self::_analytic_distance(&race_time, &press_time))
            .filter(|d| *d > record)
            .count() as Timespan
    }
    fn _analytic_solution(race: Race) -> Timespan {
        // r < x * (t - x)
        // r < t*x - x^2
        // x^2 - t*x + r < 0
        // x^2 - t*x + r
        let a: f64 = 1.;
        let b: f64 = -(race.time as f64);
        let c: f64 = race.record as f64;
        let d: f64 = ((b.powf(2.) - 4. * a * c) as f64).powf(0.5);
        let x1 = (- b - d) / (2. * a);
        let x2 = (- b + d) / (2. * a);
        let (x1_c, x2_f) = (x1.ceil(), x2.floor());
        ((if x2 == x2_f {x2_f-1.} else {x2_f}) as u64) + 1 - ((if x1 == x1_c {x1_c + 1.} else {x1_c}) as u64)
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 6;

    type Answer = Option<u64>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        DaySolution::parse_input(Part::One, text_input)
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        DaySolution::parse_input(Part::Two, text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|&race| DaySolution::_new_record_time_span(race))
            .product();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|&race| DaySolution::_new_record_time_span(race))
            .product();
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

    //use super::{Room, DaySolution};

    use crate::solution::day_06::Timespan;

    use super::{DaySolution, Race, Part};

    #[test]
    fn parse_input() {
        let line = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(
            DaySolution::parse_input(Part::One, String::from(line)),
            vec![
                Race { time: 7, record: 9 },
                Race {
                    time: 15,
                    record: 40
                },
                Race {
                    time: 30,
                    record: 200
                },
            ]
        );
        assert_eq!(
            DaySolution::parse_input(Part::Two, String::from(line)),
            vec![
                Race { time: 71530, record: 940200 }
            ]
        )
    }

    #[test]
    fn _analytic_solution() {
        assert_eq!(
            DaySolution::_analytic_solution(Race{ time: 7, record: 9}),
            4 as Timespan
        );
        assert_eq!(
            DaySolution::_analytic_solution(Race{ time: 15, record: 40}),
            8 as Timespan
        );
        assert_eq!(
            DaySolution::_analytic_solution(Race{ time: 30, record: 200}),
            9 as Timespan
        );
    }
}
