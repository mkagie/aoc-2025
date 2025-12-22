//! Command line executable for running part one and part two
use std::collections::HashMap;
use std::hash::RandomState;
use std::time::Instant;

use clap::Parser;
use petgraph::algo::all_simple_paths;
use petgraph::prelude::*;

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

/// Graph Manager
#[derive(Debug, Clone)]
struct GraphManager {
    graph: Graph<String, i32>,
    nodes: HashMap<String, NodeIndex>,
}
impl GraphManager {
    pub fn new(input: &str) -> Self {
        let mut graph = Graph::new();
        let mut nodes = HashMap::new();
        input.lines().for_each(|line| {
            let node = line.split(":").next().unwrap().to_string();
            let connected_to: Vec<_> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();
            if !nodes.contains_key(&node) {
                let idx = graph.add_node(node.clone());
                nodes.insert(node.clone(), idx);
            }
            let source_idx = *nodes.get(&node).unwrap();
            for node in &connected_to {
                if !nodes.contains_key(node) {
                    let idx = graph.add_node(node.clone());
                    nodes.insert(node.clone(), idx);
                }
                // Create edges
                let dep_idx = *nodes.get(node).unwrap();
                graph.add_edge(source_idx, dep_idx, 1);
            }
        });
        Self { graph, nodes }
    }

    pub fn part_one(&self) -> usize {
        let you_idx = *self.nodes.get("you").unwrap();
        let out_idx = *self.nodes.get("out").unwrap();
        let all_paths =
            all_simple_paths::<Vec<_>, _, RandomState>(&self.graph, you_idx, out_idx, 0, None);
        all_paths.count()
    }

    pub fn part_two(&self) -> usize {
        let svr_idx = *self.nodes.get("svr").unwrap();
        let dac_idx = *self.nodes.get("dac").unwrap();
        let fft_idx = *self.nodes.get("fft").unwrap();
        let out_idx = *self.nodes.get("out").unwrap();
        let max_intermediate = Some(17);
        let n_paths_svr2dac = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("svr2dac".into())
                .spawn(move || {
                    println!("Starting with svr2dac");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        svr_idx,
                        dac_idx,
                        0,
                        max_intermediate,
                    )
                    // Filter out to make sure we do not already go there
                    .filter(|x| !x.contains(&fft_idx) && !x.contains(&out_idx))
                    .count();
                    println!("Completed with svr2dac");
                    count
                })
                .unwrap()
        };
        let n_paths_svr2fft = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("svr2fft".into())
                .spawn(move || {
                    println!("Starting with svr2fft");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        svr_idx,
                        fft_idx,
                        0,
                        max_intermediate,
                    )
                    .filter(|x| !x.contains(&dac_idx) && !x.contains(&out_idx))
                    .count();
                    println!("Completed with svr2fft");
                    count
                })
                .unwrap()
        };
        let n_paths_dac2fft = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("dac2fft".into())
                .spawn(move || {
                    println!("Starting with dac2fft");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        dac_idx,
                        fft_idx,
                        0,
                        max_intermediate,
                    )
                    .filter(|x| !x.contains(&svr_idx) && !x.contains(&out_idx))
                    .count();
                    println!("Completed with dac2fft");
                    count
                })
                .unwrap()
        };
        let n_paths_fft2dac = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("fft2dac".into())
                .spawn(move || {
                    println!("Starting with fft2dac");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        fft_idx,
                        dac_idx,
                        0,
                        max_intermediate,
                    )
                    .filter(|x| !x.contains(&svr_idx) && !x.contains(&out_idx))
                    .count();
                    println!("Completed with fft2dac");
                    count
                })
                .unwrap()
        };
        let n_paths_dac2out = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("dac2out".into())
                .spawn(move || {
                    println!("Starting with dac2out");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        dac_idx,
                        out_idx,
                        0,
                        max_intermediate,
                    )
                    .filter(|x| !x.contains(&svr_idx) && !x.contains(&fft_idx))
                    .count();
                    println!("Completed with dac2out");
                    count
                })
                .unwrap()
        };
        let n_paths_fft2out = {
            let graph = self.graph.clone();
            std::thread::Builder::new()
                .name("fft2out".into())
                .spawn(move || {
                    println!("Starting with fft2out");
                    let count = all_simple_paths::<Vec<_>, _, RandomState>(
                        &graph,
                        fft_idx,
                        out_idx,
                        0,
                        max_intermediate,
                    )
                    .filter(|x| !x.contains(&svr_idx) && !x.contains(&dac_idx))
                    .count();
                    println!("Completed with fft2out");
                    count
                })
                .unwrap()
        };

        // Path from svr -> dac -> fft -> out
        let path0 = n_paths_svr2dac.join().unwrap()
            * n_paths_dac2fft.join().unwrap()
            * n_paths_fft2out.join().unwrap();
        // Path from svr -> fft -> dac -> out
        let path1 = n_paths_svr2fft.join().unwrap()
            * n_paths_fft2dac.join().unwrap()
            * n_paths_dac2out.join().unwrap();
        path0 + path1
    }
}

fn part_one(s: &str) -> usize {
    let manager = GraphManager::new(s);
    manager.part_one()
}

fn part_two(s: &str) -> usize {
    let manager = GraphManager::new(s);
    manager.part_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Function to modify for input to test
    fn input_one() -> &'static str {
        "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
    }

    fn input_two() -> &'static str {
        "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
    }

    #[test]
    fn test_one() {
        let output = part_one(input_one());

        // TODO fill this out
        assert_eq!(output, 5);
    }

    #[test]
    fn test_two() {
        let output = part_two(input_two());

        // TODO fill this out
        assert_eq!(output, 2);
    }
}
