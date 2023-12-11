use std::collections::{HashSet, HashMap};
use regex::Regex;

type NumSet = HashSet<u32>;
type CardCount = HashMap<u32, u32>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {no: u32, numbers_win: NumSet, numbers_have: NumSet}

pub type P = Vec<Card>;

pub struct DaySolution(P);

impl DaySolution {
    fn line_of_num_to_set(line_of_num: &str) -> NumSet {
        Regex::new(r#"\d+"#)
        .unwrap()
        .captures_iter(line_of_num)
        .map(|c| c.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect()
    }
    fn parse_one_line(line: &str) -> Card {
        let re = Regex::new(r#"Card +(\d+): ([ \d]+)\|([ \d]+)"#).unwrap();
        re
        .captures(line)
        .map(|c| {
            Card{
                no:           c.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                numbers_win:  Self::line_of_num_to_set(c.get(2).unwrap().as_str()),
                numbers_have: Self::line_of_num_to_set(c.get(3).unwrap().as_str()),
            }
        })
        .expect(format!("Could not unwrap values out of input '{}'", line).as_str())
    }
}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 4;

    type Answer = Option<u32>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
        .lines()
        .map(DaySolution::parse_one_line)
        .collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|g| g.numbers_have.intersection(&g.numbers_win).count() as u32)
            .map(|x| if x == 0 {0_u32} else {2_u32.pow(x-1)})
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let mut card_count: CardCount = (1..(problem.len()+1)).map(|x| (x as u32, 1_u32)).collect();
        problem
        .iter()
        .for_each(|x| {
            let num = x.numbers_have.intersection(&x.numbers_win).count() as u32;
            let cnt = card_count[&x.no];
            (1..num+1).for_each(|c| {
                card_count
                .entry(&x.no + c)
                .and_modify(|v| {*v += cnt} );
            });
        });
        let answer: u32 =
            card_count
            .into_values()
            .sum();

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

    use std::collections::HashSet;

    use super::{DaySolution, Card};

    #[test]
    fn line_of_num_to_set() {
        assert_eq!(
            DaySolution::line_of_num_to_set("  1 12"),
            HashSet::from([1_u32, 12])
        );
        assert_eq!(
            DaySolution::line_of_num_to_set("10  9 22 01"),
            HashSet::from([1_u32, 9, 10, 22])
        );
    }
    #[test]
    fn parse_one_line() {
        let line = "Card 1:  1 12 | 10  9 22 01";
        //BallSet { red: 4, green: 0, blue: 3 },
        //BallSet { red: 1, green: 2, blue: 6 },
        //BallSet { red: 0, green: 2, blue: 0 }
        assert_eq!(
            DaySolution::parse_one_line(line),
            Card {
                no          : 1,
                numbers_win : HashSet::from([1_u32, 12]),
                numbers_have: HashSet::from([1_u32, 9, 10, 22]),
            }
        )
    }
}
