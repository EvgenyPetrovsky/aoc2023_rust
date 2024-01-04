use std::{collections::HashMap, thread::panicking};

const TILE_START: u8 = b'S';
const TILE_TRAIL: u8 = b'.';
const TILE_TREES: u8 = b'#';
const TILE_SL_LT: u8 = b'>';
const TILE_SL_RT: u8 = b'<';
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
        unimplemented!()
    }

    fn visited_location(&self, location: &Location) -> bool {
        unimplemented!()
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
    fn is_longest_path(&self, history: LongestHikes) -> bool {
        let location = self.current_location();
        let path_len = self.0.len();
        let hist_len = history.get(&location).unwrap_or(&0);
        path_len > *hist_len

    }

}

impl HikingMap {

    // all locations, but validated for map bounds
    fn adjacent_locations(&self, of: &Location) -> Vec<Location> {
        unimplemented!()
    }

    // check if move can be done 'from' one location 'to' another
    fn valid_move(&self, from: &Location, to: &Location) -> bool {
        let (r0, c0) = from.clone();
        let (r1, c1) = to.clone();
        let tile = self.tiles[r1][c1];
        // we must not come to start (that is also checked by visited locations od path)
        if tile == TILE_START {false}
        // we must not move to forest tile
        else if tile == TILE_TREES {false}
        // we must not climb icy slopes
        else if tile == TILE_SL_RT && c1 < c0 {false}
        else if tile == TILE_SL_LT && c1 > c0 {false}
        else if tile == TILE_SL_UP && r1 > r0 {false}
        else if tile == TILE_SL_DN && r1 < r0 {false}
        else {true}
    }

    // check if you are on the slope and must slide
    fn stand_on_slope(&self, location: &Location) -> bool {
        match self.tiles[location.0][location.1] {
            TILE_SL_DN | TILE_SL_UP| TILE_SL_RT| TILE_SL_LT => true,
            _ => false
        }
    }

    // if location is slope then slide from it until it is tile
    // it must be BAD function because it doesn't register locations in the path!
    fn slide_from_slope(&self, location: &Location) -> Location {
        let (r, c) = (location.0, location.1);
        let tile = self.tiles[r][c];

        match tile {
            TILE_TRAIL => (r, c),
            TILE_SL_DN => (r+1, c+0),
            TILE_SL_UP => (r-1, c+0),
            TILE_SL_RT => (r+0, c+1),
            TILE_SL_LT => (r+0, c-1),
            _ => panic!("unexpected tile {} in location ({}, {})", tile, r, c)
        }
    }


}

impl super::Solution for DaySolution {
    const DAY_NUMBER: u8 = 23;

    type Answer = Option<i32>;
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
        HikingMap{tiles, size}
    }

    fn parse_input_part_2(_text_input: String) -> Self::Problem {
        Self::parse_input_part_1(_text_input)
    }

    fn solve_part_1(_problem: Self::Problem) -> Self::Answer {

        /*
        initial path from start.
        cycle for every path in consideration:
            if you stand on the slope slide from it to the next tile
            else discover all valid adjacent moves for current position (last location of path)
            validate that it was not visited in this path
            check what was the longest path in history
            if this path is longer, or no history is found then update the history of longest hikes keep moving for this path
            repeat the iteration for all paths
        */

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
