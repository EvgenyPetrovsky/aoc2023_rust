use regex::Regex;

type Direction = u8;
type Colour = String;
type Location = (i32, i32);

const DIR_U: Direction = b'U';
const DIR_R: Direction = b'R';
const DIR_D: Direction = b'D';
const DIR_L: Direction = b'L';

pub struct Instruction {
    dir: Direction,
    len: usize,
    colour: String,
}

type Seg = (Location, Colour);
type Trench = Vec<Seg>;

type P = Vec<Instruction>;

pub struct DaySolution(P);

impl DaySolution {
    fn parse_one_line(line: &str) -> Instruction {
        let re = Regex::new(r#"(U|R|D|L) (\d+) \((#[[:xdigit:]]{6})\)"#).unwrap();

        re.captures(line)
            .map(|c| {
                let (_, [dir, len, colour]) = c.extract();
                Instruction {
                    dir: dir.as_bytes()[0],
                    len: len.parse::<usize>().unwrap(),
                    colour: String::from(colour),
                }
            })
            .unwrap()
    }

    fn visualise_trench(trench: &Trench) -> () {

        let (rows, cols, lstr, lstc) = trench
            .iter()
            .fold((0_i32, 0_i32, 0_i32, 0_i32), |(rg0, cg0, rl0, cl0), ((r1, c1), _)| {
                (rg0.max(*r1), cg0.max(*c1), rl0.min(*r1), cl0.min(*c1))
            });

            let len = trench.len();
        let (rows, cols) = (1 + (rows - lstr) as usize, 1 + (cols - lstc) as usize);
        println!("Trench has {len} segments and forms area of {rows} rows and {cols} columns");
        let mut area: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
        trench
            .iter()
            .for_each(|s| {
                let (r, c) = s.0;
                let (r, c) = (r - lstr, c - lstc);
                area[r as usize][c as usize] = '#';
            });
        area
            .iter()
            .for_each(|r| {
                println!("{}", r.iter().collect::<String>());
            })
        /*
        let a =
        self
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
    }

    fn build_trench(instructions: Vec<Instruction>) -> Trench {
        let initial_state = ((0_i32, 0_i32), String::from(""));
        let trench = instructions
            .iter()
            .flat_map(|i| {
                let len = i.len+1;
                let (row_inc, col_inc) =
                    match i.dir {
                        DIR_U => ( -1,  0 ),
                        DIR_R => (  0,  1 ),
                        DIR_D => (  1,  0 ),
                        DIR_L => (  0, -1 ),
                        _ => unreachable!()
                    };
                (1..len).map(move |_| ((row_inc, col_inc), i.colour.clone()))
            })
            .scan(initial_state, |((r0, c0), _), ((r1, c1), col)| {
                *r0 += r1 as i32;
                *c0 += c1 as i32;
                let res: (Location, Colour) = ((*r0 as i32, *c0 as i32), col);
                Some(res)
            })
            .collect();
        trench

    }
}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 18;

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
        let trench = DaySolution::build_trench(problem);
        DaySolution::visualise_trench(&trench);
        None
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("\x1b[93mError\x1b[0m"),
        }
    }
}
