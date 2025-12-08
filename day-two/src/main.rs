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

fn parse_input(mut file: BufReader<File>) -> Vec<Range> {
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Failed to read");
    s.split(",").map(Range::new).collect()
}

// TODO -- Update this with the return type
type ReturnType = usize;
type VectorType = Range;
type VectorType2 = Range;

/// Range Entry to validate
#[derive(Debug, Clone)]
pub struct RangeEntry(usize);
impl RangeEntry {
    /// Validate according to the rules outlined
    pub fn validate(&self) -> bool {
        !Self::check_for_repeats(self.0) && !Self::check_if_starts_with_zero(self.0)
    }

    pub fn validate_two(&self) -> bool {
        !Self::check_for_repeats_part2(self.0) && !Self::check_if_starts_with_zero(self.0)
    }

    fn check_if_starts_with_zero(val: usize) -> bool {
        let s = val.to_string();
        s.chars().next().expect("No characters") == '0'
    }

    fn check_for_repeats(val: usize) -> bool {
        let s = val.to_string();
        // The possible max length of a pattern is the floor of the length of the string
        let l = (s.len() as f32 / 2.0).ceil() as usize;
        // Determine the number of times you'll have a first pointer
        let n_chunks = s.len() / l;
        let base_iter = s.chars();
        for chunk_num in 0..n_chunks {
            let first_chunk: String = base_iter.clone().skip(chunk_num * l).take(l).collect();
            let second_chunk: String = base_iter
                .clone()
                .skip((chunk_num + 1) * l)
                .take(l)
                .collect();
            if first_chunk == second_chunk {
                return true;
            }
        }
        false
    }

    /// We must go through, divide into different size chunks, and see if all chunks are the same
    /// So, we can start with 1 to max size of chunks
    /// The first chunk is the truth
    /// Then, look at all other chunks and see if they match
    fn check_for_repeats_part2(val: usize) -> bool {
        let s = val.to_string();
        // The possible max length of a pattern is the floor of the length of the string
        let max_length = (s.len() as f32 / 2.0).ceil() as usize;
        // Look at chunk sizes from 1 to the max_length
        for l in 1..=max_length {
            // We can only use this chunk length if we can evenly divide the number of chunks
            if !s.len().is_multiple_of(l) {
                continue;
            }
            // Determine the number of times you'll have a first pointer
            let n_chunks = s.len() / l;
            let base_iter = s.chars();
            let first_chunk: String = base_iter.clone().take(l).collect();
            let mut chunks_are_all_the_same = true;
            for chunk_num in 1..n_chunks {
                // Determine the first chunk
                // Continue to look at chunks until
                let second_chunk: String = base_iter.clone().skip(chunk_num * l).take(l).collect();
                chunks_are_all_the_same = chunks_are_all_the_same && first_chunk == second_chunk;
            }
            if chunks_are_all_the_same {
                println!(
                    "Val: {s}\tChunk size: {l}\tFirst: {first_chunk:?}\tLast: {:?}\tEqual: {:?}",
                    base_iter
                        .clone()
                        .skip(l * (n_chunks - 1))
                        .take(l)
                        .collect::<String>(),
                    first_chunk
                        == base_iter
                            .skip(l * (n_chunks - 1))
                            .take(l)
                            .collect::<String>()
                );
                return true;
            }
        }
        false
    }
}

/// Range -- consists of 2 range entries
#[derive(Debug, Clone)]
pub struct Range {
    left: usize,
    right: usize,
}
impl Range {
    pub fn new(entry: &str) -> Self {
        let mut vals = entry.split("-");
        Self {
            left: vals.next().expect("Need 2 sides").trim().parse().unwrap(),
            right: vals.next().expect("Need 2 sides").trim().parse().unwrap(),
        }
    }

    pub fn invalid_ids(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for val in self.left..=self.right {
            if !RangeEntry(val).validate() {
                v.push(val)
            }
        }
        v
    }

    pub fn invalid_ids_part2(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for val in self.left..=self.right {
            if !RangeEntry(val).validate_two() {
                v.push(val)
            }
        }
        v
    }
}

/// Internal logic for part_one
fn part_one_internal(input: Vec<VectorType>) -> ReturnType {
    input.into_iter().fold(0, |acc, range| {
        acc + range.invalid_ids().into_iter().sum::<ReturnType>()
    })
}

/// Internal logic for part two
fn part_two_internal(input: Vec<VectorType2>) -> ReturnType {
    input.into_iter().fold(0, |acc, range| {
        acc + range.invalid_ids_part2().into_iter().sum::<ReturnType>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    /// Function to split above into different inputs
    fn parse_input_test(input: &str) -> Vec<Range> {
        input.split(",").map(Range::new).collect()
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 1227775554);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 4174379265);
    }

    #[test]
    fn test_range_entry() {
        assert!(RangeEntry::check_for_repeats(11));
        assert!(RangeEntry::check_for_repeats(22));
        assert!(!RangeEntry::check_for_repeats(23));
        assert!(RangeEntry::check_for_repeats(6464));
        assert!(RangeEntry::check_for_repeats(123123));
        assert!(!RangeEntry::check_for_repeats(123124));
    }

    #[test]
    fn test_range_entry_part2() {
        assert!(RangeEntry::check_for_repeats_part2(11));
        assert!(RangeEntry::check_for_repeats_part2(22));
        assert!(!RangeEntry::check_for_repeats_part2(23));
        assert!(RangeEntry::check_for_repeats_part2(6464));
        assert!(RangeEntry::check_for_repeats_part2(123123));
        assert!(RangeEntry::check_for_repeats_part2(565656));
        assert!(RangeEntry::check_for_repeats_part2(824824824));
        assert!(!RangeEntry::check_for_repeats_part2(123124));
    }

    #[test]
    fn test_part_one_deeper() {
        let r = Range::new("11-22");
        assert_eq!(r.invalid_ids(), vec![11, 22]);
        let r = Range::new("95-115");
        assert_eq!(r.invalid_ids(), vec![99]);
    }

    #[test]
    fn test_part_two_deeper() {
        let r = Range::new("11-22");
        assert_eq!(r.invalid_ids_part2(), vec![11, 22]);
        let r = Range::new("95-115");
        assert_eq!(r.invalid_ids_part2(), vec![99, 111]);
    }
}
