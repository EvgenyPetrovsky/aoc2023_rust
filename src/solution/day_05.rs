use regex::Regex;
use std::collections::HashMap;

use crate::Part;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rng {start: u64, len: u64}

#[derive(Debug, Clone)]
struct Mapping {dest_start: u64, src_rng: Rng}


pub struct P {
    seeds: Vec<Rng>,
    maps:  HashMap<String, Vec<Mapping>>,
}

pub struct DaySolution(P);

impl DaySolution {

    fn extract_seeds(part: Part, line: &str) -> Vec<Rng> {
        match part {
            Part::One =>
                Regex::new(r#"\d+"#)
                .unwrap()
                .captures_iter(line)
                .map(|c| {
                    let start = c.get(0).unwrap().as_str().parse::<u64>().unwrap();
                    let len   = 1;
                    Rng{ start, len }
                })
                .collect(),
            Part::Two =>
                Regex::new(r#"\d+ \d+"#)
                .unwrap()
                .captures_iter(line)
                .map(|c| {
                    let start = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let len   = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    Rng{ start, len }
                })
                .collect(),
        }
    }

    fn parse_one_mapping_line(line: &str) -> Mapping {
        Regex::new(r#"(\d+) (\d+) (\d+)"#).unwrap()
        .captures(line)
        .map(|c| Mapping{
            dest_start: c.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            src_rng: Rng {
                start:  c.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                len:    c.get(3).unwrap().as_str().parse::<u64>().unwrap(),
            },
        })
        .expect(format!("Can't parse the string '{}' into mapping", line).as_str())
    }

    fn fill_in_missing_mapping_ranges(mappings: Vec<Mapping>) -> Vec<Mapping> {
        let rng_min = u64::MIN;
        let rng_max = u64::MAX;
        // collect starting points for existing mappings, add MAX of data type
        let starts: Vec<u64> = mappings.iter()
            .filter(|m| m.src_rng.len > 0)
            .map(|r| r.src_rng.start)
            .chain([rng_max])
            .collect();
        // find starting points for missing ranges, add MIN of data type
        let voids : Vec<u64> = mappings
            .iter()
            .map(|r| r.src_rng.start + r.src_rng.len)
            .chain([rng_min])
            .chain(starts.clone())
            .filter(|e| !starts.contains(e))
            .collect();
        // new mappings should map source value 1 : 1 for those ranges that were missing in initial mapping
        let complements: Vec<Mapping> = voids
            .iter()
            .map(|x| {
                let add_beg = x;
                let add_end = starts.iter().filter(|&x1| x1 > add_beg).min().unwrap();
                Mapping{
                    dest_start: *add_beg,
                    src_rng: Rng{start: *add_beg, len: *add_end - *add_beg}
                }
            })
            .filter(|r| r.src_rng.len > 0)
            .collect();
        // combine initial mappings plus new mappings for missing ranges
        mappings.iter().chain(complements.iter()).map(|x| x.clone()).collect()


    }

    fn parse_mapping_lines(text: &str) -> Vec<Mapping> {
        let mappings: Vec<Mapping> =
            text
            .lines()
            .filter(|l| !l.is_empty())
            .map(Self::parse_one_mapping_line)
            .collect();
        Self::fill_in_missing_mapping_ranges(mappings)
    }

    fn parse_input(part: Part, text_input: String) -> P {
        let re_seeds = Regex::new(r#"seeds: ([\d ])+"#).unwrap();
        let seeds: Vec<Rng> = re_seeds
            .captures(&text_input)
            .map(|c| Self::extract_seeds(part, c.get(0).unwrap().as_str()))
            .unwrap();
        let re_maps = Regex::new(r#"seed-to-soil map:([\d \n]*)soil-to-fertilizer map:([0-9 \n]*)fertilizer-to-water map:([0-9 \n]*)water-to-light map:([0-9 \n]*)light-to-temperature map:([0-9 \n]*)temperature-to-humidity map:([0-9 \n]*)humidity-to-location map:([0-9 \n]*)"#).unwrap();
        let maps: HashMap<String, Vec<Mapping>> = re_maps
            .captures(&text_input)
            .map(|c| [
                ("seed-to-soil",            Self::parse_mapping_lines(c.get(1).unwrap().as_str())),
                ("soil-to-fertilizer",      Self::parse_mapping_lines(c.get(2).unwrap().as_str())),
                ("fertilizer-to-water",     Self::parse_mapping_lines(c.get(3).unwrap().as_str())),
                ("water-to-light",          Self::parse_mapping_lines(c.get(4).unwrap().as_str())),
                ("light-to-temperature",    Self::parse_mapping_lines(c.get(5).unwrap().as_str())),
                ("temperature-to-humidity", Self::parse_mapping_lines(c.get(6).unwrap().as_str())),
                ("humidity-to-location",    Self::parse_mapping_lines(c.get(7).unwrap().as_str())),
            ])
            .unwrap()
            .iter()
            .map(|(k, v)| (String::from(*k), v.clone()))
            .collect();
        P{ seeds, maps }
    }

    /*
        Take the range and pass it trhough all mappings by splitting the range
        when it is not fully included into mapping
    */
    fn map_once(seed_rng: &Rng, mappings: &Vec<Mapping>) -> Vec<Rng> {
        let (s_beg, s_end) = (seed_rng.start, seed_rng.start + seed_rng.len);

        mappings
            .iter()
            .filter_map(|m| {
                // beg - inclusive, end - exclusive
                let (m_beg, m_end) = (m.src_rng.start, m.src_rng.start + m.src_rng.len);
                let dest = m.dest_start;
                // {seed_rng} to the left  from [mapping]: __{____}_[____]__
                // {seed_rng} to the right from [mapping]: __[____]_{____}__
                if s_end <= m_beg || s_beg > m_end {
                    None
                }
                // right part {seed_rng} inside [mapping]: __{__[**}__]__
                else if s_beg <= m_beg && m_beg < s_end && s_end <= m_end {
                    Some(Rng { start: dest + (m_beg - s_beg), len: s_end - m_beg }) }
                // whole      {seed_rng} inside [mapping]: __[__{**}__]__
                else if m_beg <= s_beg && s_end <= m_end {
                    Some(Rng { start: dest + (s_beg - m_beg), len: s_end - s_beg })}
                // left  part {seed_rng} inside [mapping]: __[__{**]__}__
                else if m_beg <= s_beg && s_beg < m_end && m_end <= s_end {
                    Some(Rng { start: dest + (s_beg - m_beg), len: m_end - s_beg }) }
                // {seed_rng} fully includes [mapping]:    __{__[**]__}__
                else if s_beg <= m_beg && m_end <= s_end {
                    Some(Rng { start: dest + (m_beg - s_beg), len: m_end - m_beg }) }
                // :-O unknown case
                else {panic!("unknown case! seed: {:?}, mapping: {:?}", seed_rng, m)}
            })
            .collect()

    }
    fn map_though_all(seed_rng: Rng, mappings_set: &HashMap<String, Vec<Mapping>>) -> u64 {
        let init = vec![seed_rng];
        let mapping_cats = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location",];
        mapping_cats
        .iter()
        .fold(init, |z, &x| {
            z.iter()
            .flat_map(|rng| Self::map_once(rng, &mappings_set[x]))
            .collect()
        })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
    }

}

impl super::Solution for DaySolution {

    const DAY_NUMBER: u8 = 5;

    type Answer = Option<u64>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        DaySolution::parse_input(Part::One, text_input)
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        DaySolution::parse_input(Part::Two, text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        //fn compose<T, U, V>(f: (T -> U), g: (U -> V)) -> (T -> V) {|x| g(f(x))};
        problem.seeds
        .iter()
        .map(|&r| DaySolution::map_though_all(r, &problem.maps))
        .min()
    }

    fn solve_part_2(_problem: Self::Problem) -> Self::Answer {
        /*
        Okay, we need to update solution so it operates ranges.
        every time when range passed into mapping list of ranges is returned: (num_rng, Mapping) -> [num_rng]
        each range from returned list is mapped trhough next mapping
         */
        None
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!("")
        }
    }
}
