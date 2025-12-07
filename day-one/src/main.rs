//! Command line executable for running part one and part two
use std::{
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
    let input = parse_input(file, map_one);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input(file, map_two);
    part_two_internal(input)
}

fn parse_input<F, T>(file: BufReader<File>, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    file.lines().map(|x| f(x.unwrap().as_str())).collect()
}

// TODO -- Update this with the return type
type ReturnType = usize;
type VectorType = Rotation;
type VectorType2 = u32;

/// Rotation
pub enum Rotation {
    Left(u16),
    Right(u16),
}
impl Rotation {
    pub fn from_line(line: &str) -> Self {
        // Unwrap because too lazy to check -- feel free to crash
        let rot_value: u16 = line.split_at(1).1.parse().expect("Invalid");
        match line.chars().next().expect("Invalid line -- no characters") {
            'L' => Rotation::Left(rot_value),
            'R' => Rotation::Right(rot_value),
            _ => panic!("Not valid start to line"),
        }
    }
}

/// Counter
#[derive(Debug)]
pub struct Counter {
    val: u16,
    counter_pt_1: usize,
}
impl Default for Counter {
    fn default() -> Self {
        Self {
            val: 50,
            counter_pt_1: 0,
        }
    }
}
impl Counter {
    pub fn rotate(&mut self, rot: &Rotation) {
        match rot {
            Rotation::Left(val) => {
                self.val = ((self.val as i16 + *val as i16) % 100_i16) as u16;
            }
            Rotation::Right(val) => {
                self.val = ((self.val as i16 - *val as i16) % 100_i16) as u16;
            }
        }
        if self.val == 0 {
            self.counter_pt_1 += 1;
        }
    }

    pub fn get_counter_pt_1(&self) -> usize {
        self.counter_pt_1
    }
}

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    Rotation::from_line(input)
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    todo!()
}

/// Internal logic for part_one
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    let mut counter = Counter::default();
    for rot in input {
        counter.rotate(&rot);
    }
    counter.get_counter_pt_1()
}

/// Internal logic for part two
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
    }

    /// Function to split above into different inputs
    fn parse_input_test<F, T>(input: &str, f: F) -> Vec<T>
    where
        F: Fn(&str) -> T,
    {
        input.lines().map(|x| f(x)).collect()
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one(), map_one);
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 3);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one(), map_two);
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 0);
    }
}
