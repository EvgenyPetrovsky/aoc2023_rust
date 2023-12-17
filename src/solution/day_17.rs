use std::collections::HashMap;

type HeatLoss = u32;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir { N, E, S, W }


type Location = (i32, i32);
#[derive(Debug)]
struct Position { loc: Location, dir: Dir, cnt: u32, acc_heat_loss: u32}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MemKey {loc: Location, dir: Dir, cnt: u32}

type Memory = HashMap<MemKey, HeatLoss>;

type CityMap = Vec<Vec<HeatLoss>>;
type P = CityMap;


const STRAIGHT_LIMIT: u32 = 3;
const ALL_DIRECTIONS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::E];

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Vec<HeatLoss> {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn is_opposite_direction(d1: &Dir, d2: &Dir) -> bool {
        match (d1, d2) {
            (Dir::S, Dir::N) | (Dir::N, Dir::S) => true,
            (Dir::W, Dir::E) | (Dir::E, Dir::W) => true,
            _ => false
        }
    }

    fn make_one_move(pos: &Position, dir: &Dir, on_map: &CityMap) -> Position {
        let (r, c) = pos.loc;
        let cnt = if &pos.dir == dir {pos.cnt + 1} else {1};
        let loc = match dir {
            Dir::N => (r-1, c+0),
            Dir::E => (r+0, c+1),
            Dir::S => (r+1, c+0),
            Dir::W => (r+0, c-1),
        };
        let acc_heat_loss = pos.acc_heat_loss + on_map[loc.0 as usize][loc.1 as usize];

        Position { loc, dir: dir.clone(), cnt, acc_heat_loss }
    }

    fn map_size(map: &CityMap) -> (i32, i32) {
        let r = map.len() as i32;
        let c: i32 = if r == 0 {0} else {map[0].len() as i32};
        (r, c)
    }

    fn find_possible_moves(pos: &Position, on_map: &CityMap, mem: &Memory) -> Vec<Position> {
        let (rows, cols) = Self::map_size(on_map);
        //let m = HashMap::
        ALL_DIRECTIONS
        .iter()
        .filter(|&d2| !Self::is_opposite_direction(&pos.dir, d2))
        .map(|dir| Self::make_one_move(pos, dir, on_map))
        .filter(|p| {
            0 <= p.loc.0 && p.loc.0 < rows && 0 <= p.loc.1 && p.loc.1 < cols
        })
        .filter(|p| {p.cnt <= STRAIGHT_LIMIT})
        .filter(|p| {
            let key = MemKey { loc: p.loc, dir: p.dir.clone(), cnt: p.cnt};
            if let Some((_, x)) = mem.get_key_value(&key) {
                x > &p.acc_heat_loss
            } else {
                true
            }
        })
        .collect()
    }

    fn iterate(new_pos: &Vec<Position>, on_map: &CityMap, memory: &Memory) -> Memory {
        if new_pos.len() == 0 {
            memory.clone()
        } else {
            let old_pos = new_pos;
            let new_pos: Vec<Position> =
            old_pos
            .iter()
            .flat_map(|pos| Self::find_possible_moves(pos, on_map, memory))
            .collect();

            let mut mem = memory.clone();
            new_pos
                .iter()
                .for_each(|pos| {
                    let key = MemKey { loc: pos.loc, dir: pos.dir.clone(), cnt: pos.cnt};
                    mem.insert(key, pos.acc_heat_loss);
                });

            Self::iterate(&new_pos, on_map, &mem)

        }
    }

}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 17;

    type Answer = Option<u32>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input.lines().map(DaySolution::parse_one_line).collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let init_mem: Memory = HashMap::new();
        let map_size = DaySolution::map_size(&problem);
        let heat_loss_at_start = problem[0][0];
        let new_pos = vec![
            Position {loc: (0,0), dir: Dir::E, cnt: 1, acc_heat_loss: heat_loss_at_start},
            Position {loc: (0,0), dir: Dir::S, cnt: 1, acc_heat_loss: heat_loss_at_start},
        ];
        let memory = DaySolution::iterate(&new_pos, &problem, &init_mem);

        memory
            .iter()
            .filter(|(k, _)| {
                (k.loc.0 + 1, k.loc.1 + 1) == map_size
            })
            .map(|(k, v)| *v)
            .max()
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}
