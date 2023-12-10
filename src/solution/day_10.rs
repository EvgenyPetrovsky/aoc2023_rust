type P = ();

struct Position{r: usize, c: usize}
#[derive(Debug, PartialEq, Eq)]
enum Segment {NS, EW, NE, NW, SW, SE, S, O}
#[derive(Debug, PartialEq, Eq)]
enum Direction {N, E, S, W}
struct State {pos: Position, dir: Direction}
struct Map { max_pos: Position, content: Vec<Vec<Segment>> }
pub struct DaySolution(P);

impl DaySolution {
    fn parse_byte(c: char) -> Segment {

        match c as u8 {
            b'|' => Segment::NS,
            b'-' => Segment::EW,
            b'L' => Segment::NE,
            b'J' => Segment::NW,
            b'F' => Segment::SE,
            b'7' => Segment::SW,
            b'.' => Segment::O,
            b'S' => Segment::S,
            _ => panic!("could't parse '{}'", c),
        }
    }
    fn locate_start(map: &Map) -> Position {
        let cardinality = map.max_pos.r * map.max_pos.c;
        let c = map.max_pos.c;
        (0..cardinality)
        .map(|i| {(i/c, i%c)})
        .filter_map(|(r,c)| {
            if map.content[r][c] == Segment::S {Some(Position {r, c})} else {None}
        })
        .nth(0)
        .unwrap()
    }
    fn init_state(map: &Map) -> State {
        let pos = Self::locate_start(map);
        // we assume that we always star facing south (based on test and real input)
        State { dir: Direction::S, pos }
    }
    fn move_once(map: &Map, state: State) -> State{
        let (r0,c0) = (state.pos.r, state.pos.c);
        let d0 = &state.dir;
        let (r1, c1) = match d0 {
            Direction::N => (r0+0, c0-1),
            Direction::E => (r0+1, c0+0),
            Direction::S => (r0+0, c0+1),
            Direction::W => (r0-1, c0+0),
        };
        let s1 = &map.content[r1][c1];
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

            _ => panic!("Couldn't recognize new direction based on {:?} and {:?}", d0, s1),
        };

        State{ pos: Position { r: r1, c: c1 }, dir: d1 }

    }
  //fn move()
}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 10;

    type Answer = Option<i32>;
    type Problem = P;

    fn parse_input_part_1(_text_input: String) -> Self::Problem {
        unimplemented!();
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(_problem: Self::Problem) -> Self::Answer {
        None
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
