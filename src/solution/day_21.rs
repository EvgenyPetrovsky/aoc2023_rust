use std::collections::HashSet;

enum Tile {
    Plot,
    Rock,
    Start,
}

type Location = (usize, usize);

pub struct Garden {
    plan: Vec<Vec<Tile>>,
    size: Location,
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
                        Tile::Start => Some((ridx, cidx)),
                        _ => None,
                    })
            })
            .nth(0)
            .unwrap()
    }

    // only plots, no rocks no other location outside garden
    fn adj_plots(&self, location: &Location) -> Vec<Location> {
        let (rows, cols) = (self.size.0 as i32, self.size.1 as i32);
        let (r, c) = (location.0 as i32, location.1 as i32);
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|(r, c)| 0 <= *r && *r < rows && 0 <= *c && *c < cols)
            .map(|(r, c)| (r as usize, c as usize))
            .filter(|(r, c)| match self.plan[*r][*c] {
                Tile::Plot => true,
                Tile::Start => true,
                _ => false,
            })
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
            (0_usize, 0_usize)
        } else {
            (tiles.len(), tiles[0].len())
        };
        Garden {
            plan: tiles,
            size: (rows, cols),
        }
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
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

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
