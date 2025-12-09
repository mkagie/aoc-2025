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

/// Bank of batteries
#[derive(Debug, Clone)]
pub struct BatteryBank(String);
impl BatteryBank {
    /// Find the largest possible joltage
    ///
    /// We are going to start by brute forcing it and seeing what would happen
    pub fn find_largest(&self) -> u16 {
        let mut largest = 0;
        let chars: Vec<_> = self.0.chars().collect();
        for idx0 in 0..chars.len() - 1 {
            let digit0: u16 = chars[idx0].to_digit(10).unwrap() as u16 * 10;
            for c in chars.iter().skip(idx0 + 1) {
                let digit1: u16 = c.to_digit(10).unwrap() as u16;
                let sum = digit0 + digit1;
                largest = largest.max(sum);
            }
        }
        largest
    }

    pub fn find_largest_k(&self, k: usize) -> usize {
        let digits: Vec<usize> = self
            .0
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        Self::pick_k(&digits, k)
    }

    /// Function that will pick k digits out of a list of characters
    fn pick_k(digits: &[usize], n_digits_to_select: usize) -> usize {
        // Base case -- there are no digits left to select
        if n_digits_to_select == 0 {
            // We can return 0 because we are going to accumulate
            return 0;
        }

        // We must pick k digits, so the search window ends at len-k
        let window_end_inclusive = digits.len() - n_digits_to_select;

        // Find max digit in this window
        let (max_idx, max_digit) = digits[..=window_end_inclusive]
            .iter()
            .enumerate()
            .rev() // Must reverse because max_by_key selects the
            // last one in a tie
            .max_by_key(|(_, d)| **d)
            .unwrap();

        // Place it in the correct power-of-10 position
        let rest = Self::pick_k(&digits[max_idx + 1..], n_digits_to_select - 1);
        max_digit * 10usize.pow((n_digits_to_select - 1) as u32) + rest
    }
}

// TODO -- Update this with the return type
type ReturnType = usize;
type VectorType = BatteryBank;
type VectorType2 = VectorType;

/// Map a line to a VectorType
fn map_one(input: &str) -> VectorType {
    BatteryBank(input.to_string())
}

/// Map a line to a VectorType
fn map_two(input: &str) -> VectorType2 {
    map_one(input)
}

/// Internal logic for part_one
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input
        .into_iter()
        .fold(0_usize, |acc, bat| acc + bat.find_largest() as usize)
}

/// Internal logic for part two
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    input
        .into_iter()
        .fold(0_usize, |acc, bat| acc + bat.find_largest_k(12))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "987654321111111
811111111111119
234234234234278
818181911112111"
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
        assert_eq!(output, 357);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one(), map_two);
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 3121910778619);
    }

    #[test]
    fn test_in_depth() {
        // let b = BatteryBank("987654321111111".to_string());
        // assert_eq!(b.find_largest(), 98);
        // assert_eq!(b.find_largest_k(12), 987654321111);

        let b = BatteryBank("818181911112111".to_string());
        assert_eq!(b.find_largest_k(12), 888911112111);
    }
}
