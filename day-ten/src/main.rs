//! Command line executable for running part one and part two
use std::time::Instant;

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

/// Machine
#[derive(Debug, Clone)]
struct Machine {
    /// Light diagram
    light_diagram: IndicatorLights,
    /// Button wiring schematics
    buttons: Vec<Button>,
    /// Joltage requirements
    joltage_requirements: Vec<u32>, // This will change
}
impl Machine {
    pub fn from_line(line: &str) -> Machine {
        let line = line.trim();
        // Parse indicator lights
        let idx_start = line.find("[").unwrap();
        let idx_stop = line.find("]").unwrap();
        let light_diagram = IndicatorLights::from_str(&line[idx_start + 1..idx_stop]);

        let idx_start = idx_stop + 2;
        let idx_stop = line.find("{").unwrap() - 2;
        let buttons = line[idx_start..=idx_stop]
            .split_whitespace()
            .map(Button::from_str)
            .collect();

        let idx_start = idx_stop + 3;
        let idx_end = line.len() - 1;
        let joltage_requirements = line[idx_start..idx_end]
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect();
        Self {
            light_diagram,
            buttons,
            joltage_requirements,
        }
    }

    fn build_equations(&self) -> Vec<Equation> {
        self.light_diagram
            .inner
            .iter()
            .enumerate()
            .map(|(light_idx, status)| {
                let mut row = 0u64;

                for (btn_idx, btn) in self.buttons.iter().enumerate() {
                    if btn.lights_affected.contains(&light_idx) {
                        row |= 1 << btn_idx;
                    }
                }
                let rhs = matches!(status, LightStatus::On);
                Equation { row, rhs }
            })
            .collect()
    }

    fn build_joltage_equations(&self) -> Vec<(u64, u32)> {
        self.joltage_requirements
            .iter()
            .enumerate()
            .map(|(idx, target)| {
                let mut row = 0u64;
                for (j, button) in self.buttons.iter().enumerate() {
                    if button.lights_affected.contains(&idx) {
                        row |= 1 << j;
                    }
                }
                (row, *target)
            })
            .collect()
    }

    /// Use Gaussian elimination to simplify the equations
    ///
    /// Trying to solve Ax = b (mod 2)
    fn gaussian_elim_gf2(mut eqs: Vec<Equation>, n_buttons: usize) -> Option<(u64, Vec<u64>)> {
        // Bookkeeping -- which row is the pivot off the column `col`
        let mut pivot_col = vec![None; n_buttons];
        // Current pivot row during elimination
        let mut row = 0;

        // Iterate through the various columns trying to remove redundent scenarios
        for (col, item) in pivot_col.iter_mut().enumerate() {
            // Look for a row >= row where variable col appears with coefficient 1
            let pivot = (row..eqs.len()).find(|&r| (eqs[r].row >> col) & 1 == 1);
            if pivot.is_none() {
                // If non exist, this variable is free, because this button does not affect any of
                // the outcomes -- skip this column
                continue;
            }
            let pivot = pivot.unwrap();

            // Standard Gaussian elimination -- swap pivot row upward, record where the pivot lives
            eqs.swap(row, pivot);
            *item = Some(row);

            // Eliminate this button from all other rows
            for r in 0..eqs.len() {
                // if row r has a 1 in this pivot column, then subtract pivot row from it -- this
                // zeros out column col in row r and preserves the equation's validity
                if r != row && ((eqs[r].row >> col) & 1) == 1 {
                    eqs[r].row ^= eqs[row].row;
                    eqs[r].rhs ^= eqs[row].rhs;
                }
            }
            row += 1;
        }

        // Consistency check -- detects 0 == 1 mod 2
        for eq in &eqs {
            if eq.row == 0 && eq.rhs {
                return None; // No solution
            }
        }
        // Particular solution (set free vars = 0)
        // Build one concrete solution x
        let mut particular = 0u64;
        for (col, item) in pivot_col.iter().enumerate() {
            if let Some(r) = item
                && eqs[*r].rhs
            {
                particular |= 1 << col;
            }
        }

        // Nullspace basis
        let mut nullspace = Vec::new();
        for free_col in 0..n_buttons {
            if pivot_col[free_col].is_none() {
                // Start with free variable == 1, all others == 0
                let mut vec = 1u64 << free_col;
                // Enforces A vec = 0 -- turning on this free variable forces some pivot variables
                // to flip, so overall effect is no lights change
                for (col, item) in pivot_col.iter().enumerate() {
                    if let Some(r) = item
                        && ((eqs[*r].row >> free_col) & 1) == 1
                    {
                        vec |= 1 << col;
                    }
                }
                nullspace.push(vec);
            }
        }

        Some((particular, nullspace))
    }

    pub fn find_min_button_presses(&self) -> usize {
        let equations = self.build_equations();
        let n_buttons = self.buttons.len();

        let (particular, nullspace) =
            Self::gaussian_elim_gf2(equations, n_buttons).expect("Machine has no solution");

        let mut best = particular.count_ones() as usize;
        let k = nullspace.len();

        // Brute force nullspace (usually small)
        for mask in 0..(1u64 << k) {
            let mut x = particular;
            for (i, nspace) in nullspace.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    x ^= nspace;
                }
            }
            best = best.min(x.count_ones() as usize);
        }

        best
    }

    fn dfs(eqs: &[(u64, u32)], idx: usize, x: &mut Vec<u32>, best: &mut u32) {
        if idx == x.len() {
            if eqs.iter().all(|(row, rhs)| {
                let sum: u32 = x
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| (row >> j) & 1 == 1)
                    .map(|(_, v)| *v)
                    .sum();
                sum == *rhs
            }) {
                *best = (*best).min(x.iter().sum());
            }
            return;
        }

        for v in 0..=*best {
            x[idx] = v;
            if x.iter().take(idx + 1).sum::<u32>() >= *best {
                break;
            }
            Self::dfs(eqs, idx + 1, x, best);
        }
    }

    pub fn find_min_button_presses_pt_2(&self) -> usize {
        let eqs = self.build_joltage_equations();

        let n_buttons = self.buttons.len();

        let mut x = vec![0u32; n_buttons];

        let rhs_max = eqs.iter().map(|(_, rhs)| *rhs).max().unwrap_or(0);
        let mut best = rhs_max * n_buttons as u32;

        Self::dfs(&eqs, 0, &mut x, &mut best);

        best as usize
    }
}

/// Indicator lights
#[derive(Debug, Clone)]
struct IndicatorLights {
    inner: Vec<LightStatus>,
}
impl IndicatorLights {
    pub fn from_str(s: &str) -> Self {
        let inner = s.chars().map(|c| c.into()).collect();
        Self { inner }
    }
}

/// Light statuses
#[derive(Debug, Clone)]
enum LightStatus {
    On,
    Off,
}
impl From<char> for LightStatus {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::On,
            '.' => Self::Off,
            _ => panic!("Not valid"),
        }
    }
}

/// Button
#[derive(Debug, Clone)]
struct Button {
    lights_affected: Vec<usize>,
}
impl Button {
    pub fn from_str(s: &str) -> Self {
        // Assume comes in the form of (...), where ... can be any number of buttons
        // Remove the ends
        let mut s = s.trim();
        s = &s[1..s.len() - 1];
        let lights_affected = s.split(",").map(|c| c.parse().unwrap()).collect();
        Self { lights_affected }
    }
}

/// One equtions: (row * x) = rhs (mod 2)
#[derive(Debug, Clone)]
struct Equation {
    row: u64, // assuming <=64 buttons, bitset
    rhs: bool,
}

fn part_one(s: &str) -> usize {
    s.lines().map(Machine::from_line).fold(0, |accum, machine| {
        accum + machine.find_min_button_presses()
    })
}

fn part_two(s: &str) -> usize {
    s.lines().map(Machine::from_line).fold(0, |accum, machine| {
        accum + machine.find_min_button_presses_pt_2()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    }

    #[test]
    fn test_one() {
        let output = part_one(input_one());

        // TODO fill this out
        assert_eq!(output, 7);
    }

    #[test]
    fn test_two() {
        let output = part_two(input_one());

        // TODO fill this out
        assert_eq!(output, 33);
    }
}
