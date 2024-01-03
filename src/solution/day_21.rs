use std::collections::HashSet;
use super::day_09;

enum Tile {
    Plot,
    Rock,
    Start,
}

type Location = (i32, i32);

pub struct Garden {
    plan: Vec<Vec<Tile>>,
    size: Location,
    infinite: bool,
}

type P = Garden;

impl Garden {
    fn start(&self) -> Location {
        self.plan
            .iter()
            .enumerate()
            .flat_map(|(ridx, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(cidx, tile)| match tile {
                        Tile::Start => Some((ridx as i32, cidx as i32)),
                        _ => None,
                    })
            })
            .nth(0)
            .unwrap()
    }

    // only plots, no rocks no other location outside garden
    fn adj_plots(&self, location: &Location) -> Vec<Location> {
        let inf = self.infinite;
        let (rows, cols) = (self.size.0 as i32, self.size.1 as i32);
        let (r, c) = (location.0 as i32, location.1 as i32);
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|(r, c)| inf || 0 <= *r && *r < rows && 0 <= *c && *c < cols)
            .filter(|(r, c)| {
                let r_mod = ((r % rows + rows) % rows) as usize;
                let c_mod = ((c % cols + cols) % cols) as usize;
                match self.plan[r_mod][c_mod] {
                    Tile::Plot => true,
                    Tile::Start => true,
                    _ => false,
                }})
            .collect()
    }
}

impl DaySolution {
    fn parse_one_line(line: &str) -> Vec<Tile> {
        line.chars()
            .map(|c| match c {
                '.' => Tile::Plot,
                'S' => Tile::Start,
                '#' => Tile::Rock,
                _ => unreachable!()
            })
            .collect()
    }
}

pub struct DaySolution(P);

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 21;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let tiles: Vec<Vec<Tile>> = text_input
            .lines()
            .map(DaySolution::parse_one_line)
            .collect();
        let (rows, cols) = if tiles.len() == 0 {
            (0_i32, 0_i32)
        } else {
            (tiles.len() as i32, tiles[0].len() as i32)
        };
        Garden {
            plan: tiles,
            size: (rows, cols),
            infinite: false,
        }
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        let mut problem = Self::parse_input_part_1(text_input);
        problem.infinite = true;
        problem
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let garden = problem;
        let start_location = garden.start();
        let locations: HashSet<Location> = HashSet::from([start_location]);
        let answer = (0..64)
            .fold(locations, |locations, _| {
                let new_locations: HashSet<Location> = locations
                    .iter()
                    .flat_map(|location| garden.adj_plots(location))
                    .collect();
                new_locations
            })
            .len();

        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        /*
        the key to the solution is a sequence of numbers up to 26501365, with step of 131
        day 9 had a problem that requires to find following number in the sequence
        by differentiating the given numbers of the sequence.
        */
        //
        let total_steps = 26501365;
        let min_req_sequence = 131*2+65;
        let garden = problem;

        // first we solve problem for monimum required number of steps to get the sequence
        let start_location = garden.start();
        let locations: HashSet<Location> = HashSet::from([start_location]);
        let init_sequence: Vec<i64> = (1..(min_req_sequence+1))
            .scan(locations, |locations, _| {
                let new_locations =
                    locations
                    .iter()
                    .flat_map(|location| garden.adj_plots(location))
                    .collect::<HashSet<Location>>();
                *locations = new_locations;
                Some(locations.clone())
            })
            .map(|ls| ls.len())
            .enumerate()
            .filter(|(idx, _)| (idx+1) % 131 == 65)
            .map(|(_idx, n)| {
                println!("{:>5}: {:>12}", _idx, n);
                n as i64
            })
            .collect();

        // then we extrapolate the sequence to the desired number of steps
        let answer =
            ((min_req_sequence+1)..(total_steps+1))
            .filter(|idx| idx % 131 == 65)
            .fold(init_sequence, |z, _| {
                let next = day_09::DaySolution::find_next_number(0, &z);
                let mut new = z;
                new.push(next);
                new
            })
            .last()
            .map(|v| *v)
            .unwrap();

        Some(answer as usize)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
