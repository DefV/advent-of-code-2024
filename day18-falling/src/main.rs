use std::{collections::BinaryHeap, fmt::Display};

use aoc::{Map, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    position: Point,
    cost_so_far: u32,
    estimated_total_cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.estimated_total_cost.cmp(&self.estimated_total_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct MemorySpace {
    incoming_bytes: Vec<Point>,
    map: Map<Tile>,
}

impl From<&str> for MemorySpace {
    fn from(input: &str) -> Self {
        let incoming_bytes = input
            .lines()
            .filter_map(|line| {
                let mut nums = line.split(",").filter_map(|n| n.parse().ok());

                // Our map implementation uses (y, x) coordinates
                let y = nums.next()?;
                let x = nums.next()?;

                Some((x, y))
            })
            .collect();

        Self {
            incoming_bytes,
            ..Default::default()
        }
    }
}

impl Default for MemorySpace {
    fn default() -> Self {
        Self {
            incoming_bytes: Vec::new(),
            map: Map::default(),
        }
    }
}

fn manhattan_distance(a: Point, b: Point) -> u32 {
    let (x1, y1) = a;
    let (x2, y2) = b;

    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u32
}

impl MemorySpace {
    fn shortest_path(&self, end_point: Point) -> Option<u32> {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited = vec![vec![false; MAP_SIZE]; MAP_SIZE];

        queue.push(Node {
            position: (0, 0),
            cost_so_far: 0,
            estimated_total_cost: manhattan_distance((0, 0), end_point),
        });

        while let Some(Node {
            position,
            cost_so_far,
            ..
        }) = queue.pop() {
            if position == end_point {
                return Some(cost_so_far)
            }

            if visited[position.0][position.1] {
                continue;
            } else {
                visited[position.0][position.1] = true;
            }

            for (neighbor, tile) in self.map.cardinal_neighbours(position) {
                if let Some(Tile::Empty) = tile {
                    let new_cost = cost_so_far + 1;
                    let estimated_total_cost = new_cost + manhattan_distance(neighbor, end_point);

                    queue.push(Node {
                        position: neighbor,
                        cost_so_far: new_cost,
                        estimated_total_cost,
                    });
                }
            }
        }

        None
    }
}

fn main() {
    let input = aoc::input();
    part1(input.trim());
    part2(input.trim());
}

const MAP_SIZE: usize = 71;
const BYTES: usize = 1024;
fn part1(input: &str) {
    let mut memory_space: MemorySpace = input.into();

    memory_space.map = Map {
        width: MAP_SIZE,
        height: MAP_SIZE,
        data: vec![vec![Tile::Empty; MAP_SIZE]; MAP_SIZE],
    };

    for &(x, y) in &memory_space.incoming_bytes[..BYTES] {
        memory_space.map.set(x as usize, y as usize, Tile::Wall);
    }

    println!("{}", memory_space.map);
    let shortest_path = memory_space.shortest_path((MAP_SIZE - 1, MAP_SIZE - 1));
    println!("Part 1: {:?}", shortest_path);
}

fn part2(input: &str) {
    let mut memory_space: MemorySpace = input.into();

    memory_space.map = Map {
        width: MAP_SIZE,
        height: MAP_SIZE,
        data: vec![vec![Tile::Empty; MAP_SIZE]; MAP_SIZE],
    };

    for &(x, y) in &memory_space.incoming_bytes[..BYTES + 1] {
        memory_space.map.set(x as usize, y as usize, Tile::Wall);
    }

    let mut idx = BYTES + 1;
    while let Some(_) = memory_space.shortest_path((MAP_SIZE - 1, MAP_SIZE - 1)) {
        let (x, y) = memory_space.incoming_bytes[idx + 1];
        memory_space.map.set(x as usize, y as usize, Tile::Wall);
        idx += 1;
    }
    println!("Part 2: {:?}", memory_space.incoming_bytes[idx]);
}
