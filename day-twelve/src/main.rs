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
}

fn main() {
    let args = Args::parse();

    // Read to a string
    let s = std::fs::read_to_string(args.input_file).expect("Failed to read file");

    let start = Instant::now();
    let answer = match args.part {
        Part::Part1 => part_one(&s),
    };

    println!("{:?}", answer);
    println!("Completed in {:?}", start.elapsed());
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Shape {
    grid: Vec<Vec<bool>>,
}
impl Shape {
    /// Assuming lines are:
    /// ---
    /// ---
    /// ---
    pub fn from_lines(lines: &[&str]) -> Self {
        // Skip the first line
        let grid = lines
            .iter()
            .map(|line| {
                let line = line.trim();
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Not valid"),
                    })
                    .collect()
            })
            .collect();
        Self { grid }
    }
    pub fn size(&self) -> usize {
        self.grid.iter().fold(0_usize, |accum, v| {
            accum + v.iter().map(|x| *x as usize).sum::<usize>()
        })
    }
}

/// Represent a region
#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}
impl Region {
    pub fn from_line(line: &str) -> Self {
        let line = line.trim();
        let mut split = line.split(":");
        let mut wxh = split.next().unwrap().split("x").map(|x| x.parse().unwrap());
        let width = wxh.next().unwrap();
        let height = wxh.next().unwrap();
        let shape_counts = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Self {
            width,
            height,
            shape_counts,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

/// Parsing State
#[derive(Debug, Clone)]
enum ParsingState {
    NewShape,
    InProgress,
    ParsingRegions,
}

/// Driver
#[derive(Debug, Clone)]
struct Driver {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}
impl Driver {
    pub fn new(s: &str) -> Self {
        let mut shapes = Vec::new();
        let mut regions = Vec::new();
        let mut state = ParsingState::NewShape;
        let mut lines_for_shape = Vec::new();

        // Shape logic

        for line in s.lines() {
            match state {
                ParsingState::NewShape => {
                    if line.trim().split(":").nth(1).unwrap().is_empty() {
                        lines_for_shape.clear();
                        state = ParsingState::InProgress;
                    } else {
                        // We are now in the regions portion
                        state = ParsingState::ParsingRegions;
                        // We do not want to miss this line... so parse
                        regions.push(Region::from_line(line));
                    }
                }
                ParsingState::InProgress => {
                    if line.is_empty() {
                        // We have reached the end -> go to start of new shape
                        state = ParsingState::NewShape;
                        // add the new shape
                        shapes.push(Shape::from_lines(&lines_for_shape));
                    } else {
                        lines_for_shape.push(line.trim());
                    }
                }
                ParsingState::ParsingRegions => {
                    regions.push(Region::from_line(line));
                }
            }
        }
        Self { shapes, regions }
    }

    pub fn part_one(&self) -> usize {
        // Let's solve this heuristically instead
        let mut n_successes = 0;
        for region in &self.regions {
            let min_size: f32 = region
                .shape_counts
                .iter()
                .enumerate()
                .map(|(shape_idx, count)| {
                    let size = self.shapes[shape_idx].size();
                    size * count
                })
                .sum::<usize>() as f32;
            if min_size * 1.2 <= region.area() as f32 {
                n_successes += 1;
            }
        }
        n_successes
    }
}

fn part_one(s: &str) -> usize {
    let driver = Driver::new(s);
    driver.part_one()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
    }

    #[test]
    fn test_one() {
        let output = part_one(input_one());

        // TODO fill this out
        assert_eq!(output, 2);
    }
}
