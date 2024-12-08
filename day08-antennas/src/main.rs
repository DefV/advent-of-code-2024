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
    bounds: (usize, usize),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut antennas = HashMap::new();

        let points: Vec<Vec<Point>> = input
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

        let bounds = (points.len(), points[0].len());

        Map { points, antennas, bounds }
    }
}

impl Map {
    fn point_at(&self, x: usize, y: usize) -> Option<&Point> {
        self.points.get(x)?.get(y)
    }

    fn combinations(&self, antennas: &[Location]) -> Vec<(Location, Location)> {
        antennas.iter().flat_map(|antenna| {
            antennas
                .iter()
                .filter(move |&other| other != antenna)
                .map(|other| (*antenna, *other))
        }).collect()
    }

    fn antinodes_for(&self, antennas: &[Location]) -> Vec<Location>{
        self.combinations(antennas).iter().filter_map(|((x1, y1), (x2, y2))| {
            Some((
                ((*x1 * 2) as isize - *x2 as isize).try_into().ok()?,
                ((*y1 * 2) as isize - *y2 as isize).try_into().ok()?
            ))
        }).collect()
    }

    fn repeated_antinodes_for(&self, antennas: &[Location]) -> Vec<Location> {
        let mut antinodes = vec![];
        for (antenna, other) in self.combinations(antennas) {
            let mut n = 0;
            loop {
                let x = antenna.0 as isize + (antenna.0 as isize - other.0 as isize) * n;
                let y = antenna.1 as isize + (antenna.1 as isize - other.1 as isize) * n;

                if x < 0 || y < 0 || x >= self.bounds.0 as isize || y >= self.bounds.1 as isize {
                    break;
                }

                antinodes.push((x as usize, y as usize));
                n += 1;
            }
        }

        antinodes
    }

    fn antinodes(&self, repeatable: bool) -> Vec<Location> {
        self.antennas.values().flat_map(|antennas| {
            if repeatable {
                self.repeated_antinodes_for(antennas)
            } else {
                self.antinodes_for(antennas)
            }
        }).collect()
    }

    fn placeable_antinodes(&self, repeatable: bool) -> HashSet<Location> {
        self.antinodes(repeatable).iter().filter(|(x, y)| {
            self.point_at(*x, *y).is_some()
        }).cloned().collect()
    }
}

fn main() {
    let input = aoc::input();

    let map = Map::from(input.as_str());
    let part1_result = map.placeable_antinodes(false).len();
    println!("Part 1: {}", part1_result);

    let part2_result = map.placeable_antinodes(true).len();
    println!("Part 2: {}", part2_result);
}
