//! Command line executable for running part one and part two
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
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

fn parse_input(file: BufReader<File>) -> InputType {
    // let mut fresh_ingredients = FreshIngredients::default();
    let mut fresh_ingredients = Ranges::default();
    let mut has_found_blank_line = false;
    let mut ingredients = IngredientsList::default();
    for line in file.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            has_found_blank_line = true;
            println!("Filled the fresh list");
            continue;
        }
        if !has_found_blank_line {
            fresh_ingredients.add_range(&line);
        } else {
            ingredients.add_ingredient(line.trim().parse().unwrap());
        }
    }
    (fresh_ingredients, ingredients)
}

// TODO -- Update this with the return type
type ReturnType = usize;
type InputType = (Ranges, IngredientsList);

/// Ingredients List -- a HashMap with a number of times it is called
#[derive(Debug, Clone, Default)]
pub struct IngredientsList(HashMap<usize, usize>);
impl IngredientsList {
    pub fn add_ingredient(&mut self, ingredient: usize) {
        if let Some(val) = self.0.get_mut(&ingredient) {
            *val += 1;
        } else {
            self.0.insert(ingredient, 1);
        }
    }
}

/// Try without memoization
#[derive(Debug, Clone)]
pub struct Range {
    start: usize,
    end: usize,
}
impl Range {
    pub fn new(input: &str) -> Self {
        // Convert the range to numbers
        let mut split = input.trim().split("-");
        let start: usize = split.next().unwrap().parse().unwrap();
        let end: usize = split.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    pub fn contains(&self, value: usize) -> bool {
        value >= self.start && value <= self.end
    }
}

/// Ranges
#[derive(Debug, Clone, Default)]
pub struct Ranges(Vec<Range>);
impl Ranges {
    pub fn add_range(&mut self, input: &str) {
        self.0.push(Range::new(input))
    }

    pub fn contains(&self, value: usize) -> bool {
        for range in self.0.iter() {
            if range.contains(value) {
                return true;
            }
        }
        false
    }
}

/// Internal logic for part_one
fn part_one_internal(input: InputType) -> ReturnType {
    let (fresh_ingredients, ingredients_to_check) = input;
    ingredients_to_check.0.keys().fold(0, |acc, ingredient| {
        acc + if fresh_ingredients.contains(*ingredient) {
            1
        } else {
            0
        }
    })
}

/// Internal logic for part two
fn part_two_internal(input: InputType) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
    }

    /// Function to split above into different inputs
    fn parse_input_test(input: &str) -> InputType {
        // let mut fresh_ingredients = FreshIngredients::default();
        let mut fresh_ingredients = Ranges::default();
        let mut has_found_blank_line = false;
        let mut ingredients = IngredientsList::default();
        for line in input.lines() {
            if line.is_empty() {
                has_found_blank_line = true;
                continue;
            }
            if !has_found_blank_line {
                fresh_ingredients.add_range(line);
            } else {
                ingredients.add_ingredient(line.trim().parse().unwrap());
            }
        }
        (fresh_ingredients, ingredients)
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 3);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 0);
    }
}
