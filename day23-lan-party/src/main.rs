use std::collections::{HashMap,HashSet};
use std::convert::TryInto;
use std::net;

type Address = String;

#[derive(Debug)]
struct Puzzle {
    connections: HashMap<Address, Vec<Address>>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let mut connections = HashMap::new();

        for line in input.lines() {
            let (left, right) = line.split_once("-").unwrap();
            let left: Address = left.try_into().unwrap();
            let right: Address = right.try_into().unwrap();

            connections.entry(left.clone()).or_insert_with(Vec::new).push(right.clone());
            connections.entry(right).or_insert_with(Vec::new).push(left);
        }

        Self {
            connections,
        }
    }
}

impl Puzzle {
    fn networks_of_size(&self, size: usize) -> HashSet<[Address;3]> {
        let mut networks = HashSet::new();
        let mut queue: Vec<Vec<Address>> = self.connections.keys().map(|address| vec![address.clone()]).collect();

        while let Some(network) = queue.pop() {
            let last = network.last().unwrap();
            let neighbors = self.connections.get(last).unwrap();

            for neighbor in neighbors {
                if network.first() == Some(neighbor) {
                    if network.len() == size {
                        let mut array: [Address;3] = Default::default();
                        array.clone_from_slice(&network[..3]);
                        array.sort(); // Order of addresses doesn't matter
                        networks.insert(array);
                    }
                    continue;
                }

                if network.len() >= size {
                    continue;
                }

                queue.push(network.iter().cloned().chain(std::iter::once(neighbor.clone())).collect());
            }
        }

        networks
    }
}

fn main() {
    let input = aoc::input();
    let puzzle = Puzzle::from(input.as_str());

    let size = puzzle.networks_of_size(3).iter().filter(|network| network.iter().any(|address| address[0..1].eq("t") )).count();

    println!("Part 1: {}", size);
}
