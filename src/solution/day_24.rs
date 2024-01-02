use regex::Regex;

use super::day_19::Part;

type Time = i64;
struct Velocity (i64, i64, i64);
struct Location (i64, i64, i64);
pub struct Particle {loc: Location, vel: Velocity}

type P = Vec<Particle>;

impl Particle {
    fn location_at(&self, time: Time) -> Location {
        let t = time;
        let Location(x0, y0, z0) = self.loc;
        let Velocity(dx, dy, dz) = self.vel;
        Location(x0 + t * dx, y0 + t * dy, z0 + t * dz)
    }
    fn parse(from_str: &str) -> Self {
        fn to_i64(str: &str) -> i64 {
            str.parse::<i64>().unwrap()
        }
        let re = Regex::new(r#"(-?\d+), +(-?\d+), +(-?\d+) +@ +(-?\d+), +(-?\d+), +(-?\d+)"#).unwrap();
        re.captures(from_str).map(|c| {
            let(_,[x0, y0, z0, dx, dy, dz]) = c.extract();
            Particle {
                loc: Location (to_i64(x0), to_i64(y0), to_i64(z0)),
                vel: Velocity (to_i64(dx), to_i64(dy), to_i64(dz)),
            }
        })
        .unwrap()
    }
    fn crossess_with(&self, other_particle: &Particle) -> bool {
        unimplemented!()
    }
    fn crosses_within_bounds(&self, other_particle: &Particle, t1: Time, t2: Time) -> bool {
        unimplemented!()
    }
}
pub struct DaySolution(P);

impl DaySolution {

}

/*
x = -2 * t + 19
y =  1 * t + 13
z = -2 * t + 30

x = -1 * t + 18
y = -1 * t + 19
z = -2 * t + 22

===============

y = -x/2 + 19/2 + 13
y = x - 18 + 19

2x + 2 = -x + 19 + 26
3x = 43
x = 14.333
*/

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 24;

    type Answer = Option<i64>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input
            .lines()
            .map(Particle::parse)
            .collect()
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
            None => format!(""),
        }
    }
}
