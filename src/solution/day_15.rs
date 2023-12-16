use std::fmt;

type Hash = u8;
#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal: u8,
}

#[derive(Debug, Clone)]
enum Do {
    Pull { label: String },
    Push { label: String, focal: u8 },
}
#[derive(Clone, Debug)]
struct Box(Vec<Lens>);
type Boxes = Vec<Box>;

#[derive(Debug)]
pub struct Instruction {
    encoded: String,
    operation: Do,
}
type P = Vec<Instruction>;

impl Instruction {
    // parse instruction from text
    fn from(encoded: &String) -> Self {
        let operation = if let Some('-') = encoded.chars().last() {
            let label = encoded.split("-").collect::<Vec<&str>>();
            Do::Pull {
                label: String::from(label[0]),
            }
        } else {
            let parts = encoded.split("=").collect::<Vec<&str>>();
            Do::Push {
                label: String::from(parts[0]),
                focal: parts[1].parse::<u8>().unwrap(),
            }
        };

        let i = Self {
            encoded: encoded.clone(),
            operation,
        };
        //println!("instruction: {:?}", &i);
        i
    }
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {:<3}]", self.label, self.focal)
    }
}

impl Lens {
    fn from(label: &String, focal: u8) -> Self {
        Lens {
            label: label.clone(),
            focal: focal,
        }
    }
    fn hash(&self) -> Hash {
        DaySolution::hash(&(self.label))
    }
}

impl Box {
    // initialize empty box
    fn new() -> Self {
        Self(vec![])
    }
    // position of the lens in the box
    fn locate(&self, lens: &Lens) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| {
                if x.label == lens.label {
                    Some(idx)
                } else {
                    None
                }
            })
            .nth(0)
    }
    // if box contains lens with the same label
    fn contains(&self, lens: &Lens) -> bool {
        self.locate(lens).is_some()
    }
    // remove lens from the box, returns the new state of the box
    fn pull(self, lens: &Lens) -> Self {
        if self.contains(lens) {
            let new: Vec<Lens> = self
                .0
                .iter()
                .filter(|x| x.label != lens.label)
                .map(|x| x.clone())
                .collect();
            Self(new)
        } else {
            self.clone()
        }
    }
    // add lens - remember that oldest len is closer to the beginning
    fn push(self, lens: &Lens) -> Self {
        match self.locate(lens) {
            Some(idx) => {
                let mut new = self.0.clone();
                new[idx] = lens.clone();
                Self(new)
            }
            None => {
                let mut new = self.0;
                new.push(lens.clone());
                Self(new)
            }
        }
    }
    // focusing power of the box
    fn power(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, l)| (idx + 1) * l.focal as usize)
            .sum()
    }
}

pub struct DaySolution(P);

impl DaySolution {
    // function that returns hash for given string
    fn hash(bytes: &String) -> Hash {
        let val = bytes
            .bytes()
            //.iter()
            .map(|b| b as u32)
            .fold(0, |z, x| (z + x) * 17 % 256) as u8;
        //println!("hash value of '{}' is {}", bytes, val);
        val
    }
}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 15;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
            .split(",")
            .map(|x| Instruction::from(&(String::from(x))))
            .collect()
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let answer = problem
            .iter()
            .map(|instruction| DaySolution::hash(&(instruction.encoded)) as usize)
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let b = Box::new();
        let mut boxes: Boxes = vec![b; 256];
        problem
            .iter()
            .map(|i| i.operation.clone())
            .for_each(|op| match op {
                Do::Pull { label } => {
                    let lens = Lens::from(&label, 0);
                    let idx = lens.hash() as usize;
                    boxes[idx] = boxes[idx].clone().pull(&lens);
                }
                Do::Push { label, focal } => {
                    let lens = Lens::from(&label, focal);
                    let idx = lens.hash() as usize;
                    boxes[idx] = boxes[idx].clone().push(&lens);
                }
            });
        /*
        println!("boxes:");
        boxes.iter()
        .enumerate()
        .filter(|(_, b)| b.0.len() > 0)
        .for_each(|(i, b)| println!("Box {i:<3}: {:?}" , b.0));
        */
        let answer: usize = boxes
            .iter()
            .enumerate()
            .map(|(idx, b)| (idx + 1) * b.power())
            .sum();
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
