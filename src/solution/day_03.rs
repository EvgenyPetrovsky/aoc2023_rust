use std::collections::HashSet;

use regex::Regex;

type Row = usize;
type Col = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(Row, Col);
#[derive(Clone, Copy, PartialEq)]
enum Data { None , Num , GearSym , OtherSym }

type P = Vec<Vec<char>>;
pub struct DaySolution(P);

impl DaySolution {

    fn char_to_data(c: char) -> Data {
        if c == '.' {Data::None}
        else if c.is_digit(10) {Data::Num}
        else if c == '*' {Data::GearSym}
        else {Data::OtherSym}
    }
    fn data_matrix(vec_vec_c: &Vec<Vec<char>>) -> Vec<Vec<Data>> {
        vec_vec_c
        .iter()
        .map(|vec_c| vec_c.iter().map(|c| Self::char_to_data(*c)).collect::<Vec<Data>>())
        .collect()
    }
    fn symbol_positions(dmx: &Vec<Vec<Data>>) -> Vec<Position> {
        let(rs, cs) = (dmx.len(), dmx[0].len());
        (0..rs*cs).filter_map(|i| {
            let r = i.clone() / cs;
            let c = i.clone() % cs;
            match dmx[r][c] {
                Data::GearSym | Data::OtherSym => Some(Position(r, c)),
                _ => None
            }
        })
        .collect()
    }

    fn gear_positions(dmx: &Vec<Vec<Data>>) -> Vec<Position> {
        let(rs, cs) = (dmx.len(), dmx[0].len());
        (0..rs*cs).filter_map(|i| {
            let r = i.clone() / cs;
            let c = i.clone() % cs;
            match dmx[r][c] {
                Data::GearSym => Some(Position(r, c)),
                _ => None
            }
        })
        .collect()
    }

    fn data_in_position(p: &Position, dmx: &Vec<Vec<Data>>) -> Data {
        let Position(r, c) = p;
        dmx[*r][*c]
    }
    fn adjacent_positions(p: &Position, dims: (Row, Col), v: bool, h: bool) -> Vec<Position> {
        let (rows, cols) = dims;
        let (rows, cols) = (rows as i32, cols as i32);
        let &Position(r, c) = p;
        let (r, c) = (r as i32, c as i32);
        [(r-1, c-1), (r-1, c+0), (r-1, c+1), (r+0, c-1), (r+0, c+0), (r+0, c+1), (r+1, c-1), (r+1, c+0), (r+1, c+1)]
        .iter()
        .filter(|(r1, __)| (*r1 >= 0 && *r1 < rows && v || *r1 == r))
        .filter(|(__, c1)| (*c1 >= 0 && *c1 < cols && h || *c1 == c))
        .filter(|(r1, c1)| !(*r1 == r && *c1 == c))
        .map(|(r1,c1)| Position(*r1 as usize, *c1 as usize))
        .collect()
    }


    fn numbers_adj_to_symbol_position(ps: &Vec<Position>, dmx: &Vec<Vec<Data>>) -> HashSet<Position> {
        let dims = (dmx.len(), dmx[0].len());
        let mut num_positions: HashSet<Position> =
            ps
            .iter()
            .flat_map(|p| Self::adjacent_positions(p, dims, true, true))
            .filter(|p| {Self::data_in_position(p, &dmx) == Data::Num})
            .collect();
        //start.iter().for_each(|p| {seen.insert(p.clone());});
        //let mut count: usize = 0;
        let mut new_count = num_positions.len();

        while new_count > 0 {
            let new_positions: HashSet<Position> =
                num_positions
                .iter()
                .flat_map(|p| Self::adjacent_positions(p, dims, false, true))
                .filter(|p| {Self::data_in_position(p, &dmx) == Data::Num})
                .collect();
            new_count = new_positions.difference(&num_positions).collect::<HashSet<&Position>>().len();
            num_positions = num_positions.union(&new_positions).map(|&p| p).collect();
        }

        num_positions

    }

}

impl super::Solution for DaySolution {


    type Answer = Option<u32>;
    type Problem = P;

    const DAY_NUMBER: u8 = 3;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {

        /*
        the plan
        1. convert given data into Symbol or Number or Nothing
        2. find all positions of symbols
        3. find all adjacent positions of symbols
        4. for those starting positions expand selectons left and right to get complete coverage for numbers
        5. use this mask to highlight numbers on initial map and shadow all other positions
        */

        let dmx = DaySolution::data_matrix(&problem);
        let sym_positions = DaySolution::symbol_positions(&dmx);
        let num_positions = DaySolution::numbers_adj_to_symbol_position(&sym_positions, &dmx);

        let only_numbers = problem
            .iter().enumerate()
            .map(|(r_idx, r)|
                r.iter().enumerate().map(|(c_idx, c)| {
                    if num_positions.contains(&Position(r_idx, c_idx)) {*c} else {' '}
                }).collect::<String>()
            )
            .collect::<Vec<String>>();
        let re = Regex::new(r#"\d+"#).unwrap();
        let answer =
            only_numbers
            .iter()
            .flat_map(|line| {
                re
                .captures_iter(line)
                .map(|c| c.get(0).unwrap().as_str().parse::<u32>().unwrap())
            })
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {

        /*
        Solution is similar to part one, but instead of finding numbers adjacent to all symbols
        we are finding numbers gear by gear and multiplying them only if there are exactly 2 of them
        */

        let dmx = DaySolution::data_matrix(&problem);
        let gear_positions = DaySolution::gear_positions(&dmx);
        let num_positions_by_gear =
            gear_positions
            .iter()
            .map(|p|
                DaySolution::numbers_adj_to_symbol_position(&vec![*p], &dmx)
            )
            .collect::<Vec<HashSet<Position>>>();

        let re = Regex::new(r#"\d+"#).unwrap();

        let answer =
            num_positions_by_gear
            .iter()
            .map(|num_positions| {
                let only_numbers =
                    problem
                    .iter().enumerate()
                    .map(|(r_idx, r)|
                        r.iter().enumerate().map(|(c_idx, c)| {
                            if num_positions.contains(&Position(r_idx, c_idx)) {*c} else {' '}
                        }).collect::<String>()
                    )
                    .collect::<Vec<String>>();
                let numbers = only_numbers
                    .iter()
                    .flat_map(|line| {
                        re
                        .captures_iter(line)
                        .map(|c| c.get(0).unwrap().as_str().parse::<u32>().unwrap())
                    });

                numbers.collect::<Vec<u32>>()
            })
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums.iter().fold(1 as u32, |z, x| z * x))
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
