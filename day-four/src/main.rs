//! Command line executable for running part one and part two
use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

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

    let file = BufReader::new(File::open(args.input_file).expect("Cannot find file"));

    let start = Instant::now();
    let answer = match args.part {
        Part::Part1 => part_one(file),
        Part::Part2 => part_two(file),
    };

    println!("{:?}", answer);
    println!("Completed in {:?}", start.elapsed());
}

fn part_one(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file);
    part_two_internal(input)
}

fn parse_input(mut file: BufReader<File>) -> Grid {
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    Grid::new(&s)
}

// TODO -- Update this with the return type
type ReturnType = usize;

/// Grid
#[derive(Debug, Clone)]
pub struct Grid {
    inner: Vec<Vec<bool>>, // Represents whether or not there is a roll there
    neighbor_map: Vec<Vec<usize>>, // Represents the number of neighbors with a roll
    accessibility_map: Vec<Vec<bool>>, // Represents whether the roll is accessible or not
}
impl Grid {
    pub fn new(input: &str) -> Self {
        let inner: Vec<Vec<bool>> = input
            .lines()
            .map(|line| {
                // Convert a line to an array of bools
                line.chars().map(|c| matches!(c, '@')).collect()
            })
            .collect();
        let (neighbor_map, accessibility_map) = Self::populate_neighbor_map(&inner);
        Self {
            inner,
            neighbor_map,
            accessibility_map,
        }
    }

    pub fn count_roll_access(&self) -> usize {
        self.accessibility_map.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc_row, c| acc_row + if *c { 1 } else { 0 })
        })
    }

    pub fn part2(&mut self) -> usize {
        let mut s = 0;
        loop {
            let n_rolls_removed = self.evolve();
            s += n_rolls_removed;
            if n_rolls_removed == 0 {
                return s;
            }
        }
    }

    fn populate_neighbor_map(inner: &[Vec<bool>]) -> (Vec<Vec<usize>>, Vec<Vec<bool>>) {
        let mut neighbor_map = Vec::new();
        let mut part1_map = Vec::new();
        for r in 0..inner.len() {
            let mut row_vec = Vec::new();
            let mut row_vec_pt1 = Vec::new();
            for c in 0..inner[0].len() {
                let mut sum_neighbors = 0;
                for offset_r in -1..=1 {
                    for offset_c in -1..=1 {
                        if let Ok((idx_r, idx_c)) =
                            Self::check_neighbor(inner, r, c, offset_r, offset_c)
                            && inner[idx_r][idx_c]
                        {
                            sum_neighbors += 1;
                        }
                    }
                }
                row_vec.push(sum_neighbors);
                row_vec_pt1.push(sum_neighbors < 4 && inner[r][c]);
            }
            neighbor_map.push(row_vec);
            part1_map.push(row_vec_pt1);
        }
        (neighbor_map, part1_map)
    }

    /// Validate whether this neighbor
    fn check_neighbor(
        inner: &[Vec<bool>],
        row: usize,
        col: usize,
        offset_row: i8,
        offset_col: i8,
    ) -> Result<(usize, usize), ()> {
        // This is not a neighbor
        if offset_row == 0 && offset_col == 0 {
            return Err(());
        }

        let new_row = (row as isize) + (offset_row as isize);
        let new_row = if new_row >= (inner.len() as isize) || new_row < 0 {
            return Err(());
        } else {
            new_row as usize
        };

        let new_col = (col as isize) + (offset_col as isize);
        let new_col = if new_col >= (inner[0].len() as isize) || new_col < 0 {
            return Err(());
        } else {
            new_col as usize
        };
        Ok((new_row, new_col))
    }

    /// Function to evolve -- remove the rolls and recompute everything
    ///
    /// Returns the number of rolls removed
    fn evolve(&mut self) -> usize {
        // Start by copying the accessibility_map
        let accessibility_map = self.accessibility_map.clone();
        let mut n_rolls_removed = 0;
        // We do not need to copy the accessibility_map, as we can modify that in place

        for (idx_r, row) in accessibility_map.iter().enumerate() {
            for (idx_c, entry) in row.iter().enumerate() {
                // If the entry is accessible, remove it
                if *entry {
                    // Accessible, let's remove
                    n_rolls_removed += 1;
                    // Modify the current board to be false in that location
                    self.inner[idx_r][idx_c] = false;
                    self.accessibility_map[idx_r][idx_c] = false;
                    // Modify the neighbors counts to no longer consider that one as a roll
                    Self::update_removal_and_accessibility_of_neighbors(
                        &self.inner,
                        &mut self.neighbor_map,
                        &mut self.accessibility_map,
                        idx_r,
                        idx_c,
                    );
                }
            }
        }
        n_rolls_removed
    }

    /// Update the removal and accessility of neighbors
    fn update_removal_and_accessibility_of_neighbors(
        inner: &[Vec<bool>],
        neighbor_map: &mut [Vec<usize>],
        accessibility_map: &mut [Vec<bool>],
        row: usize,
        col: usize,
    ) {
        for offset_row in -1..=1 {
            for offset_col in -1..=1 {
                if let Ok((idx_r, idx_c)) =
                    Self::check_neighbor(inner, row, col, offset_row, offset_col)
                {
                    // Subtract from the neighbor map
                    neighbor_map[idx_r][idx_c] -= 1; // We don't have to check, because we know
                    // previously it had at least one
                    // Re-evaluate accessibility_map
                    accessibility_map[idx_r][idx_c] =
                        neighbor_map[idx_r][idx_c] < 4 && inner[idx_r][idx_c]
                }
            }
        }
    }
}

/// Internal logic for part_one
fn part_one_internal(input: Grid) -> ReturnType {
    input.count_roll_access()
}

/// Internal logic for part two
fn part_two_internal(mut input: Grid) -> ReturnType {
    input.part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
    }

    /// Function to split above into different inputs
    fn parse_input_test(input: &str) -> Grid {
        Grid::new(input)
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 13);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 43);
    }
}
