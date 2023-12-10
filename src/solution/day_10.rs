type P = ();

struct Position{r: usize, c: usize}
enum Segment {NS, EW, NE, NW, SW, SE, S, O}
enum Direction {N, E, S, W}
struct State {pos: Position, dir: Direction}
type Map = Vec<Vec<Segment>>;
pub struct DaySolution(P);

impl DaySolution {
    fn parse_byte(b: Byte) -> Segment {
        match b {
            b'|' => NS,
            b'-' => EW,
            b'L' => NE,
            b'J' => NW,
            b'F' => SE,
            b'7' => SW,
            b'.' => O,
            b'S' => S,
            _ => panic!("could't parse '{}'", b.make_ascii_uppercase()),
        }
    }
    fn locate_start() -> Position {
        unimplemented!()
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
