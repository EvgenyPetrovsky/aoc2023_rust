#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Location {
    r: usize,
    c: usize,
}
#[derive(Debug, PartialEq, Eq)]
enum Segment {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    S,
    O,
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}
struct State {
    lcn: Location,
    dir: Direction,
}
pub struct PipeMap {
    size: Location,
    segments: Vec<Vec<Segment>>,
}

type P = PipeMap;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_byte(b: u8) -> Segment {
        match b {
            b'|' => Segment::NS,
            b'-' => Segment::EW,
            b'L' => Segment::NE,
            b'J' => Segment::NW,
            b'F' => Segment::SE,
            b'7' => Segment::SW,
            b'.' => Segment::O,
            b'S' => Segment::S,
            _ => panic!("could't parse '{}'", b),
        }
    }
    /*
    fn encode_segment(s: &Segment) -> char {
        match s {
            Segment::NS => '│',
            Segment::EW => '─',
            Segment::NE => '└',
            Segment::NW => '┘',
            Segment::SE => '┌',
            Segment::SW => '┐',
            Segment::O => '.',
            Segment::S => 'S',
        }
    }
    */

    fn locate_start(map: &PipeMap) -> Location {
        let cardinality = map.size.r * map.size.c;
        let c = map.size.c;
        (0..cardinality)
            .map(|i| (i / c, i % c))
            .filter_map(|(r, c)| {
                if map.segments[r][c] == Segment::S {
                    Some(Location { r, c })
                } else {
                    None
                }
            })
            .nth(0)
            .unwrap()
    }
    fn init_state(map: &PipeMap) -> State {
        let lcn = Self::locate_start(map);
        // we assume that we always star facing south (based on test and real input)
        State {
            dir: Direction::S,
            lcn,
        }
    }
    fn move_once(map: &PipeMap, state: &State) -> State {
        let (r0, c0) = (state.lcn.r, state.lcn.c);
        let d0 = &state.dir;
        let (r1, c1) = match d0 {
            Direction::N => (r0 - 1, c0 + 0),
            Direction::E => (r0 + 0, c0 + 1),
            Direction::S => (r0 + 1, c0 + 0),
            Direction::W => (r0 + 0, c0 - 1),
        };
        let s1 = &map.segments[r1][c1];
        let d1 = match (d0, s1) {
            (Direction::N, Segment::NS) => Direction::N,
            (Direction::N, Segment::SE) => Direction::E,
            (Direction::N, Segment::SW) => Direction::W,

            (Direction::E, Segment::EW) => Direction::E,
            (Direction::E, Segment::NW) => Direction::N,
            (Direction::E, Segment::SW) => Direction::S,

            (Direction::S, Segment::NS) => Direction::S,
            (Direction::S, Segment::NE) => Direction::E,
            (Direction::S, Segment::NW) => Direction::W,

            (Direction::W, Segment::EW) => Direction::W,
            (Direction::W, Segment::NE) => Direction::N,
            (Direction::W, Segment::SE) => Direction::S,

            // doesn't really matter, because traverse will stop once Start is reached
            (_, Segment::S) => Direction::S,

            _ => panic!(
                "Couldn't recognize new direction based on direction {:?} and segment {:?} in {:?}",
                d0, s1, state.lcn
            ),
        };

        State {
            lcn: Location { r: r1, c: c1 },
            dir: d1,
        }
    }
    //fn move()
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 10;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let segments: Vec<Vec<Segment>> = text_input
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|b| DaySolution::parse_byte(b))
                    .collect::<Vec<Segment>>()
            })
            .collect();
        let size = Location {
            r: segments.len(),
            c: segments[0].len(),
        };
        PipeMap { size, segments }
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let map = problem;
        let start = DaySolution::init_state(&map);
        let start_lcn = &start.lcn;
        fn iter(acc: usize, state: &State, map: &PipeMap, start: &Location) -> usize {
            let new_state = DaySolution::move_once(&map, state);
            if new_state.lcn.c == start.c && new_state.lcn.r == start.r {
                acc
            } else {
                iter(acc + 1, &new_state, map, start)
            }
        }
        let loop_len = iter(0, &start, &map, start_lcn);
        let answer = (loop_len + 1) / 2;
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let pipemap = problem;
        let start = DaySolution::init_state(&pipemap);
        let start_lcn = &start.lcn;
        let mut acc: Vec<Location> = vec![start_lcn.clone()];
        fn iter(
            acc: &mut Vec<Location>,
            state: &State,
            map: &PipeMap,
            start: &Location,
        ) -> Vec<Location> {
            let new_state = DaySolution::move_once(&map, state);
            if &new_state.lcn == start {
                acc.clone()
            } else {
                acc.push(new_state.lcn.clone());
                iter(acc, &new_state, map, start)
            }
        }
        let loop_lcns = iter(&mut acc, &start, &pipemap, start_lcn);
        let answer: usize = pipemap
            .segments
            .iter()
            .enumerate()
            .map(|(ir, r)| {
                r.iter()
                    .enumerate()
                    .fold((0_usize, 0_usize), |(crosses, acc), (ic, s)| {
                        if loop_lcns.contains(&Location { r: ir, c: ic }) {
                            match s {
                                Segment::NS => (crosses + 1, acc),
                                _ => (crosses, acc),
                            }
                        } else if crosses % 2 == 1 {
                            (crosses, acc + 1)
                        } else {
                            (crosses, acc)
                        }
                    })
            })
            .map(|(_, acc)| acc)
            .sum();
        /*
        // now we will print the map and count inner elements manually
        pipemap
        .segments
        .iter()
        .enumerate()
        .for_each(|(ir, r)| {
            let line =
                r.iter().enumerate().map(|(ic, s)| {
                    let c = DaySolution::encode_segment(s);
                    if loop_lcns.contains(&Location{ r: ir, c: ic}) {c} else {'.'}
                }).collect::<String>();
            println!("{}", line);
        });
        */
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
