use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location(i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    location: Location,
    direction: Direction,
}

type Memory = HashSet<Position>;

#[derive(Debug, Clone)]
pub struct Contraption {
    size: (usize, usize),
    data: Vec<Vec<u8>>,
}

type P = Contraption;

impl Position {
    fn new(r: i32, c: i32, d: u8) -> Self {
        let location = Location(r, c);
        let direction = match d {
            b'N' => Direction::N,
            b'E' => Direction::E,
            b'S' => Direction::S,
            b'W' => Direction::W,
            _ => panic!("wrong direction paramerter value, chose from N, E, S, W"),
        };
        Position {
            location,
            direction,
        }
    }
}

pub struct DaySolution(P);

impl DaySolution {
    fn is_valid_location(location: &Location, on_map: &Contraption) -> bool {
        let Location(r, c) = location;
        let (rows, cols) = on_map.size;
        0 <= *r && *r < rows as i32 && 0 <= *c && *c < cols as i32
    }

    fn new_location(location: &Location, direction: &Direction) -> Location {
        let Location(r, c) = location;
        match direction {
            Direction::N => Location(r - 1, c + 0),
            Direction::E => Location(r + 0, c + 1),
            Direction::S => Location(r + 1, c + 0),
            Direction::W => Location(r + 0, c - 1),
        }
    }

    fn new_position(
        new_location: &Location,
        direction: &Direction,
        on_map: &Contraption,
    ) -> Vec<Position> {
        let r = new_location.0 as usize;
        let c = new_location.1 as usize;

        let new_directions = match (on_map.data[r][c], direction) {
            (b'.', _) => vec![direction.clone()],
            (b'-', Direction::E) | (b'-', Direction::W) => vec![direction.clone()],
            (b'|', Direction::N) | (b'|', Direction::S) => vec![direction.clone()],
            (b'/', Direction::E) | (b'\\', Direction::W) => vec![Direction::N],
            (b'\\', Direction::E) | (b'/', Direction::W) => vec![Direction::S],
            (b'/', Direction::S) | (b'\\', Direction::N) => vec![Direction::W],
            (b'\\', Direction::S) | (b'/', Direction::N) => vec![Direction::E],

            (b'|', Direction::E) | (b'|', Direction::W) => vec![Direction::N, Direction::S],
            (b'-', Direction::N) | (b'-', Direction::S) => vec![Direction::W, Direction::E],

            _ => unreachable!(),
        };

        new_directions
            .iter()
            .map(|d| Position {
                location: new_location.clone(),
                direction: d.clone(),
            })
            .collect()
    }

    fn make_step(from_pos: &Position, on_map: &Contraption) -> Vec<Position> {
        let new_l = Self::new_location(&from_pos.location, &from_pos.direction);
        if !Self::is_valid_location(&new_l, on_map) {
            vec![]
        } else {
            Self::new_position(&new_l, &from_pos.direction, on_map)
        }
    }
    // print energised locations
    fn visualise_locations(locations: &Vec<Location>) {
        let (void, fill) = ('.', '#');
        let (rows, cols) = locations
            .iter()
            .fold((0, 0), |z, l| (z.0.max(l.0), z.1.max(l.1)));
        let mut print_area: Vec<Vec<char>> = vec![vec![void; 1 + cols as usize]; 1 + rows as usize];
        for l in locations {
            print_area[l.0 as usize][l.1 as usize] = fill;
        }
        println!("Energised locations ({} items):", locations.len());
        print_area
            .iter()
            .for_each(|r| println!("{}", r.iter().collect::<String>()));
    }

    fn iterate(
        problem: &Contraption,
        memory: &mut Memory,
        positions: Vec<Position>,
        cnt: usize,
    ) -> Memory {
        let new_ps: Vec<Position> = positions
            .iter()
            .flat_map(|p| DaySolution::make_step(p, problem))
            .filter(|p| !memory.contains(p))
            .collect::<Vec<Position>>();
        if cnt % 100 == 0 {
            //println!("Step: {:<5}, new positions: {}", cnt, new_ps.len());
        }
        if new_ps.len() == 0 {
            memory.clone()
        } else {
            for p in &new_ps {
                memory.insert(p.clone());
            }
            Self::iterate(problem, memory, new_ps, cnt + 1)
        }
    }

    fn find_all_init_locations(problem: &Contraption) -> Vec<Position> {
        let (rows, cols) = problem.size;
        let (rows, cols) = (rows as i32, cols as i32);
        let it1 = (0..rows).map(|x| Position::new(x, -1, b'E'));
        let it2 = (0..rows).map(|x| Position::new(x, cols, b'W'));
        let it3 = (0..cols).map(|x| Position::new(-1, x, b'S'));
        let it4 = (0..cols).map(|x| Position::new(rows, x, b'N'));
        it1.chain(it2).chain(it3).chain(it4).collect()
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 16;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let data: Vec<Vec<u8>> = text_input
            .lines()
            .map(|l| l.as_bytes().iter().map(|x| *x).collect())
            .collect();
        let size: (usize, usize) = (data.len(), data[0].len());
        Contraption { size, data }
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let location = Location(0, -1);
        let direction = Direction::E;
        let init_pos = Position {
            location,
            direction,
        };
        let mut memory: Memory = Memory::new();

        let new_positions = DaySolution::iterate(&problem, &mut memory, vec![init_pos], 0);
        let locations: HashSet<Location> =
            new_positions.iter().map(|p| p.location.clone()).collect();
        let answer = locations.len();

        // print locations
        if false {
            let vec_locations: Vec<Location> =
                locations.clone().iter().map(|x| x.clone()).collect();
            DaySolution::visualise_locations(&vec_locations);
        }

        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let init_locations = DaySolution::find_all_init_locations(&problem);
        //let answer =
        init_locations
            .iter()
            .map(|init_pos| {
                let mut memory: Memory = Memory::new();

                let new_positions =
                    DaySolution::iterate(&problem, &mut memory, vec![init_pos.clone()], 0);
                let locations: HashSet<Location> =
                    new_positions.iter().map(|p| p.location.clone()).collect();
                locations.len()
            })
            .max()
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
