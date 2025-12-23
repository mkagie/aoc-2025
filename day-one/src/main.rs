//! Command line executable for running part one and part two
use std::{fs::File, io::Read, time::Instant};

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

    let mut s = String::new();
    let mut file = File::open(args.input_file).expect("Cannot find file");
    let _ = file.read_to_string(&mut s).unwrap();

    let start = Instant::now();
    let answer = match args.part {
        Part::Part1 => part_one(&s),
        Part::Part2 => part_two(&s),
    };

    println!("{:?}", answer);
    println!("Completed in {:?}", start.elapsed());
}

/// Rotation
#[derive(Debug)]
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
    val: u8,
    counter_pt_1: usize,
    counter_pt_2: usize,
}
impl Default for Counter {
    fn default() -> Self {
        Self {
            val: 50,
            counter_pt_1: 0,
            counter_pt_2: 0,
        }
    }
}
impl Counter {
    pub fn rotate(&mut self, rot: &Rotation) {
        let (int_val, v) = match rot {
            Rotation::Left(v) => (self.val as i16 - *v as i16, *v as i16),
            Rotation::Right(v) => (self.val as i16 + *v as i16, *v as i16),
        };
        let diff = if self.val == 0 {
            100
        } else {
            match rot {
                Rotation::Left(_) => self.val as i16,
                Rotation::Right(_) => 100 - self.val as i16,
            }
        };
        if v >= diff {
            self.counter_pt_2 += ((v - diff) / 100) as usize + 1;
        }
        self.val = int_val.rem_euclid(100_i16) as u8;
        if self.val == 0 {
            self.counter_pt_1 += 1;
        }
    }

    pub fn get_counter_pt_1(&self) -> usize {
        self.counter_pt_1
    }

    pub fn get_counter_pt_2(&self) -> usize {
        self.counter_pt_2
    }
}

fn part_one(input: &str) -> usize {
    let rotations: Vec<_> = input.lines().map(Rotation::from_line).collect();
    let mut counter = Counter::default();
    for rot in rotations {
        counter.rotate(&rot);
    }
    counter.get_counter_pt_1()
}

fn part_two(input: &str) -> usize {
    let rotations: Vec<_> = input.lines().map(Rotation::from_line).collect();
    let mut counter = Counter::default();
    for rot in rotations {
        counter.rotate(&rot);
    }
    counter.get_counter_pt_2()
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

    #[test]
    fn test_one() {
        let output = part_one(input_one());

        // TODO fill this out
        assert_eq!(output, 3);
    }

    #[test]
    fn test_two() {
        let output = part_two(input_one());

        // TODO fill this out
        assert_eq!(output, 6);
    }

    #[test]
    fn test_euclid() {
        assert_eq!((-20_i16).rem_euclid(100), 80);
        assert_eq!((-120_i16).div_euclid(100), -2);
    }
}
