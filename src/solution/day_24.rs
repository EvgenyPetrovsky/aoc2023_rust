use regex::Regex;

const LST:i64 = 200_000_000_000_000;
const MST:i64 = 400_000_000_000_000;

type Time = i64;

type CTime = f64;

#[derive(Debug)]
struct Velocity (i64, i64, i64);

#[derive(Debug)]
struct Location (i64, i64, i64);

#[derive(Debug)]
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

    fn when_crossess_on_xyz_with(&self, other_particle: &Particle) -> Option<CTime> {
        unimplemented!()
    }


    fn crossess_on_xy_with(&self, other_particle: &Particle) -> bool {
        match self.when_crossess_on_xy_with(other_particle) {
            None => false,
            _ => true
        }
    }

    fn when_crossess_on_xy_with(&self, other_particle: &Particle) -> Option<CTime> {
        let that = other_particle;
        println!("collide {:?} with {:?}", self, that);
        let a = vec!(
            self.vel.0, -that.vel.0,
            self.vel.1, -that.vel.1
        );
        let b = vec!(
            that.loc.0 - self.loc.0,
            that.loc.1 - self.loc.1
        );

        let det_m = a[0] * a[3] - a[1] * a[2];
        let det_0 = b[0] * a[3] - a[1] * b[1];
        //let det_1 = a[0] * b[1] - b[0] * a[2];

        if det_m == 0 {
            None
        } else {
            let (d, n) = (det_0 as f64, det_m as f64);
            let t = d / n;
            println!("Collision at time {}", t);
            Some(t)
        }

    }
}
pub struct DaySolution(P);

impl DaySolution {

}

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
        text_input
            .lines()
            .map(Particle::parse)
            .collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let pi_1 = problem.iter();
        let answer =
            pi_1
            .clone()
            .enumerate()
            .flat_map(|(idx, p1)|{
                let pi_2 = pi_1.clone().skip(idx + 1);
                pi_2.
                    filter_map(|p2| p1.when_crossess_on_xy_with(p2))
            })
            .filter(|t| (LST as f64) <= *t && *t <= (MST as f64))
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
