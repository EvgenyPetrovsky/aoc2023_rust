use std::{cmp::Ordering, collections::HashMap};

const TILE_TRAIL: u8 = b'.';
const TILE_TREES: u8 = b'#';
const TILE_SL_LT: u8 = b'<';
const TILE_SL_RT: u8 = b'>';
const TILE_SL_UP: u8 = b'^';
const TILE_SL_DN: u8 = b'v';

type Tile = u8;

// Meaning of indices in Location: .0 is row, .1 is column
type Location = (usize, usize);

#[derive(Clone)]
struct Path(Vec<Location>);

pub struct HikingMap {
    tiles: Vec<Vec<Tile>>,
    size: Location,
}

type Distance = usize;
type LongestHikes = HashMap<Location, Distance>;

type P = HikingMap;

pub struct DaySolution(P);

impl Path {
    fn current_location(&self) -> Location {
        // last element of the path
        self.0.last().unwrap().clone()
    }

    fn visited_location(&self, location: &Location) -> bool {
        self.0.contains(location)
    }

    fn length(&self) -> Distance {
        self.0.len()
    }
    /*
    extend the path to new location

    there is no check for the validity of location (adjacent, not a forest)
    those checks must be done before
    */
    fn extend_to(&self, location: &Location) -> Self {
        let mut new_path: Self = self.clone();
        new_path.0.push(location.clone());
        new_path
    }

    // if this path is longer than anything registered in the history
    // or it is not in the history at all
    fn compare_to_longest(&self, history: &LongestHikes) -> Ordering {
        let location = self.current_location();
        let path_len = self.0.len();
        let hist_len = history.get(&location).unwrap_or(&0);
        path_len.cmp(hist_len)
    }
}

impl HikingMap {
    // find location of the start tile
    fn find_start_location(&self) -> Location {
        let r = 0;
        let c = self.tiles[r]
            .iter()
            .position(|tile| tile == &TILE_TRAIL)
            .unwrap();
        (r, c)
    }

    // find location of the start tile
    fn find_finish_location(&self) -> Location {
        let (rows, _) = self.size;
        let r = rows - 1;
        let c = self.tiles[r]
            .iter()
            .position(|tile| tile == &TILE_TRAIL)
            .unwrap();
        (r, c)
    }

    // all locations, but validated for map bounds
    fn adjacent_locations(&self, of: &Location) -> Vec<Location> {
        let (rows, cols) = self.size;
        let (r, c) = *of;
        let (rows, cols, r, c) = (rows as i32, cols as i32, r as i32, c as i32);

        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|(r, c)| 0 <= *r && *r < rows && 0 <= *c && *c < cols)
            .map(|(r, c)| (r as usize, c as usize))
            .collect()
    }

    // check if move can be done 'from' one location 'to' another
    fn valid_move(&self, from: &Location, to: &Location) -> bool {
        let (r0, c0) = from.clone();
        let (r1, c1) = to.clone();
        let tile = self.tile(to);
        // we must not come to start (that is also checked by visited locations od path)
        if tile == TILE_TRAIL {
            true
        }
        // we must not move to forest tile
        else if tile == TILE_TREES {
            false
        }
        // we must not climb icy slopes
        else if tile == TILE_SL_RT && c1 < c0
            || tile == TILE_SL_LT && c1 > c0
            || tile == TILE_SL_UP && r1 > r0
            || tile == TILE_SL_DN && r1 < r0
        {
            false
        }
        // all other slope combinations are valid
        else {
            true
        }
    }

    // check if you are on the slope and must slide
    fn stand_on_slope(&self, location: &Location) -> bool {
        match self.tile(location) {
            TILE_SL_DN | TILE_SL_UP | TILE_SL_RT | TILE_SL_LT => true,
            _ => false,
        }
    }

    // if location is slope then slide from it until it is tile
    // it must be BAD function because it doesn't register locations in the path!
    fn slide_from_slope(&self, location: &Location) -> Location {
        let (r, c) = (location.0, location.1);
        let tile = self.tile(location);

        match tile {
            TILE_SL_DN => (r + 1, c + 0),
            TILE_SL_UP => (r - 1, c + 0),
            TILE_SL_RT => (r + 0, c + 1),
            TILE_SL_LT => (r + 0, c - 1),
            _ => panic!("unexpected tile {} in location ({}, {})", tile, r, c),
        }
    }

    fn tile(&self, location: &Location) -> Tile {
        let (r, c) = location;
        self.tiles[*r][*c]
    }
}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 23;

    type Answer = Option<usize>;
    type Problem = P;

    fn parse_input_part_1(text_input: String) -> Self::Problem {
        let tiles: Vec<Vec<Tile>> = text_input
            .lines()
            .map(|l| l.as_bytes().iter().map(|b| *b).collect::<Vec<Tile>>())
            .collect();
        let size = if tiles.len() == 0 {
            (0, 0)
        } else {
            (tiles.len(), tiles[0].len())
        };
        HikingMap { tiles, size }
    }

    fn parse_input_part_2(text_input: String) -> Self::Problem {
        Self::parse_input_part_1(text_input)
    }

    fn solve_part_1(problem: Self::Problem) -> Self::Answer {
        /*
        initial path consists only from start.
        cycle for every path in consideration:
            if you stand on the slope slide from it to the next tile
            else discover all valid adjacent moves for current position (last location of path)
            validate that it was not visited in this path
            check what was the longest path in history
            if this path is longer, or no history is found then update the history of longest hikes keep moving for this path
            repeat the iteration for all paths
        */
        let start: Location = problem.find_start_location();
        let finish: Location = problem.find_finish_location();
        //println!("Start: {:?}. Finish: {:?}", start, finish);
        let init_path: Path = Path(vec![start]);
        let history: LongestHikes = HashMap::new();

        fn iterate(hmap: &HikingMap, history: LongestHikes, paths: Vec<Path>) -> LongestHikes {
            let init_new_paths: Vec<Path> = Vec::new();
            let (new_history, new_paths) =
                paths
                    .iter()
                    .fold((history, init_new_paths), |(history, nps), p| {
                        let location = p.current_location();
                        let new_locations: Vec<Location> = if hmap.stand_on_slope(&location) {
                            vec![hmap.slide_from_slope(&location)]
                        } else {
                            hmap.adjacent_locations(&location)
                                .iter()
                                .filter(|to| hmap.valid_move(&location, to))
                                .filter(|location| !p.visited_location(location))
                                .map(|l| l.clone())
                                .collect()
                        };
                        let new_paths: Vec<Path> = new_locations
                            .into_iter()
                            .map(|location| p.extend_to(&location))
                            .filter(|np| np.compare_to_longest(&history) == Ordering::Greater)
                            .collect();

                        let mut new_history = history;
                        new_paths.iter().for_each(|p| {
                            new_history.insert(p.current_location(), p.length());
                        });

                        let mut new_paths = new_paths
                            .into_iter()
                            .filter(|p| p.compare_to_longest(&new_history) == Ordering::Equal)
                            .collect::<Vec<Path>>();

                        new_paths.append(&mut nps.clone());

                        (new_history, new_paths)
                    });
            /*
            println!(
                "Number of new paths: {}, number of history records: {}",
                new_paths.len(),
                new_history.len()
            );
            */
            if new_paths.len() == 0 {
                //println!("History:\n{:?}", &new_history);
                new_history
            } else {
                iterate(hmap, new_history, new_paths)
            }
        }

        let longest_hikes = iterate(&problem, history, vec![init_path]);

        //let answer = longest_hikes.get(&finish).map(|v| *v).unwrap();
        longest_hikes.get(&finish).map(|answer| *answer - 1)
    }

    fn solve_part_2(problem: Self::Problem) -> Self::Answer {
        let start: Location = problem.find_start_location();
        let finish: Location = problem.find_finish_location();
        //println!("Start: {:?}. Finish: {:?}", start, finish);
        let init_path: Path = Path(vec![start]);
        let history: LongestHikes = HashMap::new();

        fn iterate(hmap: &HikingMap, history: LongestHikes, path: Path, finish: &Location) -> LongestHikes {
            /*
            BFS does not work, let's try DFS
            */

            let location = path.current_location();

            // first update the history
            let mut new_history = history;

            if path.compare_to_longest(&new_history) == Ordering::Greater {
                let length = path.length();
                if &location == finish {
                    println!("Update length for finish location. New value: {}", length);
                }
                new_history.insert(path.current_location(), length);
            }

            // extend paths from current location
            let new_paths: Vec<Path> = hmap
                .adjacent_locations(&location)
                .iter()
                .filter(|to| hmap.tile(to) != TILE_TREES)
                .filter(|location| !path.visited_location(location))
                .map(|location| path.extend_to(&location))
                .collect();

            // calculate further for each path 1 after 1 updating history
            let new_history: LongestHikes = new_paths
                .into_iter()
                .fold(new_history, | history, path| {
                    iterate(hmap, history, path, finish)
                });

            new_history

        }

        let longest_hikes = iterate(&problem, history, init_path, &finish);

        //let answer = longest_hikes.get(&finish).map(|v| *v).unwrap();
        longest_hikes.get(&finish).map(|answer| *answer - 1)
    }

    fn show_answer(answer: Self::Answer) -> String {
        match answer {
            Some(value) => format!("{}", value),
            None => format!(""),
        }
    }
}
