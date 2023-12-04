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
        let mut card_count: CardCount = HashMap::from_iter((1..(problem.len()+1)).map(|x| (x as u32, 1_u32)));//new();
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

/*
--- Part Two ---

Just as you're about to report your findings to the Elf, one of you realizes that the rules have actually been printed on the back of every card this whole time.

There's no such thing as "points". Instead, scratchcards only cause you to win more scratchcards equal to the number of winning numbers you have.

Specifically, you win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Copies of scratchcards are scored like normal scratchcards and have the same card number as the card they copied. So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that the original card 10 won: cards 11, 12, 13, 14, and 15. This process repeats until none of the copies cause you to win any more cards. (Cards will never make you copy a card past the end of the table.)

This time, the above example goes differently:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

    Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
    Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
    Your copy of card 2 also wins one copy each of cards 3 and 4.
    Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
    Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
    Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
    Your one instance of card 6 (one original) has no matching numbers and wins no more cards.

Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!

Process all of the original and copied scratchcards until no more scratchcards are won. Including the original set of scratchcards, how many total scratchcards do you end up with?

Answer:

Although it hasn't changed, you can still get your puzzle input.

You can also [Shareon Twitter Mastodon] this puzzle.
*/