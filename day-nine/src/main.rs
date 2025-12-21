//! Command line executable for running part one and part two
use std::{cmp::Ordering, time::Instant};

use clap::Parser;
use nalgebra::DMatrix;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short)]
    input_file: String,

    #[command(subcommand)]
    part: Part,
}

#[derive(clap::Subcommand, Debug)]
enum Part {
    Part1,
    Part2,
}

fn main() {
    let args = Args::parse();

    // Read to a string
    let s = std::fs::read_to_string(args.input_file).expect("Failed to read file");

    let start = Instant::now();
    let answer = match args.part {
        Part::Part1 => part_one(&s),
        Part::Part2 => part_two(&s),
    };

    println!("{:?}", answer);
    println!("Completed in {:?}", start.elapsed());
}

/// Location
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}
impl Location {
    pub fn new(line: &str) -> Self {
        let mut numbers = line.trim().split(",").map(|c| c.parse().unwrap());
        Self {
            x: numbers.next().unwrap(),
            y: numbers.next().unwrap(),
        }
    }

    pub fn area(&self, other: &Location) -> usize {
        // Area is distance in x and distance in y
        (((self.x as isize - other.x as isize).abs() + 1)
            * ((self.y as isize - other.y as isize).abs() + 1)) as usize
    }

    pub fn get_range(&self, other: &Location) -> Range {
        if self.x == other.x {
            Range {
                direction: Direction::Y,
                start: self.y.min(other.y),
                end: self.y.max(other.y),
            }
        } else if self.y == other.y {
            Range {
                direction: Direction::X,
                start: self.x.min(other.x),
                end: self.x.max(other.x),
            }
        } else {
            panic!("This doesn't make sense")
        }
    }

    /// Return bottom left and top right
    pub fn get_corners(&self, other: &Location) -> (Location, Location) {
        // bottom left is the min x and the max y
        let min_x = self.x.min(other.x);
        let max_y = self.y.max(other.y);
        let bottom_left = Location { x: min_x, y: max_y };
        // Top right is max x and min y
        let max_x = self.x.max(other.x);
        let min_y = self.y.min(other.y);
        let top_right = Location { x: max_x, y: min_y };
        (bottom_left, top_right)
    }
}
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // We will define less than as is more bottom left -- so x is <= and y is >=
        if self.x <= other.x && self.y >= other.y {
            Some(Ordering::Less)
        }
        // We will define greater than as is more upper right, so x >= and y is <=
        else if self.x >= other.x && self.y <= other.y {
            Some(Ordering::Greater)
        } else if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

/// Range for sweeping
#[derive(Debug, Clone)]
struct Range {
    direction: Direction,
    // Inclusive
    start: usize,
    // Inclusive
    end: usize,
}

/// direction
#[derive(Debug, Clone)]
enum Direction {
    X,
    Y,
}

/// Driver
#[derive(Debug, Clone)]
struct Driver {
    red_tiles: Vec<Location>,
}
impl Driver {
    pub fn new(s: &str) -> Self {
        let red_tiles = s.lines().map(Location::new).collect();
        Self { red_tiles }
    }

    pub fn part_one(&self) -> usize {
        let mut max_area = 0_usize;
        for idx0 in 0..self.red_tiles.len() - 1 {
            let tile0 = &self.red_tiles[idx0];
            for idx1 in idx0 + 1..self.red_tiles.len() {
                let tile1 = &self.red_tiles[idx1];
                max_area = max_area.max(tile0.area(tile1));
            }
        }
        max_area
    }

    pub fn part_two(&self) -> usize {
        // Create the board
        let largest_x_value = self
            .red_tiles
            .iter()
            .fold(0_usize, |max_value, current_value| {
                max_value.max(current_value.x)
            });
        let largest_y_value = self
            .red_tiles
            .iter()
            .fold(0_usize, |max_value, current_value| {
                max_value.max(current_value.y)
            });
        // 0 -- open, 1 -- red, 2 -- green
        let mut board = DMatrix::from_element(largest_y_value + 1, largest_x_value + 1, 0_u8);
        for idx0 in 0..self.red_tiles.len() {
            let red_tile = &self.red_tiles[idx0];
            // Mark the current as red
            *board.get_mut((red_tile.y, red_tile.x)).unwrap() = 1;
            if idx0 > 0 {
                let prev_red_tile = &self.red_tiles[idx0 - 1];
                let range_that_is_green = red_tile.get_range(prev_red_tile);
                match range_that_is_green.direction {
                    Direction::X => {
                        for x in range_that_is_green.start..=range_that_is_green.end {
                            if board[(red_tile.y, x)] == 0 {
                                *board.get_mut((red_tile.y, x)).unwrap() = 2;
                            }
                        }
                    }
                    Direction::Y => {
                        for y in range_that_is_green.start..=range_that_is_green.end {
                            if board[(y, red_tile.x)] == 0 {
                                *board.get_mut((y, red_tile.x)).unwrap() = 2;
                            }
                        }
                    }
                }
            }
            if idx0 == self.red_tiles.len() - 1 {
                let prev_red_tile = &self.red_tiles[0];
                let range_that_is_green = red_tile.get_range(prev_red_tile);
                match range_that_is_green.direction {
                    Direction::X => {
                        for x in range_that_is_green.start..=range_that_is_green.end {
                            if board[(red_tile.y, x)] == 0 {
                                *board.get_mut((red_tile.y, x)).unwrap() = 2;
                            }
                        }
                    }
                    Direction::Y => {
                        for y in range_that_is_green.start..=range_that_is_green.end {
                            if board[(y, red_tile.x)] == 0 {
                                *board.get_mut((y, red_tile.x)).unwrap() = 2;
                            }
                        }
                    }
                }
            }
        }
        println!("Initial board -- {}x{}", board.nrows(), board.ncols());
        // Now, fill in the board
        for idx_r in 0..board.nrows() {
            // We need to find the first non-zero and the last non-zero
            let mut first_non_zero = board.ncols();
            let mut last_non_zero = 0_usize;
            for idx_c in 0..board.ncols() {
                if board[(idx_r, idx_c)] != 0 {
                    first_non_zero = first_non_zero.min(idx_c);
                    last_non_zero = last_non_zero.max(idx_c);
                }
            }
            // Go through again and set
            for idx_c in 0..board.ncols() {
                if idx_c > first_non_zero && idx_c < last_non_zero && board[(idx_r, idx_c)] == 0 {
                    *board.get_mut((idx_r, idx_c)).unwrap() = 2;
                }
            }
            if idx_r % 100 == 0 {
                println!(
                    "Completed {idx_r} rows of {} -- {:0.2}%",
                    board.nrows(),
                    idx_r as f32 / board.nrows() as f32 * 100.0
                );
            }
        }
        println!("Filled in the board");
        let mut areas = Vec::new();
        for idx0 in 0..self.red_tiles.len() - 1 {
            let tile0 = &self.red_tiles[idx0];
            for idx1 in idx0 + 1..self.red_tiles.len() {
                let tile1 = &self.red_tiles[idx1];
                let area = tile0.area(tile1);
                areas.push(AreaResults {
                    area,
                    tile0: tile0.clone(),
                    tile1: tile1.clone(),
                });
            }
            println!(
                "Completed {idx0} tiles of {} -- {:0.2}%",
                self.red_tiles.len(),
                idx0 as f32 / self.red_tiles.len() as f32 * 100.0
            );
        }
        println!("Completed areas");
        // Now, we need to sort the areas and then iterate until we find one that is valid
        areas.sort_by_key(|val| val.area);
        areas.reverse();
        let mut previously_invalidated_locations = Vec::new();
        for area in areas {
            if area.validate(&board, &previously_invalidated_locations) {
                return area.area;
            } else {
                previously_invalidated_locations.push(area);
            }
        }
        0
    }
}

/// Results
#[derive(Debug, Clone)]
struct AreaResults {
    area: usize,
    tile0: Location,
    tile1: Location,
}
impl AreaResults {
    pub fn validate(
        &self,
        board: &DMatrix<u8>,
        previously_invalidated_locations: &[AreaResults],
    ) -> bool {
        // Check to see if it has already been Invalidated
        for prev_loc in previously_invalidated_locations {
            if self.consumes(prev_loc) {
                println!("Invalidated because we have seen before");
                // We already proved this one doesn't work, stop looking
                return false;
            }
        }
        for y in self.tile0.y.min(self.tile1.y)..=self.tile0.y.max(self.tile1.y) {
            for x in self.tile0.x.min(self.tile1.x)..=self.tile0.x.max(self.tile1.x) {
                if board[(y, x)] == 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Determine if this area consumes the other area
    ///
    /// In order to determine this, we need to determine which is top right and which is bottom
    /// left for both tiles. Then, we need to see if our bottom left is less than their bottom left
    /// and our top right is greater than their top right
    pub fn consumes(&self, other: &AreaResults) -> bool {
        let (bottom_left, top_right) = self.tile0.get_corners(&self.tile1);
        let (other_bottom_left, other_top_right) = other.tile0.get_corners(&other.tile1);
        println!(
            "Bottom lefts: {bottom_left:?} -- {other_bottom_left:?} -- {:?}\tTop rights: {top_right:?} -- {other_top_right:?} -- {:?}",
            bottom_left < other_bottom_left,
            top_right > other_top_right
        );
        bottom_left < other_bottom_left && top_right > other_top_right
    }
}

fn part_one(s: &str) -> usize {
    let driver = Driver::new(s);
    driver.part_one()
}

fn part_two(s: &str) -> usize {
    let driver = Driver::new(s);
    driver.part_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
    }

    #[test]
    fn test_one() {
        let output = part_one(input_one());

        // TODO fill this out
        assert_eq!(output, 50);
    }

    #[test]
    fn test_two() {
        let output = part_two(input_one());

        // TODO fill this out
        assert_eq!(output, 24);
    }
}
