use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Location(usize, usize);
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Body {Void, Galaxy}
type Picture = Vec<Vec<Body>>;
type P = Picture;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Vec<Body> {
        line
        .chars()
        .map(|c| match c {'#' => Body::Galaxy, _ => Body::Void})
        .collect()
    }
    fn galaxy_location(picture: &Picture) -> Vec<Location> {
        picture
        .iter()
        .enumerate()
        .flat_map(move |(r_idx, r)| {
            r.iter().enumerate().filter_map(move |(c_idx, c)| {
                match c {Body::Galaxy => Some(Location(r_idx, c_idx)), _ => None}
            })
        })
        .collect()
    }
    fn distance(l1: &Location, l2: &Location) -> usize {
        // all that bla-bla-bla about shortest path is just about manhattan distance
        l2.0.abs_diff(l1.0) + l2.1.abs_diff(l1.1)
    }

    fn find_empty_rows(picture: &Picture) -> Vec<usize> {
        picture
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            match row.iter().all(|body| body == &Body::Void) {
                true => Some(idx),
                false => None
            }
        })
        .collect()
    }
    // number of rows & columns
    fn dims(picture: &Picture) -> (usize, usize) {
        (picture.len(), picture[0].len())
    }

    fn transpose(picture: Picture) -> Picture {
        let (rows, cols) = Self::dims(&picture);
        (0..cols)
        .map(|c| {
            (0..rows).map(|r| picture[r][c]).collect::<Vec<Body>>()
        }).collect()
    }
    fn find_empty_cols(picture: &Picture) -> Vec<usize> {
        let transposed = Self::transpose(picture.clone());
        Self::find_empty_rows(&transposed)
    }

    fn calculate_shift(range_size: usize, empty_items: &Vec<usize>, multiplier: usize) -> Vec<usize> {
        (0..range_size)
        .scan(0, |add_shift, x| {
            if empty_items.contains(&x) {
                *add_shift += multiplier;
                Some(*add_shift)
            } else {
                Some(*add_shift)
            }
        })
        .collect()
    }

    fn solve(problem: P, multiplier: usize) -> Option<usize> {
        let picture = problem;
        let (n_rows, n_cols) = DaySolution::dims(&picture);
        let empty_rows = DaySolution::find_empty_rows(&picture);
        let empty_cols = DaySolution::find_empty_cols(&picture);
        let shift_rows = DaySolution::calculate_shift(n_rows, &empty_rows, multiplier);
        let shift_cols = DaySolution::calculate_shift(n_cols, &empty_cols, multiplier);
        let galaxy_loc = DaySolution::galaxy_location(&picture);
        let galaxy_loc_corrected: Vec<Location> =
            galaxy_loc
            .iter()
            .map(|Location(r, c)| {
                Location(r + &shift_rows[*r], c + &shift_cols[*c])
            })
            .collect();
        let answer =
            galaxy_loc_corrected
            .iter()
            .flat_map(|l1| {
                galaxy_loc_corrected
                .iter()
                .map(|l2| (*l1, *l2))
                .collect::<Vec<(Location, Location)>>()
            })
            .filter(|(l1, l2)| l1.cmp(l2) == Ordering::Less)
            .map(|(l1, l2)| DaySolution::distance(&l1, &l2))
            .sum();
        Some(answer)

    }

}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 11;

    type Answer = Option<usize>;
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
        DaySolution::solve(problem, 1)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        DaySolution::solve(problem, 1_000_000-1)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}
