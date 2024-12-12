use aoc::{Map, Point};
use std::{
    collections::{HashSet, VecDeque},
    vec,
};

#[derive(Debug)]
struct Plot {
    letter: char,
    points: Vec<Point>,
    area: u32,
    perimeter: u32,
    corners: u32,
}

struct Input {
    map: Map<char>,
    plots: Vec<Plot>,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let map = Map::from(input);

        Self {
            map,
            plots: Vec::new(),
        }
    }
}

impl Input {
    fn find_plots(&mut self) {
        let mut seen: HashSet<Point> = HashSet::new();
        for (point, letter) in self.map.iter() {
            if seen.contains(&point) {
                continue;
            }

            self.plots.push(self.explore_plot(point, letter, &mut seen));
        }
    }

    fn explore_plot(&self, start: Point, letter: &char, seen: &mut HashSet<Point>) -> Plot {
        let mut points = vec![];
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;
        let mut to_explore: VecDeque<Point> = VecDeque::from(vec![start]);

        while let Some(point) = to_explore.pop_front() {
            if !seen.insert(point) {
                continue;
            }

            points.push(point);
            area += 1;

            let mut area_corners = 0;

            let neighbors = self.map.all_neighbours(point);
            for i in 0..4 {
                let (np, neighbor) = neighbors[i * 2];
                let (_, diagonal) = neighbors[(i * 2 + 1) % 8];
                let (_, next_neighbor) = neighbors[(i * 2 + 2) % 8];

                match neighbor {
                    Some(c) if c == letter => {
                        to_explore.push_front(np);

                        if let (Some(next_char), Some(diagonal_char)) = (next_neighbor, diagonal) {
                            if next_char == letter && diagonal_char != letter {
                                area_corners += 1
                            }
                        }
                    }
                    Some(_) => {
                        perimeter += 1;

                        // Check inner corner
                        if next_neighbor.is_none() || next_neighbor.unwrap() != letter {
                            area_corners += 1
                        }
                    }
                    None => {
                        perimeter += 1;
                        if next_neighbor.is_none() || next_neighbor.unwrap() != letter {
                            area_corners += 1;
                        }
                    }
                }
            }

            corners += area_corners;
        }

        Plot {
            letter: *letter,
            points,
            area,
            perimeter,
            corners,
        }
    }

    fn price(&self) -> u32 {
        self.plots.iter().map(|plot| plot.price()).sum()
    }

    fn discounted_price(&self) -> u32 {
        self.plots.iter().map(|plot| plot.discounted_price()).sum()
    }
}

impl Plot {
    fn price(&self) -> u32 {
        self.area * self.perimeter
    }

    fn discounted_price(&self) -> u32 {
        self.area * self.corners
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let mut plot = Input::from(input);
    plot.find_plots();

    println!("Part 1: {}", plot.price());
    println!("Part 2: {}", plot.discounted_price());
}
