//! Command line executable for running part one and part two
use std::{
    collections::{HashMap, HashSet},
    f32,
    time::Instant,
};

use clap::Parser;
use nalgebra::{DMatrix, Vector3};

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

/// Creates UUIDs
#[derive(Debug, Clone, Default)]
struct UuidGenerator {
    inner: usize,
}
impl UuidGenerator {
    pub fn get_next(&mut self) -> usize {
        let output = self.inner;
        self.inner += 1;
        output
    }
}

/// Circuit manager
///
/// Needs to keep track of the various circuits, connect them together, and query if 2 things are
/// connected
/// We need something tha4t maps position to circuit and circuit to positions in the circuit
#[derive(Debug, Clone)]
struct CircuitManager {
    uuid_gen: UuidGenerator,
    /// Mapping from position (idx) to circuit
    position_to_circuit: HashMap<usize, usize>,
    /// Mapping from circuit to what positions it contains (idx)
    circuit_to_position: HashMap<usize, HashSet<usize>>,
}
impl CircuitManager {
    pub fn new(poses: &[Vector3<usize>]) -> Self {
        let mut uuid_gen = UuidGenerator::default();

        // Create position to circuit and circuit to position
        let mut position_to_circuit = HashMap::new();
        let mut circuit_to_position = HashMap::new();
        for idx in 0..poses.len() {
            let uuid = uuid_gen.get_next();
            position_to_circuit.insert(idx, uuid);
            let mut s = HashSet::new();
            s.insert(idx);
            circuit_to_position.insert(uuid, s);
        }
        Self {
            uuid_gen,
            position_to_circuit,
            circuit_to_position,
        }
    }

    pub fn try_combine(&mut self, idx0: usize, idx1: usize) -> bool {
        // Make sure they do not belong to the same circuit
        let cid0 = self.position_to_circuit.get(&idx0).unwrap();
        let cid1 = self.position_to_circuit.get(&idx1).unwrap();
        if cid0 == cid1 {
            // They are already in the same circuit, return false
            return false;
        }
        // Remove circuit 0 and circuit 1 from the circuit to position
        let c0 = self.circuit_to_position.remove(cid0).unwrap();
        let c1 = self.circuit_to_position.remove(cid1).unwrap();
        // create a new circuit and mark that all of the positions in the c0 and c1 are now in that
        // circuit
        // modify position to circuit for each of the new positions to the new circuit
        let mut new_c = HashSet::new();
        let new_cid = self.uuid_gen.get_next();
        for pid in c0 {
            new_c.insert(pid);
            *self.position_to_circuit.get_mut(&pid).unwrap() = new_cid;
        }
        for pid in c1 {
            new_c.insert(pid);
            *self.position_to_circuit.get_mut(&pid).unwrap() = new_cid;
        }
        self.circuit_to_position.insert(new_cid, new_c);
        true
    }

    pub fn part_one(&self) -> usize {
        // We need to determine the 3 largest circuits
        let mut circuit_sizes: Vec<_> = self
            .circuit_to_position
            .values()
            .map(|pos_idxs| pos_idxs.len())
            .collect();
        circuit_sizes.sort();
        circuit_sizes.reverse();
        circuit_sizes
            .into_iter()
            .take(3)
            .reduce(|accum, x| accum * x)
            .unwrap()
    }

    pub fn is_one_large_circuit(&self) -> bool {
        self.circuit_to_position.len() == 1
    }
}

/// Distance manager
#[derive(Debug, Clone)]
struct DistanceManager {
    distances: DMatrix<f32>,
    ordered_distances: Vec<(usize, usize)>,
}
impl DistanceManager {
    pub fn new(poses: &[Vector3<usize>]) -> Self {
        let n_poses = poses.len();
        // Create a distance matrix
        let mut distances = DMatrix::from_element(n_poses, n_poses, f32::INFINITY);
        let mut distances_list = Vec::new();
        let mut idx_list = Vec::new();
        for idx0 in 0..n_poses - 1 {
            let pos0 = unsafe { poses.get_unchecked(idx0) };
            for idx1 in idx0 + 1..n_poses {
                let pos1 = unsafe { poses.get_unchecked(idx1) };
                let distance = (pos0.cast::<f32>() - pos1.cast::<f32>()).norm();
                distances[(idx0, idx1)] = distance;
                distances[(idx1, idx0)] = distance;
                distances_list.push(distance);
                idx_list.push((idx0, idx1));
            }
        }
        // Now, we need to order the idx_list by distances
        idx_list.sort_by(|idx0, idx1| {
            let d0: f32 = distances[*idx0];
            let d1: f32 = distances[*idx1];
            d0.partial_cmp(&d1).unwrap()
        });
        idx_list.reverse();
        Self {
            distances,
            ordered_distances: idx_list,
        }
    }

    pub fn next(&mut self) -> (usize, usize) {
        self.ordered_distances.pop().unwrap()
    }

    /// External API to say we connected 2 circuits
    pub fn connect(&mut self, idx0: usize, idx1: usize) {
        self.distances[(idx0, idx1)] = f32::INFINITY;
        self.distances[(idx1, idx0)] = f32::INFINITY;
    }
}

/// Manager
#[derive(Debug, Clone)]
struct Manager {
    poses: Vec<Vector3<usize>>,
    distance_manager: DistanceManager,
    circuit_manager: CircuitManager,
}
impl Manager {
    pub fn new(s: &str) -> Self {
        let poses: Vec<Vector3<usize>> = s
            .lines()
            .map(|line| {
                let mut nums = line.split(",").map(|s| s.parse().unwrap());
                Vector3::new(
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                    nums.next().unwrap(),
                )
            })
            .collect();
        let distance_manager = DistanceManager::new(&poses);
        let circuit_manager = CircuitManager::new(&poses);
        Self {
            poses,
            distance_manager,
            circuit_manager,
        }
    }

    pub fn part_one(&mut self, n_iters: usize) -> usize {
        for _ in 0..n_iters {
            // Find the shortest
            // let (idx0, idx1, _) = self.distance_manager.argmin();
            let (idx0, idx1) = self.distance_manager.next();
            let _ = self.circuit_manager.try_combine(idx0, idx1);
            // Regardless of whether or not this is an actual connection, for the purposes of the
            // distaance manager, we should connect them
            self.distance_manager.connect(idx0, idx1);
        }

        self.circuit_manager.part_one()
    }

    pub fn part_two(&mut self) -> usize {
        loop {
            // Find the shortest
            // let (idx0, idx1, _) = self.distance_manager.argmin();
            let (idx0, idx1) = self.distance_manager.next();
            let _ = self.circuit_manager.try_combine(idx0, idx1);
            // Regardless of whether or not this is an actual connection, for the purposes of the
            // distaance manager, we should connect them
            self.distance_manager.connect(idx0, idx1);

            if self.circuit_manager.is_one_large_circuit() {
                // Multiple the xs of idx0 and idx1
                let p0 = self.poses[idx0];
                let p1 = self.poses[idx1];
                return p0.x * p1.x;
            }
        }
    }
}

fn part_one(s: &str) -> usize {
    let mut manager = Manager::new(s);
    manager.part_one(1000)
}

fn part_two(s: &str) -> usize {
    let mut manager = Manager::new(s);
    manager.part_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
    }

    #[test]
    fn test_one() {
        let mut manager = Manager::new(input_one());
        let output = manager.part_one(10);

        // TODO fill this out
        assert_eq!(output, 40);
    }

    #[test]
    fn test_two() {
        let mut manager = Manager::new(input_one());
        let output = manager.part_two();

        // TODO fill this out
        assert_eq!(output, 25272);
    }
}
