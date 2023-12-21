use regex::Regex;

type Direction = u8;
type Location = (i64, i64);
type Distance = i64;

const DIR_U: Direction = b'U';
const DIR_R: Direction = b'R';
const DIR_D: Direction = b'D';
const DIR_L: Direction = b'L';

pub struct Instruction {
    dir: Direction,
    len: i64,
}

struct Vertex {
    loc: Location,
    dis: Distance,
}
struct Trench(Vec<Vertex>);

type P = Vec<Instruction>;

pub struct DaySolution(P);

impl Trench {
    fn build(instructions: Vec<Instruction>) -> Self {
        let initial_state = Vertex {
            loc: (0_i64, 0_i64),
            dis: 0,
        };
        let trench = instructions
            .iter()
            .map(|i| {
                let (row_inc, col_inc) = match i.dir {
                    DIR_U => (-1, 0),
                    DIR_R => (0, 1),
                    DIR_D => (1, 0),
                    DIR_L => (0, -1),
                    _ => unreachable!(),
                };
                ((row_inc * i.len, col_inc * i.len), i.len)
            })
            .scan(
                initial_state,
                |Vertex {
                     loc: (r0, c0),
                     dis: _,
                 },
                 ((r1, c1), len)| {
                    *r0 += r1 as i64;
                    *c0 += c1 as i64;
                    let res = Vertex {
                        loc: (*r0 as i64, *c0 as i64),
                        dis: len,
                    };
                    Some(res)
                },
            )
            .collect::<Vec<Vertex>>();
        Self(trench)
    }

    fn _visualise(&self) -> () {
        let (rows, cols, lstr, lstc) = self.0.iter().fold(
            (0_i64, 0_i64, 0_i64, 0_i64),
            |(rg0, cg0, rl0, cl0),
             Vertex {
                 loc: (r1, c1),
                 dis: _,
             }| { (rg0.max(*r1), cg0.max(*c1), rl0.min(*r1), cl0.min(*c1)) },
        );

        let len = self.0.len();
        let (rows, cols) = (1 + (rows - lstr) as usize, 1 + (cols - lstc) as usize);
        println!("Trench has {len} segments and forms area of {rows} rows and {cols} columns");
        let mut area: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
        self.0.iter().for_each(|s| {
            let (r, c) = s.loc;
            let (r, c) = (r - lstr, c - lstc);
            area[r as usize][c as usize] = '#';
        });
        area.iter().for_each(|r| {
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

    fn area(&self) -> usize {
        /*
        use Shoelace formula to calculate area using vertexes
        https://en.wikipedia.org/wiki/Shoelace_formula
        */

        let v1 = self.0.iter().map(|v| v.loc.clone());
        //let num_vs = v1.count();
        let v2 = v1.clone().chain(v1.clone().take(1)).skip(1);

        let shoelace: i64 = v1
            .zip(v2)
            .map(|(loc1, loc2)| loc1.0 * loc2.1 - loc2.0 * loc1.1)
            .sum();
        let perimeter: i64 = self.0.iter().map(|v| v.dis).sum();
        let area = shoelace.abs() / 2 + perimeter / 2 + 1;

        area as usize
        /*

        ########**
        ########**
        ##      **
        ##      **
        ##      **
        ##      **
        ********++
        ********++

        (0 0 3 3 0)
        (0 3 3 0 0)
        0*3 - 0*0 + 0*3 - 3*3 + 3*0 - 3*3 + 3*0 - 0*0 = -18
        18 / 2 + 12 / 2 + 1 = 9 + 6 + 1 = 16

        */
    }
}

impl DaySolution {
    fn parse_one_line_1(line: &str) -> Instruction {
        let re = Regex::new(r#"(U|R|D|L) (\d+) \((#[[:xdigit:]]{6})\)"#).unwrap();

        re.captures(line)
            .map(|c| {
                let (_, [dir, len, _]) = c.extract();
                Instruction {
                    dir: dir.as_bytes()[0],
                    len: len.parse::<i64>().unwrap(),
                }
            })
            .unwrap()
    }

    fn parse_one_line_2(line: &str) -> Instruction {
        let re = Regex::new(r#"\(#([[:xdigit:]]{5})([[:xdigit:]]{1})\)"#).unwrap();

        re.captures(line)
            .map(|c| {
                let (_, [len, dir]) = c.extract();
                let dir = match dir {
                    "0" => b'R',
                    "1" => b'D',
                    "2" => b'L',
                    "3" => b'U',
                    _ => unreachable!(),
                };
                Instruction {
                    dir,
                    len: i64::from_str_radix(len, 16).unwrap(),
                }
            })
            .unwrap()
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 18;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(DaySolution::parse_one_line_1)
            .collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(DaySolution::parse_one_line_2)
            .collect()
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let trench = Trench::build(problem);
        //DaySolution::visualise_trench(&trench);
        let answer = trench.area();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let answer = Trench::build(problem).area();
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("\x1b[93mError\x1b[0m"),
        }
    }
}
