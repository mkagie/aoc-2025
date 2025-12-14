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
    let input = parse_input(file);
    part_one_internal(input)
}

fn part_two(file: BufReader<File>) -> ReturnType {
    let input = parse_input2(file);
    part_two_internal(input)
}

fn parse_input(file: BufReader<File>) -> Vec<Vec<String>> {
    file.lines()
        .map(|x| {
            let x = x.unwrap();
            x.split_whitespace().map(|x| x.to_owned()).collect()
        })
        .collect()
}

fn parse_input2(file: BufReader<File>) -> Vec<Vec<char>> {
    // Go through and create a Vec<Vec<char>>
    file.lines()
        .map(|x| {
            let x = x.unwrap();
            x.chars().collect()
        })
        .collect()
}

type ReturnType = usize;
type InputType = Vec<Vec<String>>;

/// Internal logic for part_one
fn part_one_internal(input: InputType) -> ReturnType {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut big_sum = 0;
    for idx_c in 0..n_cols {
        // Select the sign
        let math = match input[n_rows - 1][idx_c].as_str() {
            "*" => {
                // Multiply all values
                let mut mult = 1;
                for idx_r in 0..n_rows - 1 {
                    mult *= input[idx_r][idx_c].parse::<usize>().unwrap();
                }
                mult
            }
            "+" => {
                // Add all values
                let mut sum = 0;
                for idx_r in 0..n_rows - 1 {
                    sum += input[idx_r][idx_c].parse::<usize>().unwrap();
                }
                sum
            }
            _ => panic!("Not a valid sign"),
        };
        big_sum += math;
    }
    big_sum
}

/// Internal logic for part two
fn part_two_internal(input: Vec<Vec<char>>) -> ReturnType {
    // Right now, we have rows x columns of numbers. We need to convert this to, for each column,
    // create a new entry that is rows by columns of numerical characters
    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut big_sum = 0;
    let mut sign = None;
    let mut tmp_value = 0;

    for idx_c in 0..n_cols {
        if sign.is_none() {
            // If sign is None, then we need to determine the sign
            sign = Some(input[n_rows - 1][idx_c]);
            match sign {
                Some('+') => tmp_value = 0,
                Some('*') => tmp_value = 1,
                _ => panic!("Not a valid value"),
            }
        }
        if is_all_space(&input, idx_c) {
            // Take the value, add it to the big_sum and remove the sign
            big_sum += tmp_value;
            sign = None;
        } else {
            // Go down the row and do something with the value
            let mut d = 0;
            'a: for idx_r in 0..n_rows - 1 {
                if input[idx_r][idx_c].is_whitespace() {
                    continue 'a;
                }
                d = d * 10 + input[idx_r][idx_c].to_digit(10).unwrap() as usize;
            }
            match sign {
                Some('+') => tmp_value += d,
                Some('*') => tmp_value *= d,
                _ => panic!("Not a valid value"),
            }
        }
    }
    // Now, we need to actually add tmp_value to big_sum
    big_sum + tmp_value
}

fn is_all_space(input: &[Vec<char>], idx_c: usize) -> bool {
    let n_rows = input.len();
    for idx_r in 0..n_rows {
        if !input[idx_r][idx_c].is_whitespace() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
    }

    /// Function to split above into different inputs
    fn parse_input_test(input: &str) -> Vec<Vec<String>> {
        input
            .lines()
            .map(|x| x.split_whitespace().map(|x| x.to_owned()).collect())
            .collect()
    }

    /// Function to split above into different inputs
    fn parse_input_test2(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|x| x.chars().collect()).collect()
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 4277556);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test2(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 3263827);
    }
}
