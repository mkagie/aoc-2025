//! Command line executable for running part one and part two
use std::time::Instant;

use clap::Parser;

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
#[derive(Debug, Clone)]
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
        println!(
            "Area: {:?}x{:?}",
            (self.x as isize - other.x as isize).abs(),
            (self.y as isize - other.y as isize).abs()
        );
        // Area is distance in x and distance in y
        (((self.x as isize - other.x as isize).abs() + 1)
            * ((self.y as isize - other.y as isize).abs() + 1)) as usize
    }

    pub fn city_block_distance(&self, other: &Location) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    pub fn distance(&self, other: &Location) -> f32 {
        ((self.x as f32 - other.x as f32).powi(2) + (self.y as f32 - other.y as f32).powi(2)).sqrt()
    }
}

/// Corner control enum
#[derive(Debug, Clone)]
enum CornerControl {
    UpperRight,
    LowerLeft,
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

    /// Returns the index of the upper right corner
    ///
    /// The upper right corner is defined as the minimum distance from the largest x value and a y
    /// value of 0
    fn find_corner(&self, control: CornerControl) -> usize {
        let comparator = match control {
            CornerControl::UpperRight => {
                // Find the largest x value
                let largest_x_value = self
                    .red_tiles
                    .iter()
                    .fold(0_usize, |max_value, current_value| {
                        max_value.max(current_value.x)
                    });
                Location {
                    x: largest_x_value,
                    y: 0,
                }
            }
            CornerControl::LowerLeft => {
                let largest_y_value = self
                    .red_tiles
                    .iter()
                    .fold(0_usize, |max_value, current_value| {
                        max_value.max(current_value.y)
                    });
                Location {
                    x: 0,
                    y: largest_y_value,
                }
            }
        };
        self.red_tiles
            .iter()
            // .map(|tile| tile.city_block_distance(&comparator))
            .map(|tile| tile.distance(&comparator))
            .enumerate()
            .min_by(|(_, value0), (_, value1)| value0.partial_cmp(value1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap()
    }

    pub fn part_one(&self) -> usize {
        // Find the upper right
        let upper_right = &self.red_tiles[self.find_corner(CornerControl::UpperRight)];
        let lower_left = &self.red_tiles[self.find_corner(CornerControl::LowerLeft)];
        println!("Upper Right: {upper_right:?}\tLower left: {lower_left:?}");
        upper_right.area(lower_left)
    }

    pub fn part_one_b(&self) -> usize {
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
}

fn part_one(s: &str) -> usize {
    let driver = Driver::new(s);
    driver.part_one_b()
}

fn part_two(s: &str) -> usize {
    todo!()
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
        assert_eq!(output, 0);
    }
}
