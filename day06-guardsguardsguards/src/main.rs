use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq,Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Map {
    // Data holds a map where true is an obstruction and false is open space
    data: Vec<Vec<bool>>,
    position: (usize, usize, Direction),
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut position = (0, 0, Direction::North);
        let data = value
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        '^' => {
                            position = (x, y, Direction::North);
                            false
                        }
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid character in map"),
                    })
                    .collect()
            })
            .collect();

        Self { data, position }
    }
}

impl Map {
    fn turn_right(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }

    }
    fn step(&mut self) -> bool {
        let (x, y, direction) = self.position;
        let (next_x, next_y) = match direction {
            Direction::North => (x as isize - 1, y as isize),
            Direction::East => (x as isize, y as isize + 1),
            Direction::South => (x as isize + 1, y as isize),
            Direction::West => (x as isize, y as isize - 1),
        };

        if let Some(obstructed) = self.at(next_x, next_y) {
            if *obstructed {
                self.position = (x, y, Self::turn_right(&direction));
            } else {
                self.position = (next_x as usize, next_y as usize, direction);
            }

            true
        } else {
            false
        }
    }

    fn is_loop(&mut self) -> bool {
        let mut visited_positions: HashSet<(usize, usize, Direction)> = HashSet::new();
        visited_positions.insert(self.position);

        while self.step() {
            if !visited_positions.insert(self.position) {
                return true;
            }
        }

        false
    }

    fn at(&self, x: isize, y: isize) -> Option<&bool> {
        self.data.get(x as usize)?.get(y as usize)
    }
}

fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut map = Map::from(input);

    visited_positions.insert((map.position.0, map.position.1));
    while map.step() {
        visited_positions.insert((map.position.0, map.position.1));
    }

    visited_positions.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut map = Map::from(input);
    let start = map.position.clone();

    while map.step() {
        visited_positions.insert((map.position.0, map.position.1));
    }

    // Let's see if we get a loop by putting something in the way on every visited position
    visited_positions.into_iter()
        .filter_map(|(x, y)| {
            let mut data = map.data.clone();
            data[x][y] = true;
            let mut map = Map { data, position: start };
            if map.is_loop() {
                Some((x, y))
            } else {
                None
            }
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...\
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
