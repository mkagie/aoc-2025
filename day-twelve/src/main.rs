//! Command line executable for running part one and part two
use std::{
    collections::HashSet,
    sync::{Arc, atomic::AtomicUsize},
    time::Instant,
};

use clap::Parser;
use rayon::prelude::*;

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
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
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

    pub fn all_variants(&self) -> Vec<Shape> {
        let mut variants = HashSet::new(); // Use to deconflict
        let mut shape = self.clone();
        for _ in 0..4 {
            variants.insert(shape.clone());
            variants.insert(shape.flip_h());
            shape = shape.rotate90();
        }
        variants.into_iter().collect()
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

    fn fits(grid: &[Vec<bool>], shape: &Shape, x: usize, y: usize) -> bool {
        let h = shape.height();
        let w = shape.width();
        for dy in 0..h {
            for dx in 0..w {
                if shape.grid[dy][dx] && grid[y + dy][x + dx] {
                    return false;
                }
            }
        }
        true
    }

    fn place(grid: &mut [Vec<bool>], shape: &Shape, x: usize, y: usize, value: bool) {
        let h = shape.height();
        let w = shape.width();
        for dy in 0..h {
            for dx in 0..w {
                if shape.grid[dy][dx] {
                    grid[y + dy][x + dx] = value;
                }
            }
        }
    }

    fn can_fit_recursive(
        grid: &mut [Vec<bool>],
        shapes: &[Shape],
        shape_idx: usize,
        failed_scenarios: &mut HashSet<Scenario>,
        depth: usize,
    ) -> CanFitResult {
        if shape_idx >= shapes.len() {
            return CanFitResult::True;
        }

        if depth >= 200 {
            return CanFitResult::MaxDepthReached;
        }

        // Try every variant
        let variants = shapes[shape_idx].all_variants();
        for (variant_idx, variant) in variants.into_iter().enumerate() {
            let h = variant.height();
            let w = variant.width();

            // Try all starting positions
            for y in 0..=grid.len() - h {
                for x in 0..=grid[0].len() - w {
                    let scenario = Scenario {
                        grid: grid.to_owned(),
                        shape_idx,
                        variant_idx,
                        x,
                        y,
                    };
                    if failed_scenarios.contains(&scenario) {
                        continue;
                    }
                    if Self::fits(grid, &variant, x, y) {
                        // Place in the grid
                        Self::place(grid, &variant, x, y, true);

                        // If this works, great -- reduce the shape counts of this index by one and
                        // try again
                        let result = Self::can_fit_recursive(
                            grid,
                            shapes,
                            shape_idx + 1,
                            failed_scenarios,
                            depth + 1,
                        );
                        if matches!(result, CanFitResult::True) {
                            return CanFitResult::True;
                        }
                        if matches!(result, CanFitResult::MaxDepthReached) {
                            return CanFitResult::MaxDepthReached;
                        }
                        // Otherwise, it failed
                        failed_scenarios.insert(scenario);

                        // If it does not, backgrack
                        // undo the increment
                        Self::place(grid, &variant, x, y, false);
                    }
                }
            }
        }
        CanFitResult::False
    }

    pub fn can_fit(region: &Region, shapes: &[Shape]) -> bool {
        let mut grid = vec![vec![false; region.width]; region.height];
        // Create a list of shapes independent of the counts
        let mut relevant_shapes = Vec::new();
        for (shape_idx, count) in region.shape_counts.iter().enumerate() {
            for _ in 0..*count {
                relevant_shapes.push(shapes[shape_idx].clone());
            }
        }
        // I would like to place largest shapes first because they will be the most restrictive
        relevant_shapes.sort_by_key(|shape| shape.size());
        relevant_shapes.reverse();

        let mut failed_scenarios = HashSet::new();

        matches!(
            Self::can_fit_recursive(&mut grid, &relevant_shapes, 0, &mut failed_scenarios, 0),
            CanFitResult::True
        )
    }

    pub fn part_one(&self) -> usize {
        let success_counter = Arc::new(AtomicUsize::new(0));
        let regions_left = Arc::new(AtomicUsize::new(self.regions.len()));
        let successes: usize = self
            .regions
            .par_iter()
            .enumerate()
            .map(move |(region_idx, region)| {
                let can_fit = Self::can_fit(region, &self.shapes);
                if can_fit {
                    success_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                regions_left.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                println!(
                    "Finished region {region_idx} -- can fit: {can_fit} -- Total: {}\t{} left",
                    success_counter.load(std::sync::atomic::Ordering::Relaxed),
                    regions_left.load(std::sync::atomic::Ordering::Relaxed)
                );
                can_fit as usize
            })
            .sum();
        successes
    }
}

/// Failed location -- need to prune
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Scenario {
    grid: Vec<Vec<bool>>,
    shape_idx: usize,
    variant_idx: usize,
    x: usize,
    y: usize,
}

/// Max Depth Indicator
#[derive(Debug, Clone)]
enum CanFitResult {
    True,
    False,
    MaxDepthReached,
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
