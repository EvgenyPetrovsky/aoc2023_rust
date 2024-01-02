use regex::Regex;
use std::fmt;

const LST: f64 = 200_000_000_000_000.;
const MST: f64 = 400_000_000_000_000.;
//const LST: f64 = 7.;
//const MST: f64 = 27.;

type Time = f64;

#[derive(Debug, Clone)]
struct Velocity(f64, f64, f64);

#[derive(Debug, Clone)]
struct Location(f64, f64, f64);

#[derive(Debug, Clone)]
pub struct Particle {
    loc: Location,
    vel: Velocity,
}

pub struct Cross {
    particle_1: Particle,
    particle_2: Particle,
    time_1: Time,
    time_2: Time,
}
type P = Vec<Particle>;

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Particle [{:>3}, {:>3}, {:>3} @ {:>3}, {:>3}, {:>3}]",
            self.loc.0, self.loc.1, self.loc.2, self.vel.0, self.vel.1, self.vel.2
        )
    }
}

impl Particle {
    fn parse(from_str: &str) -> Self {
        fn to_f64(str: &str) -> f64 {
            str.parse::<f64>().unwrap()
        }
        let re =
            Regex::new(r#"(-?\d+), +(-?\d+), +(-?\d+) +@ +(-?\d+), +(-?\d+), +(-?\d+)"#).unwrap();
        re.captures(from_str)
            .map(|c| {
                let (_, [x0, y0, z0, dx, dy, dz]) = c.extract();
                Particle {
                    loc: Location(to_f64(x0), to_f64(y0), to_f64(z0)),
                    vel: Velocity(to_f64(dx), to_f64(dy), to_f64(dz)),
                }
            })
            .unwrap()
    }

    fn location_on_xy_at(&self, time: &Time) -> Location {
        let t = time;
        let Location(x0, y0, z0) = self.loc;
        let Velocity(dx, dy, dz) = self.vel;
        Location(x0 + t * dx, y0 + t * dy, z0 + t * dz)
    }

    fn _crossess_on_xy_with(&self, other_particle: &Particle) -> bool {
        match self.cross_on_xy_with(other_particle) {
            None => false,
            _ => true,
        }
    }
    fn cross_on_xy_with(&self, other_particle: &Particle) -> Option<Cross> {
        let that = other_particle;
        let debug = false;
        if debug {
            println!("Collide {} with {}", self, that);
        }
        let a = vec![self.vel.0, -that.vel.0, self.vel.1, -that.vel.1];
        let b = vec![that.loc.0 - self.loc.0, that.loc.1 - self.loc.1];

        let det_m = a[0] * a[3] - a[1] * a[2];
        let det_0 = b[0] * a[3] - a[1] * b[1];
        let det_1 = a[0] * b[1] - b[0] * a[2];

        if det_m == 0. {
            None
        } else {
            let time_1 = det_0 / det_m;
            let time_2 = det_1 / det_m;
            if debug {
                println!(
                    " - collision in place: ({:>.3}, {:>.3}), at time of p1: {:>.3}, at time of p2: {:>.3}",
                    self.loc.0 + self.vel.0 * time_1,
                    self.loc.1 + self.vel.1 * time_1,
                    time_1,
                    time_2
                );
            }
            Some(Cross {
                particle_1: self.clone(),
                particle_2: other_particle.clone(),
                time_1,
                time_2,
            })
        }
    }
}
pub struct DaySolution(P);

impl DaySolution {}

/*
    x = -2 * t1 + 19
    y =  1 * t1 + 13
    z = -2 * t1 + 30

    x = -1 * t2 + 18
    y = -1 * t2 + 19
    z = -2 * t2 + 22

    ===============

    -2 * t1 + 19 = -1 * t2 + 18
    1 * t1 + 13 = -1 * t2 + 19
    -2 * t1 + 30 = -2 * t2 + 22

    ===============

    | -2 , 1 |   | t1 |    | 19 - 18 |
    |  1 , 1 | * | t2 |  = | 13 - 19 |
    | -2 , 2 |   |    |    | 30 - 22 |

    | -2 , 1 |   | t1 |    | 1 |
    | -1 , 3 | * | t2 |  = | 2 |

    ===============

    ΔA = -6 + 1 = 7
    Δ1 = 3 - 2 = 1
    Δ2 = -4 + 1 = -3

    t1 = 1 / 7
    t2 = -3 / 7



    y = -x/2 + 19/2 + 13
    y = x - 18 + 19

    2x + 2 = -x + 19 + 26
    3x = 43
    x = 14.333
*/

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 24;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input.lines().map(Particle::parse).collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let pi = problem.iter();
        let pi_1 = pi.clone();
        let answer = pi_1
            .clone()
            .enumerate()
            .flat_map(|(idx1, p1)| {
                let pi_2 = pi.clone().enumerate().filter(move |(idx2, _)| *idx2 > idx1);
                pi_2.clone()
                    .filter_map(move |(_, p2)| p1.cross_on_xy_with(p2))
            })
            .filter(
                |Cross {
                     particle_1,
                     particle_2: _,
                     time_1,
                     time_2,
                 }| {
                    let Location(x, y, _) = particle_1.location_on_xy_at(time_1);
                    *time_1 > 0. && *time_2 > 0. && LST <= x && x <= MST && LST <= y && y <= MST
                },
            )
            .map(|c| {
                let Location(x, y, _) = c.particle_1.location_on_xy_at(&c.time_1);
                println!(
                    "{} at time 1 {:>.3} & {} at time 2 {:>.3} are crossing the place ({:>.3}, {:>.3})",
                    c.particle_1, c.time_1, c.particle_2, c.time_2, x, y
                );
            })
            .count();
        Some(answer)
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
