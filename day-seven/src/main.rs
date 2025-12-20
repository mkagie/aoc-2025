//! Command line executable for running part one and part two
use std::{
    collections::{HashMap, HashSet},
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

fn parse_input(mut file: BufReader<File>) -> Manager {
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let manifold = TachyonManifold::new(&s);
    Manager::new(manifold)
}

// TODO -- Update this with the return type
type ReturnType = usize;

/// Type of Spot
#[derive(Debug, Clone)]
enum TachyonEntry {
    Start,
    Splitter,
    Open,
}
impl TachyonEntry {
    pub fn from_char(c: &char) -> Self {
        match c {
            '.' => Self::Open,
            '^' => Self::Splitter,
            'S' => Self::Start,
            _ => panic!("Not a valid entry"),
        }
    }
}

/// Tachyon Manifold
#[derive(Debug, Clone)]
struct TachyonManifold {
    inner: Vec<Vec<TachyonEntry>>,
    n_rows: usize,
    n_cols: usize,
}
impl TachyonManifold {
    pub fn new(input: &str) -> Self {
        let inner: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(|c| TachyonEntry::from_char(&c)).collect())
            .collect();
        Self {
            n_rows: inner.len(),
            n_cols: inner[0].len(),
            inner,
        }
    }

    pub fn get_start(&self) -> (usize, usize) {
        for idx_r in 0..self.n_rows {
            for idx_c in 0..self.n_cols {
                if matches!(self.inner[idx_r][idx_c], TachyonEntry::Start) {
                    return (idx_r, idx_c);
                }
            }
        }
        panic!("Could not find start")
    }

    pub fn query_location(
        &self,
        idx_r: usize,
        idx_c: usize,
        offset_r: isize,
        offset_c: isize,
    ) -> Option<&TachyonEntry> {
        let new_r = {
            let t = idx_r as isize + offset_r;
            if t < 0 {
                return None;
            } else {
                t as usize
            }
        };
        if new_r >= self.n_rows {
            return None;
        }

        let new_c = {
            let t = idx_c as isize + offset_c;
            if t < 0 {
                return None;
            } else {
                t as usize
            }
        };
        if new_c >= self.n_cols {
            return None;
        }
        Some(&self.inner[new_r][new_c])
    }
}

/// Tachyon Beam
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct TachyonBeam {
    pos_r: usize,
    pos_c: usize,
}
impl TachyonBeam {
    pub fn evolve(self, manifold: &TachyonManifold) -> Vec<Self> {
        let mut v = Vec::new();
        if let Some(entry) = manifold.query_location(self.pos_r, self.pos_c, 1, 0) {
            match entry {
                TachyonEntry::Start => panic!("This doesn't make sense"),
                TachyonEntry::Splitter => {
                    // We need to create a new on left and right
                    // left
                    if manifold
                        .query_location(self.pos_r, self.pos_c, 1, -1)
                        .is_some()
                    {
                        v.push(TachyonBeam {
                            pos_r: self.pos_r + 1,
                            pos_c: (self.pos_c as isize - 1) as usize,
                        });
                        // right
                        if manifold
                            .query_location(self.pos_r, self.pos_c, 1, 1)
                            .is_some()
                        {
                            v.push(TachyonBeam {
                                pos_r: self.pos_r + 1,
                                pos_c: self.pos_c + 1,
                            });
                        }
                    }
                }
                TachyonEntry::Open => {
                    // If it is open, we can inhabit
                    v.push(TachyonBeam {
                        pos_r: self.pos_r + 1,
                        pos_c: self.pos_c,
                    })
                }
            }
        };
        v
    }
}

/// A Timeline is a set of locations that eventually reach the end
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Timeline(Vec<TachyonBeam>);

/// Counter
#[derive(Debug, Clone)]
struct Manager {
    manifold: TachyonManifold,
}
impl Manager {
    pub fn new(manifold: TachyonManifold) -> Self {
        Self { manifold }
    }

    pub fn run_p1(self) -> usize {
        // Create the first beam
        let (pos_r, pos_c) = self.manifold.get_start();
        let mut beams = HashSet::new();
        beams.insert(TachyonBeam { pos_r, pos_c });
        let mut ctr = 0;
        while !beams.is_empty() {
            let mut new_beams = HashSet::new();
            for beam in beams {
                let evolved_beams = beam.evolve(&self.manifold);
                if evolved_beams.len() == 2 {
                    // We split, increment the pt1_ctr
                    ctr += 1;
                }
                new_beams.extend(evolved_beams);
            }
            beams = new_beams;
        }
        ctr
    }

    pub fn run_p2(self) -> usize {
        let (pos_r, pos_c) = self.manifold.get_start();
        let current_timeline = Timeline(vec![TachyonBeam { pos_r, pos_c }]);
        let mut prev_seen = HashMap::new();
        Self::pt2_recursive_count(&self.manifold, &mut prev_seen, current_timeline)
    }

    pub fn pt2_recursive_count(
        manifold: &TachyonManifold,
        prev_seen: &mut HashMap<TachyonBeam, usize>,
        current_timeline: Timeline,
    ) -> usize {
        let v = &current_timeline.0;
        let beam = v.last().unwrap().clone();
        // Check if we have seen this before
        if let Some(i) = prev_seen.get(&beam) {
            return *i;
        }
        let evolved_beams = beam.clone().evolve(manifold);
        if evolved_beams.is_empty() {
            return 1;
        }
        let mut count = 0;
        for beam in evolved_beams {
            let mut new_v = v.clone();
            new_v.push(beam);
            count += Self::pt2_recursive_count(manifold, prev_seen, Timeline(new_v));
        }
        // Store in previously seen for future
        prev_seen.insert(beam, count);
        count
    }
}

/// Internal logic for part_one
fn part_one_internal(input: Manager) -> ReturnType {
    input.run_p1()
}

/// Internal logic for part two
fn part_two_internal(input: Manager) -> ReturnType {
    input.run_p2()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
    }

    /// Function to split above into different inputs
    fn parse_input_test(input: &str) -> Manager {
        let manifold = TachyonManifold::new(input);
        Manager::new(manifold)
    }

    #[test]
    fn test_one() {
        let input = parse_input_test(input_one());
        let output = part_one_internal(input);

        // TODO fill this out
        assert_eq!(output, 21);
    }

    #[test]
    fn test_two() {
        let input = parse_input_test(input_one());
        let output = part_two_internal(input);

        // TODO fill this out
        assert_eq!(output, 40);
    }
}
