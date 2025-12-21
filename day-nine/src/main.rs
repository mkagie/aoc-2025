//! Command line executable for running part one and part two
use std::time::Instant;

use clap::Parser;
use geo::{Contains as _, Coord, LineString, Polygon, Rect};

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
        let board = Board::new(&self.red_tiles);
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
        }
        // Now, we need to sort the areas and then iterate until we find one that is valid
        areas.sort_by_key(|val| val.area);
        areas.reverse();
        for area in areas {
            if board.contains(&area) {
                return area.area;
            }
        }
        panic!("All are impossible");
    }
}

/// Results
#[derive(Debug, Clone)]
struct AreaResults {
    area: usize,
    tile0: Location,
    tile1: Location,
}

/// A different way to represent the board
#[derive(Debug, Clone)]
struct Board {
    hull: Polygon<f32>,
}
impl Board {
    pub fn new(red_tiles: &[Location]) -> Self {
        let coords: Vec<Coord<f32>> = red_tiles
            .iter()
            .map(|tile| Coord {
                x: tile.x as f32,
                y: tile.y as f32,
            })
            .collect();
        let mut linestring = LineString::from(coords);
        linestring.close(); // Make sure that it is closed
        let polygon = Polygon::new(linestring, Vec::new());
        Self { hull: polygon }
    }

    pub fn contains(&self, result: &AreaResults) -> bool {
        let min_x = result.tile0.x.min(result.tile1.x) as f32;
        let max_x = result.tile0.x.max(result.tile1.x) as f32;
        let min_y = result.tile0.y.min(result.tile1.y) as f32;
        let max_y = result.tile0.y.max(result.tile1.y) as f32;
        let rect: Polygon<_> =
            Rect::new(Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y }).into();
        self.hull.contains(&rect)
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
