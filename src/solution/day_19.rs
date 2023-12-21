use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Part {x: usize, m: usize, a: usize, s: usize}

pub struct PartRanges {x: (usize, usize), m: (usize, usize), a: (usize, usize), s: (usize, usize)}

#[derive(Debug, Clone)]
pub struct Workflow { name: String, rules: Vec<Rule> }

#[derive(Debug, Clone)]
enum Rule { Condition(Condition), Decision(Decision) }

#[derive(Debug, Clone)]
struct Condition { par: char, cmp: Comparison, val: usize, dec: Decision }

#[derive(Debug, Clone)]
enum Decision { Accept, Reject, SendTo (String) }

#[derive(Debug, Clone)]
enum Comparison { Lt, Gt }

pub struct P {
    flows: HashMap<String, Workflow>,
    parts: Vec<Part>
}

impl DaySolution {

    fn parse_one_rule (line: &str) -> Rule {
        let re_condition = Regex::new(r#"^([xmas])([<>])(\d+):([AR]|[a-z]+)$"#).unwrap();
        let re_decision  = Regex::new(r#"^([AR]|[a-z]+)$"#).unwrap();

        fn match_decision(dec: &str) -> Decision {
            match dec {
                "A" => Decision::Accept,
                "R" => Decision::Reject,
                _ => Decision::SendTo(String::from(dec))
            }
        }
        let condition =
            re_condition
            .captures(line)
            .map(|c| match c.get(0) {
                Some(_) => {
                    let (_, [par, cmp, val, dec]) = c.extract();
                    let c = Condition {
                        par: par.chars().nth(0).unwrap(),
                        cmp: if cmp == "<" {Comparison::Lt} else {Comparison::Gt},
                        val: val.parse::<usize>().unwrap(),
                        dec: match_decision(dec),
                    };
                    Some(c)
                },
                _ => None
            });
        let decision =
            re_decision
            .captures(line)
            .map(|c| match c.get(0) {
                Some(_) => {
                    let (_, [dec]) = c.extract();
                    Some(match_decision(dec))
                },
                _ => None
            });

        if condition.is_some() {
            Rule::Condition(condition.unwrap().unwrap())
        } else {
            Rule::Decision(decision.unwrap().unwrap())
        }

    }
    fn parse_one_workflow(line: &str) -> Workflow {
        let main_re = Regex::new(r#"(\w+)\{([0-9a-zAR<>:,]+)\}"#).unwrap();
        let rules_re = Regex::new(r#"[0-9a-zAR<>:]+"#).unwrap();
        let (name, rules_str) =
        main_re.captures(line).map(|c| {
            let(_, [name, rules]) = c.extract();
            (String::from(name), rules)
        })
        .unwrap();
        let rules =
            rules_re.captures_iter(rules_str).map(|c| {
                let rule = c.get(0).unwrap().as_str();
                Self::parse_one_rule(rule)
            })
            .collect::<Vec<Rule>>();
        Workflow { name: name , rules }
    }

    fn parse_one_part(line: &str) -> Part {
        let re = Regex::new(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"#).unwrap();
        re.captures(line).map(|c| {
            let (_, [x, m, a, s]) = c.extract();
            Part {
                x: x.parse::<usize>().unwrap(),
                m: m.parse::<usize>().unwrap(),
                a: a.parse::<usize>().unwrap(),
                s: s.parse::<usize>().unwrap(),
            }
        })
        .unwrap()
    }

    fn classify_one_part(part: &Part, flow: &String, flows: &HashMap<String, Workflow>) -> Decision {

        let rules = flows.get(flow).unwrap().clone().rules;

        let opt_decision: Option<Decision> = rules
            .iter()
            .fold(None, |z, rule| {

                let new_decision =
                    match rule {
                        Rule::Condition(c) => {
                            let lhs = match c.par {
                                'x' => part.x,
                                'm' => part.m,
                                'a' => part.a,
                                's' => part.s,
                                _ => unreachable!("pasrt name is not in XMAS list (condition:{:?})", &c)
                            };
                            match c.cmp {
                                Comparison::Gt => if lhs > c.val {Some(c.dec.clone())} else {None},
                                Comparison::Lt => if lhs < c.val {Some(c.dec.clone())} else {None},
                            }
                        }
                        Rule::Decision(d) => Some(d.clone()),
                    };
                    match z {
                        Some(d) => Some(d),
                        None => new_decision,
                    }
            });

        /*println!(
            "Classify part {:?} with workflow {:?}, decision = {:?}",
            part, flows.get(flow), &opt_decision);*/

        match opt_decision {
            Some(Decision::SendTo(new_flow)) => Self::classify_one_part(part, &new_flow, flows),
            Some(Decision::Accept) => Decision::Accept,
            Some(Decision::Reject) => Decision::Reject,
            None => Decision::Accept,
        }
    }

    fn part_ranges_to_combinations(part: &PartRanges) -> usize {
        (part.x.1 - part.x.0 + 1) as usize *
        (part.m.1 - part.m.0 + 1) as usize *
        (part.a.1 - part.a.0 + 1) as usize *
        (part.s.1 - part.s.0 + 1) as usize
    }

    fn decision_to_combinations(decision: &Decision, part: &PartRanges, flows: &HashMap<String, Workflow>) -> usize {
        match decision {
            Decision::SendTo(new_flow) => Self::find_all_combinations(part, new_flow, flows),
            Decision::Accept => Self::part_ranges_to_combinations(part),
            Decision::Reject => 0,
        }
    }

    fn split_part_ranges(part:&PartRanges, par: char, by_val: usize) -> (PartRanges, PartRanges) {
        match par {
            'x' => ( PartRanges { x: (part.x.0, by_val - 1), ..*part.clone() }, PartRanges{ x: (by_val, part.x.1), ..*part.clone()}),
            'm' => ( PartRanges { m: (part.m.0, by_val - 1), ..*part.clone() }, PartRanges{ m: (by_val, part.m.1), ..*part.clone()}),
            'a' => ( PartRanges { a: (part.a.0, by_val - 1), ..*part.clone() }, PartRanges{ a: (by_val, part.a.1), ..*part.clone()}),
            's' => ( PartRanges { s: (part.s.0, by_val - 1), ..*part.clone() }, PartRanges{ s: (by_val, part.s.1), ..*part.clone()}),
            _ => unreachable!()
        }

    }
    // this recursive function processes the part ranges through the workflows and splits ranges where required
    // at the entry point ranges of all parameters are 1..4000
    // we start atthe workflow in and apply all conditions of it splitting range by criteria
    // for example: a<1234 will lead to split of range a into
    //   [1..1233] and [1234..4000]
    // part [1..1233] will be passed into the same workflow again
    // part [1234..4000] will be processed according to the decision of the rule (A, R, SendTo)
    fn find_all_combinations(part: &PartRanges, flow: &String, flows: &HashMap<String, Workflow>) -> usize {

        let rules = flows.get(flow).unwrap().clone().rules;

        let combinations: usize = rules
            .iter()
            .fold((Some(part), 0_usize), |(opt_part, acc_combinations), rule| {
                    match opt_part {
                        Some(part) =>
                            match rule {
                                Rule::Condition(c) => {
                                    let lhs = match c.par {
                                        'x' => part.x,
                                        'm' => part.m,
                                        'a' => part.a,
                                        's' => part.s,
                                        _ => unreachable!("pasrt name is not in XMAS list (condition:{:?})", &c)
                                    };
                                    match c.cmp {
                                        // if range fully satisfies condition then we follow the decision
                                        // but if range partially satisfies the condition then we split it into 2 subranges
                                        Comparison::Gt => {
                                            // range fully meets condition
                                            if lhs.0 > c.val {
                                                let new_combinations =
                                                Self::decision_to_combinations(&c.dec, part, flows);
                                                (None, acc_combinations + new_combinations)
                                            // range partially meets condition: split and process according to decision
                                            } else if lhs.1 > c.val {
                                                let (p1, p2) = Self::split_part_ranges(part, c.par, c.val + 1);
                                                let new_combinations =
                                                    Self::find_all_combinations(&p1, flow, flows) +
                                                    Self::decision_to_combinations(&c.dec, &p2, flows);
                                                (None, acc_combinations + new_combinations)
                                            // range doesn't meet condition - go to the rule
                                            } else {
                                                (Some(part), acc_combinations)
                                            }
                                        },
                                        Comparison::Lt => {
                                            // range fully meets condition
                                            if lhs.1 < c.val {
                                                let new_combinations =
                                                Self::decision_to_combinations(&c.dec, part, flows);
                                                (None, acc_combinations + new_combinations)
                                            // range partially meets condition: split and process according to decision
                                            } else if lhs.0 < c.val {
                                                let (p1, p2) = Self::split_part_ranges(part, c.par, c.val);
                                                let new_combinations =
                                                    Self::decision_to_combinations(&c.dec, &p1, flows) +
                                                    Self::find_all_combinations(&p2, flow, flows);
                                                (None, acc_combinations + new_combinations)
                                            // range doesn't meet condition - go to the rule
                                            } else {
                                                (Some(part), acc_combinations)
                                            }
                                        },
                                    }
                                },
                                Rule::Decision(d) => {
                                    let new_combinations = Self::decision_to_combinations(d, part, flows);
                                    (None, acc_combinations + new_combinations)
                                },
                            },
                        None => (None, acc_combinations)
                    }
            })
            .1;

        combinations

        /*println!(
            "Classify part {:?} with workflow {:?}, decision = {:?}",
            part, flows.get(flow), &opt_decision);*/

    }

}

pub struct DaySolution(P);

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 19;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let v: Vec<&str> = text_input.split("\n\n").collect();
        let flows_str = v[0];
        let parts_str = v[1];
        let parts: Vec<Part> = parts_str
            .lines()
            .map(|line| DaySolution::parse_one_part(line))
            .collect();
        let flows: HashMap<String, Workflow> = flows_str
            .lines()
            .map(|line| DaySolution::parse_one_workflow(line))
            .map(|wfw| (wfw.name.clone(), wfw))
            .collect()
            ;
        P { flows, parts }
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let Self::Problem { parts, flows } = problem;
        let start = String::from("in");
        let answer =
            parts
            .iter()
            .filter(|part| match DaySolution::classify_one_part(part, &start, &flows) {
                Decision::Accept => true,
                _ => false

            })
            .map(|part| part.x + part.m + part.a + part.s)
            .sum();
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let Self::Problem { parts: _, flows } = problem;
        let start = String::from("in");
        let part = PartRanges {x: (1,4000), m: (1,4000), a: (1, 4000), s: (1, 4000)};
        let answer = DaySolution::find_all_combinations(&part, &start, &flows);
        Some(answer)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}
