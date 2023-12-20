use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Part {x: u32, m: u32, a: u32, s: u32}

#[derive(Debug, Clone)]
pub struct Workflow { name: String, rules: Vec<Rule> }

#[derive(Debug, Clone)]
enum Rule { Condition(Condition), Decision(Decision) }

#[derive(Debug, Clone)]
struct Condition { par: char, cmp: Comparison, val: u32, dec: Decision }

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
                        val: val.parse::<u32>().unwrap(),
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
                x: x.parse::<u32>().unwrap(),
                m: m.parse::<u32>().unwrap(),
                a: a.parse::<u32>().unwrap(),
                s: s.parse::<u32>().unwrap(),
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

}

pub struct DaySolution(P);

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 19;

    type Answer = Option<u32>;
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
