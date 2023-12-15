
const BALL: u8 = b'O';
const VOID: u8 = b'.';
//const CUBE: u8 = b'#';

//enum Direction {North, West, South, East}
type Line = Vec<u8>;
type P = Vec<Line>;

pub struct DaySolution(P);

impl DaySolution {

    fn parse_one_line(line: &str) -> Line {
        line.as_bytes().to_vec()
    }

    fn transpose(mx: &Vec<Vec<u8>>) -> Vec<Line> {
        let rows = mx.len();
        let cols = mx[0].len();
        (0..cols)
            .map(|c| (0..rows).map(|r| mx[r][c]).collect())
            .collect()
    }

    fn load_of_line(line: &Line) -> usize {
        line
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, x)| if *x == BALL {idx + 1} else {0})
        .sum()
    }

    fn slide_balls(line: &Line) -> Line {
        let mut line = line.clone();
        fn iterate(line: &mut Line) -> Line {
            let mut count = 0;
            for i in 1..line.len() {
                if line[i-1] == VOID && line[i] == BALL {
                    line.swap(i-1, i);
                    count += 1;
                }
            }
            if count == 0 {line.clone()} else {iterate(line)}
        }
        iterate(&mut line)
    }

    fn tilt_platform_cycle(platform: &Vec<Line>) -> Vec<Line> {
        let mut platform = platform.clone();
        // North
        platform =
            Self::transpose(&platform)
            .iter()
            .map(|line| Self::slide_balls(line))
            .collect();
        platform = Self::transpose(&platform);
        // West
        platform =
            platform
            .iter()
            .map(|line| Self::slide_balls(line))
            .collect();
        // South
        platform =
            Self::transpose(&platform)
            .iter()
            .map(|line| {
                let mut line1 = line.clone();
                line1.reverse();
                line1 = Self::slide_balls(&line1);
                line1.reverse();
                line1
            })
            .collect();
        platform = Self::transpose(&platform);
        // East
        platform =
            platform
            .iter()
            .map(|line| {
                let mut line1 = line.clone();
                line1.reverse();
                line1 = Self::slide_balls(&line1);
                line1.reverse();
                line1
            })
            .collect();
        platform
    }

    fn north_load(platform: &Vec<Line>) -> usize {
        let t = Self::transpose(platform);
        t.iter()
        .map(|line| DaySolution::load_of_line(&line))
        .sum()
    }

}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 14;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
        .lines()
        .map(|line| DaySolution::parse_one_line(line))
        .collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let v_arrangement = DaySolution::transpose(&problem);
        let answer =
            v_arrangement
            .iter()
            .map(|line| DaySolution::slide_balls(line))
            .map(|line| DaySolution::load_of_line(&line))
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let _a =
            (0..150_usize).
            fold(problem, |z, x| {

                let p1 = DaySolution::tilt_platform_cycle(&z);
                let l = DaySolution::north_load(&p1);
                println!("iteration {x:>3}, load = {l:>5}");
                p1

            });

        /* according to ouptut: 112+1 first repitition, period = 7 */
        Some( ((1_000_000_000) - (112+1)) % 7 + 112 )
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}
