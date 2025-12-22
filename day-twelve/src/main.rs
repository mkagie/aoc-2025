//! Command line executable for running part one and part two
use std::{collections::HashSet, time::Instant};

use clap::Parser;
use dlx_rs::Solver;

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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Shape {
    grid: Vec<Vec<bool>>,
}
impl Shape {
    /// Assuming lines are:
    /// ---
    /// ---
    /// ---
    fn from_lines(lines: &[&str]) -> Self {
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
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn rotate90(&self) -> Shape {
        let h = self.height();
        let w = self.width();
        let mut new_grid = vec![vec![false; h]; w];
        for i in 0..h {
            for j in 0..w {
                new_grid[j][h - 1 - i] = self.grid[i][j];
            }
        }
        Shape { grid: new_grid }
    }

    fn flip_h(&self) -> Shape {
        let mut new_grid = self.grid.clone();
        for row in &mut new_grid {
            row.reverse();
        }
        Shape { grid: new_grid }
    }

    fn all_variants(&self) -> Vec<Shape> {
        let mut variants = HashSet::new(); // Use to deconflict
        let mut shape = self.clone();
        for _ in 0..4 {
            variants.insert(shape.clone());
            variants.insert(shape.flip_h());
            shape = shape.rotate90();
        }
        variants.into_iter().collect()
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

    /// Use DLX to determine if they can fit
    ///
    /// Constraints (columns):
    /// - Constraint A: Grid cells -- each grid cell can be occupied by at most one shape
    /// - Constraint B: each present must be placed 'count' times
    ///
    /// Options (row)
    /// - Place a specific shape variant at a specific location
    fn can_fit(region: &Region, shapes: &[Shape]) -> bool {
        // In support of column layout:
        // [ grid cell 0 | grid cell 1 | ... | grid cell N | shape 0 | shape 1 | ... ]
        let n_cells = region.width * region.height;
        let n_shapes = region.shape_counts.len();

        // Create a new solver with total columns:
        // n_cells + n_shapes (present count columns)
        let mut solver = Solver::new(n_cells + n_shapes);
        for cell in 0..n_cells {
            solver.add_option((), &[cell]);
        }

        // For each shape we need to place
        for (shape_idx, &count) in region.shape_counts.iter().enumerate() {
            // This shape is not required to fit
            if count == 0 {
                continue;
            }

            // Precompute unique rotations/flips
            let variants = shapes[shape_idx].all_variants();
            for variant in variants {
                let h = variant.height();
                let w = variant.width();

                // Try all possible positions
                for y in 0..=region.height - h {
                    for x in 0..=region.width - w {
                        // The shape starts at (x, y)

                        // Create a DLX row (option)
                        let mut row = Vec::new();

                        // Cover every grid cell
                        for dy in 0..h {
                            for dx in 0..w {
                                if variant.grid[dy][dx] {
                                    let idx = (y + dy) * region.width + (x + dx);
                                    row.push(idx);
                                }
                            }
                        }

                        // And cover the present-use constraint
                        row.push(n_cells + shape_idx); // This is indicative of the current shape
                        // being placed

                        // Add this placement as an option
                        solver.add_option((), &row);
                    }
                }
            }
        }

        // Try to find *one* exact cover
        println!("Solver: {:?}", solver.next());
        solver.next().is_some()
    }

    pub fn part_one(&self) -> usize {
        let mut success = 0;
        for region in &self.regions {
            if Self::can_fit(region, &self.shapes) {
                success += 1;
            }
        }
        success
    }
}

fn part_one(s: &str) -> usize {
    let driver = Driver::new(s);
    driver.part_one()
}

fn part_two(s: &str) -> usize {
    todo!()
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

    #[test]
    fn test_two() {
        let output = part_two(input_one());

        // TODO fill this out
        assert_eq!(output, 0);
    }
}
