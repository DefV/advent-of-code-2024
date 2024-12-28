use std::collections::{HashMap,HashSet};

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
            let left: Address = left.into();
            let right: Address = right.into();

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

    fn bron_kerbosch(&self, r: Vec<Address>, p: Vec<Address>, x: Vec<Address>, cliques: &mut Vec<Vec<Address>>) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r);
            return;
        }

        let mut p = p.clone();
        let mut x = x.clone();

        while let Some(v) = p.pop() {
            let mut r = r.clone();
            r.push(v.clone());

            let neighbors = self.connections.get(&v).unwrap();
            let p = p.iter().filter(|address| neighbors.contains(address)).cloned().collect();
            let next_x: Vec<Address> = x.iter().filter(|address| neighbors.contains(address)).cloned().collect();

            self.bron_kerbosch(r, p, next_x, cliques);

            x.push(v);
        }
    }

    fn max_cliques(&self) -> Vec<Address> {
        let mut cliques: Vec<Vec<Address>> = Vec::new();
        self.bron_kerbosch(vec![], self.connections.keys().cloned().collect(), vec![], &mut cliques);

        let longest = cliques.iter().max_by_key(|clique| clique.len()).unwrap();
        longest.clone()
    }
}

fn main() {
    let input = aoc::input();
    let puzzle = Puzzle::from(input.as_str());

    let size = puzzle.networks_of_size(3).iter().filter(|network| network.iter().any(|address| address[0..1].eq("t") )).count();

    println!("Part 1: {}", size);

    let mut longest = puzzle.max_cliques();
    longest.sort();

    println!("Part 2: {}", longest.join(","));
}
