use std::collections::HashMap;

type Dir = u8;

const DIR_N: u8 = b'N';
const DIR_E: u8 = b'E';
const DIR_S: u8 = b'S';
const DIR_W: u8 = b'W';

const STRAIGHT_MIN: u8 = 4;
const STRAIGHT_MAX: u8 = 10;
const ALL_DIRECTIONS: [Dir; 4] = [DIR_N, DIR_E, DIR_S, DIR_W];

type HeatLoss = u32;

type Location = (u8, u8);
#[derive(Debug, Clone)]
struct Position {
    loc: Location,
    dir: Dir,
    cnt: u8,
    acc_heat_loss: u32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct MemKey {
    loc: Location,
    dir: Dir,
    cnt: u8,
}

//type Path = Vec<Location>;
type Memory = HashMap<MemKey, HeatLoss>;

type CityMap = Vec<Vec<HeatLoss>>;
type P = CityMap;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Vec<HeatLoss> {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn is_opposite_direction(d1: Dir, d2: Dir) -> bool {
        match (d1, d2) {
            (DIR_S, DIR_N) | (DIR_N, DIR_S) => true,
            (DIR_W, DIR_E) | (DIR_E, DIR_W) => true,
            _ => false,
        }
    }

    // check that location will still be within map boundaries after move is done
    fn within_boundaries(dir: Dir, for_pos: &Position, on_map: &CityMap) -> bool {
        let (r, c) = for_pos.loc;
        let (rows, cols) = Self::map_size(on_map);
        // the constraints are stricter by 1 move
        match dir {
            DIR_N => r > 0,
            DIR_E => c < cols - 1,
            DIR_S => r < rows - 1,
            DIR_W => c > 0,
            _ => unreachable!(),
        }
    }

    // given position and direction, make one move and update position
    fn make_one_move(pos: &Position, dir: Dir, on_map: &CityMap) -> Position {
        let (r, c) = pos.loc;
        let cnt = if pos.dir == dir { pos.cnt + 1 } else { 1 };
        let loc = match dir {
            DIR_N => (r - 1, c + 0),
            DIR_E => (r + 0, c + 1),
            DIR_S => (r + 1, c + 0),
            DIR_W => (r + 0, c - 1),
            _ => unreachable!(),
        };
        let acc_heat_loss = pos.acc_heat_loss + on_map[loc.0 as usize][loc.1 as usize];
        Position {
            loc,
            dir,
            cnt,
            acc_heat_loss,
        }
    }

    fn map_size(map: &CityMap) -> (u8, u8) {
        let r = map.len() as u8;
        let c: u8 = if r == 0 { 0 } else { map[0].len() as u8 };
        (r, c)
    }

    fn find_possible_moves(pos: &Position, on_map: &CityMap, mem: &Memory) -> Vec<Position> {
        ALL_DIRECTIONS
            .iter()
            .filter(|&d2| !Self::is_opposite_direction(pos.dir, *d2))
            .filter(|&dir| DaySolution::within_boundaries(*dir, pos, on_map))
            .map(|dir| Self::make_one_move(pos, *dir, on_map))
            .filter(|p| p.cnt <= STRAIGHT_MAX && (p.dir == pos.dir || pos.cnt >= STRAIGHT_MIN))
            .filter(|p| {
                let key = MemKey {
                    loc: p.loc,
                    dir: p.dir,
                    cnt: p.cnt,
                };
                if let Some((_, least_loss)) = mem.get_key_value(&key) {
                    least_loss > &p.acc_heat_loss
                } else {
                    true
                }
            })
            .collect()
    }

    fn iterate(new_pos: &Vec<Position>, on_map: &CityMap, memory: &Memory) -> Memory {
        // if all possible conditions were checked
        if new_pos.len() == 0 {
            memory.clone()
        } else {
            let mut mem = memory.clone();
            let old_pos = new_pos;
            let new_pos: Vec<Position> = old_pos
                .iter()
                .flat_map(|pos| Self::find_possible_moves(pos, on_map, memory))
                .collect();
            new_pos.iter().for_each(|pos| {
                let key = MemKey {
                    loc: pos.loc,
                    dir: pos.dir,
                    cnt: pos.cnt,
                };
                if let Some(v) = mem.get(&key) {
                    if *v > pos.acc_heat_loss {
                        mem.insert(key, pos.acc_heat_loss);
                    }
                } else {
                    mem.insert(key, pos.acc_heat_loss);
                }
            });
            //prune positions further and drop all those which have worse parameters than in memory
            let mut new_pos: Vec<Position> = new_pos
                //.clone()
                .into_iter()
                .filter(|p| {
                    let key = MemKey {
                        loc: p.loc,
                        dir: p.dir,
                        cnt: p.cnt,
                    };
                    if let Some(least) = mem.get(&key) {
                        *least == p.acc_heat_loss
                    } else {
                        false
                    }
                })
                .collect();
            new_pos.sort_by_key(|a| (a.loc, a.dir, a.cnt));
            new_pos.dedup_by_key(|a| MemKey {
                cnt: a.cnt,
                loc: a.loc,
                dir: a.dir,
            });
            println!(
                "map size: {:>6}, number of new positions: {:>8}",
                mem.len(),
                new_pos.len()
            );
            //new_pos.iter().for_each(|p| { println!("Position: {:?}", p) });
            Self::iterate(&new_pos, on_map, &mem)
        }
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 17;

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
        let init_mem: Memory = HashMap::new();
        let map_size = DaySolution::map_size(&problem);
        //let heat_loss_at_start = problem[0][0];
        let new_pos = vec![
            Position {
                loc: (0, 0),
                dir: DIR_E,
                cnt: 0,
                acc_heat_loss: 0,
            },
            /*
            Position {
                loc: (0, 0),
                dir: DIR_S,
                cnt: 1,
                acc_heat_loss: heat_loss_at_start,
                path: vec![(0, 0)],
            },
            */
        ];
        let memory = DaySolution::iterate(&new_pos, &problem, &init_mem);

        let best: u32 = memory
            .iter()
            .filter(|(k, _)| (k.loc.0 + 1, k.loc.1 + 1) == map_size)
            .filter(|(k, _)| k.cnt >= STRAIGHT_MIN)
            .map(|(_, v)| v)
            .fold(
                u32::MAX,
                |best, this| {
                    if *this < best {
                        *this
                    } else {
                        best
                    }
                },
            );

        let answer = best;
        Some(answer)
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        // same but with different constants STRAIGHT_MIN & STRAIGHT_MAX
        //Self::solve_part_1(_problem)
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
