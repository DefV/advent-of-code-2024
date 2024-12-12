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
        let mut to_explore: VecDeque<Point> = VecDeque::from(vec![start]);

        while let Some(point) = to_explore.pop_front() {
            if !seen.insert(point) {
                continue;
            }

            points.push(point);
            area += 1;

            for (np, neighbor) in self.map.neighbours(point) {
                match neighbor {
                    Some(c) if c == letter => {
                        if seen.contains(&np) {
                            continue;
                        }

                        to_explore.push_back(np);
                    }
                    Some(_) => perimeter += 1,
                    None => perimeter += 1,
                }
            }
        }

        Plot {
            letter: *letter,
            points,
            area,
            perimeter,
        }
    }

    fn price(&self) -> u32 {
        self.plots.iter().map(|plot| plot.price()).sum()
    }
}

impl Plot {
    fn price(&self) -> u32 {
        self.area * self.perimeter
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let mut plot = Input::from(input);
    plot.find_plots();

    println!("Part 1: {}", plot.price());
}
