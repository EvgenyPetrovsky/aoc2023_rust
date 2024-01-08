use num_rational::Ratio;
use regex::Regex;
use std::{fmt, iter::successors, str::FromStr};

//const UNIT: Rational64 = Rational64::from(1_i64);
//const UNIT: Rational64 = 1_i64.into();
//const ZERO: Rational64 = UNIT * 0;

// Ratio<i64> is not enough for part 1 because denominators are getting very big
type R = Ratio<i128>;

type Time = R;

#[derive(Debug, Clone)]
struct Velocity(R, R, R);

#[derive(Debug, Clone)]
struct Location(R, R, R);

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
    location: Location,
}

type P = Vec<Particle>;

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Particle [{:>2}, {:>2}, {:>2} @ {:>2}, {:>2}, {:>2}]",
            self.loc.0, self.loc.1, self.loc.2, self.vel.0, self.vel.1, self.vel.2
        )
    }
}

// port
struct Spiral {
    dx: i64,
    dy: i64,
    segment_length: i64,
    x: i64,
    y: i64,
    segment_passed: i64,
}

impl Spiral {
    fn new() -> Self {
        Self {
            dx: 1,
            dy: 0,
            segment_length: 1,
            x: 0,
            y: 0,
            segment_passed: 0,
        }
    }

    fn pair_of_values(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    fn grow(&self) -> Option<Self> {
        let limit: i64 = 10_000;
        let debug = false;
        let &Spiral {
            dx: odx,
            dy: ody,
            segment_length: osl,
            x: ox,
            y: oy,
            segment_passed: osp,
        } = self;
        let x: i64 = ox + odx;
        let y: i64 = oy + ody;
        // once we reach the max length of segment we must do turn
        let segment_passed: i64 = if osl == osp + 1 { 0 } else { osp + 1 };
        let dx: i64 = if osl == osp + 1 { -ody } else { odx };
        let dy: i64 = if osl == osp + 1 { odx } else { ody };
        // increase the segment length only every second turn
        let segment_length: i64 = if dy == 0 && osl == osp + 1 {
            osl + 1
        } else {
            osl
        };

        if debug && segment_passed == 0 {
            println!("new segment_length: {}", segment_length);
        }
        if segment_length > limit {
            // something is wrong
            println!(
                "Stop growing the spiral after reaching limit size: {}",
                limit
            );
            None
        } else {
            Some(Spiral {
                x,
                y,
                segment_length,
                dx,
                dy,
                segment_passed,
            })
        }
    }
}

impl Particle {
    fn from(from_str: &str) -> Self {
        fn to_r64(str: &str) -> R {
            R::from_str(str).unwrap()
        }
        let re =
            Regex::new(r#"(-?\d+), +(-?\d+), +(-?\d+) +@ +(-?\d+), +(-?\d+), +(-?\d+)"#).unwrap();
        re.captures(from_str)
            .map(|c| {
                let (_, [x0, y0, z0, dx, dy, dz]) = c.extract();
                Particle {
                    loc: Location(to_r64(x0), to_r64(y0), to_r64(z0)),
                    vel: Velocity(to_r64(dx), to_r64(dy), to_r64(dz)),
                }
            })
            .unwrap()
    }

    // port
    fn hits(&self, other: &Self) -> bool {
        let debug = false;
        let unit = R::from(1);
        let zero = unit * 0;
        let Location(x0, y0, z0) = self.loc;
        let Velocity(vx, vy, vz) = self.vel;

        let Location(x0i, y0i, z0i) = other.loc;
        let Velocity(vxi, vyi, vzi) = other.vel;

        // If same velocity then they cannot hit, unless same initial position
        if ((x0i != x0) && (vx == vxi))
            || ((y0i != y0) && (vy == vyi))
            || ((z0i != z0) && (vz == vzi))
        {
            false
        } else {
            // Check the time of the hit in x, y and z.
            // The trajectories hit each other if all times are equal

            let tx: R = if x0i == x0 {
                zero
            } else {
                (x0i - x0) / (vx - vxi)
            };
            let ty = if y0i == y0 {
                zero
            } else {
                (y0i - y0) / (vy - vyi)
            };
            let tz = if z0i == z0 {
                zero
            } else {
                (z0i - z0) / (vz - vzi)
            };
            if debug {
                println!("Collision times: tx = {tx}, ty = {ty}, tz = {tz}");
            }
            //
            (tx == ty || tx == zero || ty == zero)
                && (tx == tz || tx == zero || tz == zero)
                && (ty == tz || ty == zero || tz == zero)
        }
    }

    fn location_at(&self, time: &Time) -> Location {
        let t = time;
        let Location(x0, y0, z0) = self.loc;
        let Velocity(dx, dy, dz) = self.vel;
        Location(x0 + (t * dx), y0 + (t * dy), z0 + (t * dz))
    }

    fn trj_cross_on_xy_with(&self, other_particle: &Particle) -> Option<Cross> {
        let zero = R::from(0);
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

        if det_m == zero {
            None
        } else {
            let time_1 = det_0 / det_m;
            let time_2 = det_1 / det_m;
            let location = self.location_at(&time_1);
            if debug {
                println!(
                    " - collision in place: ({:>.3}, {:>.3}), at time of p1: {:>.3}, at time of p2: {:>.3}",
                    location.0,
                    location.1,
                    time_1,
                    time_2
                );
            }
            Some(Cross {
                particle_1: self.clone(),
                particle_2: other_particle.clone(),
                time_1,
                time_2,
                location,
            })
        }
    }

}

pub struct DaySolution(P);

impl DaySolution {
    // port
    fn compute_perfect_shot(
        vx_vy_candidates: (R, R),
        trajectories: &Vec<Particle>,
    ) -> Option<Particle> {
        // Implementing the original idea by /u/UnicycleBloke:
        // https://www.reddit.com/r/adventofcode/comments/18q7d47/2023_day_24_part_2_a_mathematical_technique_for/keubuig/

        //println!("compute_perfect_shot for {vx_vy_candidates:?}");
        let unit = R::from(1);

        let (vx, vy) = vx_vy_candidates;
        //let debug = true || (vx == unit * -3) && (vy == unit);
        let debug = false;
        if debug {
            println!("find perfect match for (vx , vy) = ({vx}, {vy})");
        }
        let (hail1, hail2, rest_hails) = match &trajectories[..] {
            [h1, h2, tail @ ..] => (h1.clone(), h2.clone(), Vec::from(tail)),
            _ => panic!("trajectories have less than 2 elements!"),
        };

        // Use the first two hails as well as two guesses for vx and vy
        // to get the times at which the rock hits them
        let Location(x01, y01, z01) = hail1.loc;
        let Velocity(vx1, vy1, vz1) = hail1.vel;

        let Location(x02, y02, z02) = hail2.loc;
        let Velocity(vx2, vy2, vz2) = hail2.vel;

        //fn compute_t1t2(vx: i64, vy: i64) -> Option<(R, R)> {

        let compute_t1t2 = |vx: R, vy: R| {
            // The differences between the two hails' positions at times t1 and t2
            // are related such that:
            //      (x01 + vx1 * t1) - (x02 + vx2 * t2) = (x0 + vx * t1) - (x0 + vx * t2)
            //      (y01 + vy1 * t1) - (y02 + vy2 * t2) = (y0 + vy * t1) - (y0 + vy * t2)
            // Use these two equations to solve for t1 and t2 and you get the
            // following (expressed as num / den)
            let t1_den = vx * vy1 - vx * vy2 - vx1 * vy + vx1 * vy2 + vx2 * vy - vx2 * vy1;
            let t2_den = vx * vy1 - vx * vy2 - vx1 * vy + vx1 * vy2 + vx2 * vy - vx2 * vy1;

            if debug {
                println!("t1 denom = {t1_den}, t2 denom = {t2_den}");
            }

            // If any of the denominators are zero, there is no solution
            if (t1_den == unit * 0) || (t2_den == unit * 0) {
                None
            } else {
                let t1_num =
                    -vx * y01 + vx * y02 + vx2 * y01 - vx2 * y02 + vy * x01 - vy * x02 - vy2 * x01
                        + vy2 * x02;
                let t2_num =
                    -vx * y01 + vx * y02 + vx1 * y01 - vx1 * y02 + vy * x01 - vy * x02 - vy1 * x01
                        + vy1 * x02;

                let t1 = t1_num / t1_den;
                let t2 = t2_num / t2_den;

                // If t1 or t2 are negative, the rock would have hit the hail in the past,
                // so it is not valid
                if (t1 < R::from(0)) || (t2 < R::from(0)) {
                    None
                } else {
                    let res = (t1, t2);
                    if debug {
                        println!("compute_t1t2 = {res:?}");
                    }
                    Some(res)
                }
            }
        };

        //def computeVZ(t1: Rational, t2: Rational): Option[i64] = {
        let compute_vz = |t1: R, t2: R| {
            let num: R = (z01 + vz1 * t1) - (z02 + vz2 * t2);
            let den: R = t1 - t2;
            let result: R = num / den;

            // All of the components of the velocities must be integers
            // Otherwise, it is not a valid solution
            if result.is_integer() {
                let res = result;
                if debug {
                    println!("compute_vz = {res}");
                }
                Some(res)
            } else {
                None
            }
        };

        //def computeInitialPos(t1: Rational, vx: i64, vy: i64, vz: i64): Option[(i64, i64, i64)] = {
        let compute_initial_pos =
            |t1: R, vx: R, vy: R, vz: R| {
                let x0: R = x01 + (vx1 - vx) * t1;
                let y0: R = y01 + (vy1 - vy) * t1;
                let z0: R = z01 + (vz1 - vz) * t1;

                // All of the components of the initial position must be integers
                // Otherwise, it is not a valid solution
                if x0.is_integer() && y0.is_integer() && z0.is_integer() {
                    let res = Location(x0, y0, z0);
                    if debug {
                        println!("compute_initial_pos = {res:?}");
                    }
                    Some(res)
                } else {
                    None
                }
            };

        //fn _computePerfectShot(vx: i64, vy: i64): Option[Trajectory] = {
        let _compute_perfect_shot = |vx: R, vy: R| {
            /*
            compute_t1t2(vx, vy).flatMap({ case (t1, t2) =>
            compute_vz(t1, t2).flatMap(vz => {
            computeInitialPos(t1, vx, vy, vz).flatMap(initialPos => {
                val proposedTrajectory = new Trajectory(initialPos, (vx, vy, vz))

                if (rest_hails.forall(proposedTrajectory.hits(_))) Some(proposedTrajectory)
                else None
            }) }) })
            */
            compute_t1t2(vx, vy).and_then(|(t1, t2)| {
                compute_vz(t1, t2).and_then(|vz| {
                    compute_initial_pos(t1, vx, vy, vz).and_then(|location| {
                        let proposed_particle = Particle {
                            loc: location,
                            vel: Velocity(vx, vy, vz),
                        };
                        if debug {
                            println!("proposed particle = {proposed_particle:?}");
                            println!(
                                "test: p hits p1 = {}; p hits p2 = {}",
                                proposed_particle.hits(&hail1),
                                proposed_particle.hits(&hail2)
                            );
                        }
                        if rest_hails.iter().all(|other| proposed_particle.hits(other)) {
                            let res = proposed_particle;
                            if debug {
                                println!("_compute_perfect_shot = {res:?}");
                            }
                            Some(res)
                        } else {
                            None
                        }
                    })
                })
            })
        };

        _compute_perfect_shot(vx, vy)
    }
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

    type Answer = Option<i64>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        text_input.lines().map(Particle::from).collect()
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        let test = false;
        let debug = false;
        let unit = R::from(1);
        let zero = unit * 0;
        let (p_lst, p_mst) = if test {
            (unit * 7, unit * 27)
        } else {
            (
                R::from(200_000_000_000_000_i128), //unit * 200_000_000_000_000_i64,
                R::from(400_000_000_000_000_i128), //unit * 400_000_000_000_000_i64,
            )
        };

        let cnt = problem.len();
        let answer = (0..cnt)
            .flat_map(|i1| (i1 + 1..cnt).map(move |i2| (i1, i2)))
            .filter_map(|(idx1, idx2)| {
                let p1 = &problem[idx1];
                let p2 = &problem[idx2];
                p1.trj_cross_on_xy_with(p2)
            })
            .filter(
                |Cross {
                     particle_1: _,
                     particle_2: _,
                     time_1,
                     time_2,
                     location,
                 }| {
                    let &Location(x, y, _) = location;
                    *time_1 > zero
                        && *time_2 > zero
                        && p_lst <= x
                        && x <= p_mst
                        && p_lst <= y
                        && y <= p_mst
                },
            )
            .map(|c| {
                let Location(x, y, _) = c.location;

                if debug {
                    println!(
                        "{} at time 1 {} & {} at time 2 {} are crossing the place ({}, {})",
                        c.particle_1, c.time_1, c.particle_2, c.time_2, x, y
                    );
                }
            })
            .count();
        let answer: i64 = answer as i64;
        Some(answer)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        /*
        Approach
        take first 3 stones
        identify times at which they all can be hit by object with initial position and velocity
        z = z0 + dz*t1 = z'0 + dz'*t1
        y = y0 + dy*t1 = y'0 + dy'*t1
        x = x0 + dx*t1 = x'0 + dx'*t1

        z = z0 + dz*t2 = z'0 + dz'*t2
        y = y0 + dy*t2 = y'0 + dy'*t2
        x = x0 + dx*t2 = x'0 + dx'*t2

        z = z0 + dz*t3 = z'0 + dz'*t3
        y = y0 + dy*t3 = y'0 + dy'*t3
        x = x0 + dx*t3 = x'0 + dx'*t3

        but this is not a system of linear equations because dz*t1 are both unknown

        ==================

        after all struggle I surrendered and blindly ported the solution
        from https://gitlab.com/javierbg/aoc2023/-/blob/main/24/perfectShot.sc

        */

        successors(Some(Spiral::new()), |s| s.grow())
            .map(|s| {
                let (vx, vy) = s.pair_of_values();
                (R::from(vx as i128), R::from(vy as i128))
            })
            .filter_map(|vx_vy_candidates| {
                DaySolution::compute_perfect_shot(vx_vy_candidates, &problem)
            })
            .map(
                |Particle {
                     loc: Location(x, y, z),
                     vel: _,
                 }| (x + y + z).to_integer(),
            )
            .take(1)
            .nth(0)
            .map(|x| x as i64)

        // None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn spiral_grow() {
        let l: Vec<(i64, i64)> = successors(Some(Spiral::new()), |s| s.grow())
            .map(|s| s.pair_of_values())
            .collect();
        let r: Vec<(i64, i64)> = Vec::new();
        assert_eq!(l, r);
    }
    #[test]
    fn particle_hits() {
        let unit = R::from(1);
        let p = Particle {
            loc: Location(unit * 24, unit * 13, unit * 10),
            vel: Velocity(unit * -3, unit, unit * 2),
        };
        let p1 = Particle {
            loc: Location(unit * 19, unit * 13, unit * 30),
            vel: Velocity(unit * -2, unit, unit * -2),
        };
        assert_eq!(p.hits(&p1), true);
    }
}
