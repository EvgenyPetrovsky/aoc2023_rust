type P = ();

pub struct DaySolution(P);

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 19;

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
