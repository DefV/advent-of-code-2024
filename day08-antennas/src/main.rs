use std::collections::{HashMap,HashSet};

type Location = (usize, usize);

#[derive(PartialEq)]
enum Point {
    Empty,
    Antenna(char),
}

struct Map {
    points: Vec<Vec<Point>>,
    antennas: HashMap<char, Vec<Location>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut antennas = HashMap::new();

        let points = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        if c == '.' {
                            Point::Empty
                        } else {
                            antennas.entry(c).or_insert(vec![]).push((x, y));

                            Point::Antenna(c)
                        }
                    })
                    .collect()
            })
            .collect();

        Map { points, antennas }
    }
}

impl Map {
    fn point_at(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.get(x)?.get(y)
    }

    fn antinodes_for(&self, antennas: &[Location]) -> Vec<Location>{
        let combinations: Vec<(Location, Location)> = antennas.iter().flat_map(|antenna| {
            antennas
                .iter()
                .filter(move |&other| other != antenna)
                .map(|other| (*antenna, *other))
        }).collect();

        // The combination of antennas loops every pair twice, in each direction
        // So we only need to add one direction
        combinations.iter().filter_map(|((x1, y1), (x2, y2))| {
            Some((
                ((*x1 * 2) as isize - *x2 as isize).try_into().ok()?,
                ((*y1 * 2) as isize - *y2 as isize).try_into().ok()?
            ))
        }).collect()
    }

    fn antinodes(&self) -> Vec<Location> {
        self.antennas.values().flat_map(|antennas| {
            self.antinodes_for(antennas)
        }).collect()
    }

    fn placeable_antinodes(&self) -> HashSet<Location> {
        self.antinodes().iter().filter(|(x, y)| {
            self.point_at(*x, *y).is_some()
        }).cloned().collect()
    }
}

fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    map.placeable_antinodes().len()
}
