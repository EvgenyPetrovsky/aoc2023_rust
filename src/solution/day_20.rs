use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Signal {
    Lo,
    Hi,
}

#[derive(Debug, Clone)]
enum State {
    On,
    Off,
}

type Label = String;

#[derive(Debug)]
enum Module {
    FlipFlop {
        name: Label,
        state: State,
        input: Signal,
    },
    Conjunction {
        name: Label,
        inputs: HashMap<Label, Signal>,
    },
    Broadcaster {
        name: Label,
        input: Signal,
    },
    Button {
        name: Label,
    },
}

#[derive(Debug, Clone)]
struct Cable {
    from: Label,
    to: Label,
}

#[derive(Debug, Clone)]
struct Pulse {
    signal: Signal,
    cable: Cable,
}

pub struct Network {
    // hashmap of module names to module definitions
    modules: HashMap<Label, Module>,
    // hashmap of output modules to linked input modules
    cbl_map: HashMap<Label, Vec<Label>>,
}

type P = Network;

pub struct DaySolution(P);

impl DaySolution {
    // parse one input line with relation to module
    fn parse_one_line_module(line: &str, cbl_map: &HashMap<Label, Vec<Label>>) -> Module {
        let cables: Vec<Cable> = cbl_map
            .iter()
            .flat_map(|(from, tos)| {
                tos.iter().map(|to| Cable {
                    from: from.clone(),
                    to: to.clone(),
                })
            })
            .collect();
        Regex::new(r#"^([%&]?)([a-z]+) -> "#)
            .unwrap()
            .captures(line)
            .map(|c| {
                let (_, [module_type, name]) = c.extract();
                //println!("Module type: '{}', module name = '{}'", module_type, name);
                match (name, module_type) {
                    (_, "%") => Module::FlipFlop {
                        name: String::from(name),
                        state: State::Off,
                        input: Signal::Lo,
                    },
                    (_, "&") => Module::Conjunction {
                        name: String::from(name),
                        // rewrite to use HashMap<from, HashSet<to>>
                        inputs: cables
                            .iter()
                            .filter(|c| c.to == name)
                            .map(|c| (c.from.clone(), Signal::Lo))
                            .collect::<HashMap<Label, Signal>>(),
                    },
                    ("broadcaster", _) => Module::Broadcaster {
                        name: String::from("broadcaster"),
                        input: Signal::Lo,
                    },
                    _ => unreachable!(),
                }
            })
            .unwrap()
    }
    // parse the input line with relation to cables that connect modules
    fn parse_one_line_cable(line: &str) -> (Label, Vec<Label>) {
        let parts = line
            .split(" -> ")
            //.map(|s| String::from(s))
            .collect::<Vec<&str>>();
        let re_name = Regex::new(r#"\w+"#).unwrap();
        // from
        let from = re_name
            .captures(parts[0])
            .map(|c| c.get(0).unwrap().as_str())
            .map(String::from)
            .unwrap();
        // to
        let to: Vec<Label> = re_name
            .captures_iter(parts[1])
            .map(|c| c.get(0).unwrap().as_str())
            .map(String::from)
            .collect();

        (from, to)
    }

    fn push_button(network: Network) -> (Network, Vec<Pulse>) {
        let init: Vec<Pulse> = Self::init_pulse();
        let mut network: Network = network;
        let pulses: Vec<Pulse> = successors(Some(init), |pulses: &Vec<Pulse>| {
            if pulses.len() == 0 {
                None
            } else {
                let ems = network.excited_modules(&pulses);
                network.consume_pulses(pulses.clone());
                let new_pulses = network.emit_pulses(ems);
                Some(new_pulses)
            }
        })
        .flatten()
        .collect::<Vec<Pulse>>();
        (network, pulses)
    }

    fn init_pulse() -> Vec<Pulse> {
        let from = String::from("button");
        let to = String::from("broadcaster");
        let pulse = Pulse {
            cable: Cable { from, to },
            signal: Signal::Lo,
        };
        vec![pulse]
    }
}

impl State {
    fn flip(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl Module {
    /*
    extract module name
     */
    fn name(&self) -> String {
        match self {
            Module::Button { name }
            | Module::Broadcaster { name, input: _ }
            | Module::FlipFlop {
                name,
                state: _,
                input: _,
            }
            | Module::Conjunction { name, inputs: _ } => name.clone(),
        }
    }
    /*
    update input memory from modules. this is iportant step of signal consumption
    and transition into excited state updated input signals are used for emission of the new signals
    */
    fn update_input(&mut self, from: Label, signal: Signal) {
        match self {
            Module::FlipFlop {
                ref mut state,
                ref mut input,
                ..
            } => {
                if signal == Signal::Lo {
                    *state = state.flip();
                }
                *input = signal;
            }
            Module::Conjunction { ref mut inputs, .. } => {
                //inputs[&from] = signal;
                inputs.entry(from).and_modify(|v| *v = signal);
            }
            Module::Broadcaster { name: _, input: _ } => (),
            Module::Button { name: _ } => {
                unreachable!("Something is wrong! There must be no input for 'Button' module")
            }
        }
        /*
        *self = match self {
            Module::FlipFlop {
                name,
                state,
                input: _,
            } => Module::FlipFlop {
                name: name.clone(),
                state: match signal {
                    Signal::Lo => state.flip(),
                    _ => state.clone(),
                },
                input: signal,
            },
            Module::Conjunction { name, inputs } => {
                //let mut inputs = inputs;
                inputs.insert(from, signal);
                Module::Conjunction {
                    name: name.clone(),
                    inputs: inputs.clone(),
                }
            }
            Module::Broadcaster { name, input: _ } => Module::Broadcaster {
                name: name.clone(),
                input: signal,
            },
            Module::Button { name: _ } => {
                unreachable!("Something is wrong! There must be no input for 'Button' module")
            }
        };
        */
    }

    /*
    produce new pulse based on input states of excited module
    there can be no pulse at all, e.g.: -high-> &a
    */
    fn emit_signal(&self) -> Option<Signal> {
        match self {
            Self::Button { name: _ } => Some(Signal::Lo),
            Self::Broadcaster { name: _, input } => Some(input.clone()),
            Self::FlipFlop {
                name: _,
                state,
                input,
            } => match (input, state) {
                /* state is already updated on previous step, therefore
                State On must emit Hi signal,
                State Off must emit Lo signal */
                (Signal::Hi, _) => None,
                (Signal::Lo, State::Off) => Some(Signal::Lo),
                (Signal::Lo, State::On) => Some(Signal::Hi),
            },
            Self::Conjunction { name: _, inputs } => {
                if inputs.iter().all(|(_, signal)| signal == &Signal::Hi) {
                    Some(Signal::Lo)
                } else {
                    Some(Signal::Hi)
                }
            }
        }
    }
}

impl Network {
    /*
    network modules consumes pulses and updates input states of excited modules
    */
    fn consume_pulses(&mut self, pulses: Vec<Pulse>) {
        //let mut labels: HashSet<Label> = HashSet::new();
        pulses.into_iter().for_each(|Pulse { signal, cable }| {
            let Cable {
                from: emitter,
                to: consumer,
            } = cable;
            self.modules
                .entry(consumer)
                .and_modify(|module| module.update_input(emitter, signal));
        });
    }
    /*
    get list of excited modules based on pulses
    it is a question whether pulses to modules must be combined within one quant of time
    */
    fn excited_modules(&self, pulses: &Vec<Pulse>) -> Vec<Label> {
        let labels: Vec<Label> = pulses
            .iter()
            .map(|s| s.cable.to.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        //labels.sort();
        labels
    }
    /* in the network configuration for given modules that are defined as ixcited modules,
    issue the set of pulses towards next modules
    */
    fn emit_pulses(&self, excited_module_names: Vec<Label>) -> Vec<Pulse> {
        excited_module_names
            .iter()
            // get signals for modules
            .filter_map(|module_label| {
                if let Some(Some(signal)) = self.modules.get(module_label).map(|m| m.emit_signal())
                {
                    Some((module_label, signal))
                } else {
                    None
                }
            })
            .flat_map(|(from, signal)| {
                self.cbl_map.get(from).unwrap().iter().map(move |to| Pulse {
                    cable: Cable {
                        from: from.clone(),
                        to: to.clone(),
                    },
                    signal: signal.clone(),
                })
            })
            .collect()
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 20;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let mut cbl_map: HashMap<Label, Vec<Label>> = text_input
            .lines()
            .map(DaySolution::parse_one_line_cable)
            .collect();
        let mut modules = text_input
            .lines()
            .map(|line| DaySolution::parse_one_line_module(line, &cbl_map))
            .collect::<Vec<Module>>();

        modules.push(Module::Button {
            name: String::from("button"),
        });
        let modules: HashMap<String, Module> = modules.into_iter().map(|m| (m.name(), m)).collect();

        cbl_map.insert(
            String::from("button"),
            vec![String::from("broadcaster")]
        );

        Network { modules, cbl_map }
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let no_pulses: Vec<Pulse> = vec![];
        let network = problem;
        let n = 5;
        let debug = true;
        let (_, pulses) = (0..n).fold((network, no_pulses), |(network, mut pulses), n| {
            let (new_network, mut new_pulses) = DaySolution::push_button(network);
            // debugging
            if debug {
                println!("Step {:>4} pulses:", n + 1);
                new_pulses.iter().for_each(|p| {
                    println!("{:<11} -{:?}-> {:>11}", p.cable.from, p.signal, p.cable.to)
                });
                new_network
                    .modules
                    .iter()
                    .map(|(_, m)| m)
                    .filter(|m| match m {
                        Module::FlipFlop {
                            name: _,
                            state: _,
                            input: _,
                        } => true,
                        _ => false,
                    })
                    .for_each(|m| println!("{:?}", m));
                new_network
                    .modules
                    .iter()
                    .map(|(_, m)| m)
                    .filter(|m| match m {
                        Module::Conjunction { inputs: _, name: _ } => true,
                        _ => false,
                    })
                    .for_each(|m| println!("{:?}", m));
            }

            pulses.append(&mut new_pulses);
            (new_network, pulses)
        });
        let signals: Vec<Signal> = pulses.into_iter().map(|pulse| pulse.signal).collect();
        let lo_cnt = signals.iter().filter(|&x| x == &Signal::Lo).count();
        let hi_cnt = signals.iter().filter(|&x| x == &Signal::Hi).count();
        println!("Pushes count: {n:>4}, low signals: {lo_cnt:>6}, high signals: {hi_cnt:>6}");
        Some(lo_cnt * hi_cnt)
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
