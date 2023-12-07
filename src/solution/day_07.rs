use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

type Label = char;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TrheeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

type Bid = u32;
pub struct Hand {
    cards: String,
    hand_type: HandType,
    bid: Bid,
}

type P = Vec<Hand>;

pub struct DaySolution(P);

impl DaySolution {
    // hand type: collect cards into hashmap and count, take values, sort,
    // compare against patterns: [5] [4,1], [3,2], [3,1,1], [2,2,1], [2,1,1,1], [1,1,1,1,1]
    fn hand_type(cards: &str) -> HandType {
        let init: HashMap<char, u8> = HashMap::new();
        let mut counts: Vec<u8> = cards
            .chars()
            .fold(init, |mut z, c| {
                let _ = z.entry(c).and_modify(|v| *v += 1).or_insert(1);
                z
            })
            .values()
            .map(|v| *v)
            .collect();
        counts.sort();
        if counts == vec![5] {
            HandType::FiveOfAKind
        } else if counts == vec![1, 4] {
            HandType::FourOfAKind
        } else if counts == vec![2, 3] {
            HandType::FullHouse
        } else if counts == vec![1, 1, 3] {
            HandType::TrheeOfAKind
        } else if counts == vec![1, 2, 2] {
            HandType::TwoPair
        } else if counts == vec![1, 1, 1, 2] {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    // parse one line
    fn parse_one_line(line: &str) -> Hand {
        Regex::new(r#"^([AKQJT2-9]+) (\d+)$"#)
            .unwrap()
            .captures(line)
            .map(|c| {
                let cards: String = String::from(c.get(1).unwrap().as_str());
                let hand_type = Self::hand_type(&cards);
                let bid: u32 = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                Hand {
                    cards,
                    hand_type,
                    bid,
                }
            })
            .unwrap()
    }

    // convert label into number that can be compared
    fn label_rank(label: Label) -> u32 {
        match label {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
            _ => panic!("unknown label {label}"),
        }
    }

    // hand type rank
    fn hand_type_rank(hand_type: HandType) -> u32 {
        match hand_type {
            HandType::FiveOfAKind => 0,
            HandType::FourOfAKind => 1,
            HandType::FullHouse => 2,
            HandType::TrheeOfAKind => 3,
            HandType::TwoPair => 4,
            HandType::OnePair => 5,
            HandType::HighCard => 6,
        }
    }

    // how much can hand win
    fn hand_win_size(strength: u32, hand: &Hand) -> u32 {
        strength * hand.bid
    }

    // convert label into number that can be compared
    fn label_rank_part_2(label: Label) -> u32 {
        match label {
            'J' => 99,
            _ => Self::label_rank(label),
        }
    }
    // count number of jokers in hand
    fn number_of_jockers(hand: &Hand) -> u32 {
        hand.cards.chars().filter(|c| *c == 'J').count() as u32
    }

    // hand type rank for part 2
    fn hand_type_rank_part_2(hand_type: HandType, number_of_jockers: u32) -> u32 {
        let n = number_of_jockers;
        match hand_type {
            HandType::FiveOfAKind => 0,
            // 1 or 4 jockers will make it five of a kind
            HandType::FourOfAKind => {
                if n == 4 || n == 1 {
                    0
                } else {
                    1
                }
            }
            // 2 or 3 jockets will make it five of a kind
            HandType::FullHouse => {
                if n == 3 || n == 2 {
                    0
                } else {
                    2
                }
            }
            // 1 or 3 jockers will make it four of a kind
            HandType::TrheeOfAKind => {
                if n == 3 || n == 1 {
                    1
                } else {
                    3
                }
            }
            // 1 jocker will make it full house, 2 jockets will make it four of a kind
            HandType::TwoPair => {
                if n == 2 {
                    1
                } else if n == 1 {
                    2
                } else {
                    4
                }
            }
            // 1 or 2 jockers will make it three of a kind
            HandType::OnePair => {
                if n == 1 || n == 2 {
                    3
                } else {
                    5
                }
            }
            // 1 jocker will make it one pair
            HandType::HighCard => {
                if n == 1 {
                    5
                } else {
                    6
                }
            }
        }
    }

    // compare hands to give them order
    fn compare_hands(hand_1: &Hand, hand_2: &Hand, jockers: bool) -> Ordering {
        let hr1 = if jockers {
            let number_of_jockers = Self::number_of_jockers(hand_1);
            Self::hand_type_rank_part_2(hand_1.hand_type.clone(), number_of_jockers)
        } else {
            Self::hand_type_rank(hand_1.hand_type.clone())
        };
        let hr2 = if jockers {
            let number_of_jockers = Self::number_of_jockers(hand_2);
            Self::hand_type_rank_part_2(hand_2.hand_type.clone(), number_of_jockers)
        } else {
            Self::hand_type_rank(hand_2.hand_type.clone())
        };
        let cmp1 = hr1.cmp(&hr2);
        match cmp1 {
            Ordering::Equal => {
                let labels1 = hand_1.cards.chars();
                let labels2 = hand_2.cards.chars();
                labels1
                    .zip(labels2)
                    .fold(Ordering::Equal, |z, (l1, l2)| match z {
                        Ordering::Equal => {
                            let lr1 = if jockers {
                                Self::label_rank_part_2(l1)
                            } else {
                                Self::label_rank(l1)
                            };
                            let lr2 = if jockers {
                                Self::label_rank_part_2(l2)
                            } else {
                                Self::label_rank(l2)
                            };
                            lr1.cmp(&lr2)
                        }
                        _ => z,
                    })
            }
            _ => cmp1,
        }
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 7;

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
        let mut hands = problem;
        hands.sort_by(|hand_1, hand_2| DaySolution::compare_hands(hand_2, hand_1, false));
        let answer = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| DaySolution::hand_win_size(idx as u32 + 1, hand))
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let mut hands = problem;
        hands.sort_by(|hand_1, hand_2| DaySolution::compare_hands(hand_2, hand_1, true));
        let answer = hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| DaySolution::hand_win_size(idx as u32 + 1, hand))
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
    use super::{DaySolution, HandType};
    use std::cmp::Ordering;

    #[test]
    fn type_order() {
        assert_eq!(
            HandType::TrheeOfAKind.cmp(&HandType::HighCard),
            Ordering::Less
        );
    }

    #[test]
    fn hand_rank() {
        assert_eq!(DaySolution::hand_type_rank(HandType::FiveOfAKind), 0);
        assert_eq!(DaySolution::hand_type_rank(HandType::TrheeOfAKind), 3);
        assert_eq!(DaySolution::hand_type_rank(HandType::HighCard), 6);
    }

    #[test]
    fn hand_type() {
        assert_eq!(DaySolution::hand_type("KKKKK"), HandType::FiveOfAKind);
        assert_eq!(DaySolution::hand_type("KQKKK"), HandType::FourOfAKind);
        assert_eq!(DaySolution::hand_type("32323"), HandType::FullHouse);
        assert_eq!(DaySolution::hand_type("T55J5"), HandType::TrheeOfAKind);
        assert_eq!(DaySolution::hand_type("AA3TT"), HandType::TwoPair);
        assert_eq!(DaySolution::hand_type("32T3K"), HandType::OnePair);
        assert_eq!(DaySolution::hand_type("AKQJT"), HandType::HighCard);
    }
}
