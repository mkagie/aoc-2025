//! Command line executable for running part one and part two
use std::time::Instant;

use clap::Parser;
use nalgebra::DMatrix;

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
    /// Internal indicator lights
    indicator_lights: IndicatorLights,
    /// Button wiring schematics
    buttons: Vec<Button>,
    /// Joltage requirements
    joltage_requirements: String, // This will change
}
impl Machine {
    pub fn from_line(line: &str) -> Machine {
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

        let idx_start = idx_stop + 2;
        let joltage_requirements = line[idx_start..].to_string();
        Self {
            indicator_lights: light_diagram.initialize_new(),
            light_diagram,
            buttons,
            joltage_requirements,
        }
    }

    fn buttons_to_matrix(&self) -> DMatrix<usize> {
        let n_lights = self.light_diagram.inner.len();
        let n_buttons = self.buttons.len();
        println!("Array should be {n_lights}x{n_buttons}");
        let rows: Vec<_> = self
            .buttons
            .iter()
            .map(|button| button.to_column_vector(n_buttons))
            .collect();
        println!("rows: {rows:?}");
        DMatrix::from_fn(n_lights, n_buttons, |row, col| {
            println!("Row: {row}\tCol: {col}");

            rows[row][(0, col)]
        })
    }

    pub fn find_min_button_presses(&self) -> usize {
        let A = self.buttons_to_matrix();
        println!("A: {A}");
        let x = DMatrix::from_element(self.buttons.len(), 1, 1);
        let b = A * x;
        println!("b: {b}");
        let is_valid = self.indicator_lights.validate(&b);
        println!("Valid?: {is_valid}");
        0
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

    pub fn toggle(&mut self, button: &Button) {
        for idx in &button.lights_affected {
            self.inner.get_mut(*idx).unwrap().toggle();
        }
    }

    pub fn initialize_new(&self) -> Self {
        let inner = self.inner.iter().map(|_| LightStatus::Off).collect();
        Self { inner }
    }

    pub fn validate(&self, b: &DMatrix<usize>) -> bool {
        for idx in 0..self.inner.len() {
            let output = b[idx];
            match self.inner[idx] {
                LightStatus::On => {
                    if output.is_multiple_of(2) {
                        return false;
                    }
                }
                LightStatus::Off => {
                    if !output.is_multiple_of(2) {
                        return false;
                    }
                }
            }
        }
        true
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
impl LightStatus {
    pub fn toggle(&mut self) {
        match self {
            LightStatus::On => *self = LightStatus::Off,
            LightStatus::Off => *self = LightStatus::On,
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

    pub fn to_column_vector(&self, n_lights: usize) -> DMatrix<usize> {
        let mut mat = DMatrix::zeros(1, n_lights);
        for idx in &self.lights_affected {
            mat[*idx] = 1;
        }
        mat
    }
}

fn part_one(s: &str) -> usize {
    s.lines().map(Machine::from_line).fold(0, |accum, machine| {
        accum + machine.find_min_button_presses()
    })
}

fn part_two(s: &str) -> usize {
    todo!()
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
        assert_eq!(output, 0);
    }
}
