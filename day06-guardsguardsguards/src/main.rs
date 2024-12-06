use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let (x, y, _) = self.position;
        let (next_x, next_y) = match self.position.2 {
            Direction::North => (x - 1, y),
            Direction::East => (x, y + 1),
            Direction::South => (x + 1, y),
            Direction::West => (x, y - 1),
        };

        if let Some(obstructed) = self.at(next_x, next_y) {
            if *obstructed {
                self.position = (x, y, Self::turn_right(&self.position.2));
            } else {
                self.position = (next_x, next_y, self.position.2);
            }

            true
        } else {
            false
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<&bool> {
        self.data.get(x)?.get(y)
    }
}

fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
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
}
