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
    let input = parse_input(file);
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
fn part_two_internal(input: InputType) -> ReturnType {
    // Right now, we have rows x columns of numbers. We need to convert this to, for each column,
    // create a new entry that is rows by columns of numerical characters
    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut big_sum = 0;
    for idx_c in 0..n_cols {
        let sign = input[n_rows - 1][idx_c].as_str();
        // Now, convert this into a new vector of vectors
        let mut v = Vec::new();
        let mut max_n_chars = 0;
        // Go through 2x -- the first time to get the max number of characters, the second time
        // to actually store.
        // This could and should be done more intelligently, but I don't care enough to do that
        // right now
        for idx_r in 0..n_rows - 1 {
            max_n_chars = max_n_chars.max(input[idx_r][idx_c].chars().collect::<Vec<_>>().len());
        }

        for idx_r in 0..n_rows - 1 {
            // Convert to characters
            let mut digits: Vec<_> = input[idx_r][idx_c].chars().collect();
            let n_digits = digits.len();
            for _ in 0..(max_n_chars - n_digits) {
                digits.insert(0, '-');
            }
            v.push(digits);
        }
        println!("Digits: {v:?}");
        // Locally override scope here, as v is the new vector we care about
        let n_rows = v.len();
        let n_cols = max_n_chars;
        println!("n_rows: {n_rows}\tn_cols: {n_cols}");
        let math = match sign {
            "+" => {
                let mut s = 0;
                for idx_c in 0..n_cols {
                    let mut d = 0;
                    // Construct the digit
                    'a: for idx_r in 0..n_rows {
                        if v[idx_r][idx_c] == '-' {
                            continue 'a;
                        }
                        print!(
                            "d: {d}\tv: {}\tidx_r: {idx_r}\tidx_c: {idx_c}",
                            v[idx_r][idx_c]
                        );
                        d = d * 10 + v[idx_r][idx_c].to_digit(10).unwrap() as usize;
                        println!("\td: {d}");
                    }
                    s += d;
                    println!("Adding d: {d}\ts: {s}");
                }
                s
            }
            "*" => {
                let mut m = 1;
                for idx_c in 0..n_cols {
                    let mut d = 0;
                    // Construct the digit
                    'a: for idx_r in 0..n_rows {
                        if v[idx_r][idx_c] == '-' {
                            continue 'a;
                        }
                        print!(
                            "d: {d}\tv: {}\tidx_r: {idx_r}\tidx_c: {idx_c}",
                            v[idx_r][idx_c]
                        );
                        d = d * 10 + v[idx_r][idx_c].to_digit(10).unwrap() as usize;
                        println!("\td: {d}");
                    }
                    m *= d;
                    println!("Multiplying d: {d}\tm: {m}");
                }
                m
            }
            _ => panic!("Invalid sign"),
        };
        big_sum += math;
    }
    big_sum
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

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 4277556);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 3263827);
    }
}
