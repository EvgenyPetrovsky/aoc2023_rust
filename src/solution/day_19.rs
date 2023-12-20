use std::collections::HashMap;
use regex::Regex;

pub struct Part {x: u32, m: u32, a: u32, s: u32}

pub struct Workflow { name: String, rules: Vec<Rule> }

enum Rule { Condition(Condition), Decision(Decision) }

struct Condition { par: char, cmp: Comparison, val: u32, dec: Decision }

enum Decision { Accept, Reject, SendTo (String) }

enum Comparison { Lt, Gt }


type P = (HashMap<String, Workflow>, Vec<Part>);

impl DaySolution {

    fn parse_one_rule (line: &str) -> Rule {
        let re_condition = Regex::new(r#"^([xmas])([\<\>])(\d+):([AR]|[a-z]+)$"#).unwrap();
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
            .map(|c| {
            let (_, [par, cmp, val, dec]) = c.extract();
                Condition {
                    par: par.chars().nth(0).unwrap(),
                    cmp: if cmp == "<" {Comparison::Lt} else {Comparison::Gt},
                    val: val.parse::<u32>().unwrap(),
                    dec: match_decision(dec),
                }
        });
        let decision =
            re_decision
            .captures(line)
            .map(|c| {
                let (_, [dec]) = c.extract();
                match_decision(dec)
            });

        if condition.is_some() {
            Rule::Condition(condition.unwrap())
        } else {
            Rule::Decision(decision.unwrap())
        }

    }
    fn parse_one_workflow(line: &str) -> Workflow {
        let main_re = Regex::new(r#"(\w+)\{([a-zAR\<\>\:,]+)\}"#).unwrap();
        let rules_re = Regex::new(r#"[0-9a-zAR\<\>\:]+"#).unwrap();
        let (name, rules_str) =
        main_re.captures(line).map(|c| {
            let(_, [name, rules]) = c.extract();
            (String::from(name), rules)
        })
        .unwrap();
        let rules =
            rules_re.captures_iter(rules_str).map(|c| {
                let (rule, [_]) = c.extract();
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
}

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
