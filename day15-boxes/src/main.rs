use aoc::{move_point_by, Map, Point};

#[derive(Debug,Clone,Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
    LeftBigBox,
    RightBigBox,
    Robot,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '[' => Tile::LeftBigBox,
            ']' => Tile::RightBigBox,
            '@' => Tile::Robot,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::LeftBigBox => '[',
            Tile::RightBigBox => ']',
            Tile::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn to_movement(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    map: Map<Tile>,
    robot: Point,
    directions: Vec<Direction>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let (map_str, directions_str) = input.split_once("\n\n").unwrap();
        let map = Map::from(map_str);
        let directions = directions_str
            .chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect();

        let mut robot = (0, 0);
        map.iter()
            .find(|(_, tile)| matches!(tile, Tile::Robot))
            .map(|(point, _)| robot = point);

        Self {
            map,
            robot,
            directions,
        }
    }
}

impl Puzzle {
    fn expanded_clone(&self) -> Puzzle {
        let data = self
            .map
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|tile| match tile {
                        Tile::Empty => vec![Tile::Empty, Tile::Empty],
                        Tile::Wall => vec![Tile::Wall, Tile::Wall],
                        Tile::Box => vec![Tile::LeftBigBox, Tile::RightBigBox],
                        Tile::Robot => vec![Tile::Robot, Tile::Empty],
                        _ => panic!("Can't expand tile: {:?}", tile),
                    })
                    .collect()
            })
            .collect();
        let map = Map { data };

        let robot = map
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::Robot))
            .map(|(point, _)| point)
            .unwrap();

        Puzzle {
            map: map,
            robot,
            directions: self.directions.clone(),
        }
    }

    fn step(&mut self, direction: Direction) {
        let (dx, dy) = direction.to_movement();

        let mut next_pos = move_point_by(self.robot, dx, dy);
        let next_robot_pos = next_pos;
        match self.map.at_point(next_pos) {
            Some(Tile::Empty) => {
                self.map.set_point(self.robot, Tile::Empty);
                self.map.set_point(next_pos, Tile::Robot);
                self.robot = next_robot_pos;
            }
            Some(Tile::Box) => loop {
                next_pos = move_point_by(next_pos, dx, dy);

                match self.map.at_point(next_pos) {
                    Some(Tile::Empty) => {
                        self.map.set_point(self.robot, Tile::Empty);
                        self.map.set_point(next_robot_pos, Tile::Robot);
                        self.map.set_point(next_pos, Tile::Box);
                        self.robot = next_robot_pos;
                        break;
                    }
                    Some(Tile::Wall) | None => break,
                    _ => {}
                }
            },
            Some(tile @ Tile::LeftBigBox) | Some(tile @ Tile::RightBigBox) => {
                // Going horizontal is easy
                match direction {
                    Direction::Left | Direction::Right => {
                        let mut positions = vec![];
                        loop {
                            next_pos = aoc::move_point_by(next_pos, dx, dy);

                            match self.map.at_point(next_pos) {
                                Some(Tile::LeftBigBox) | Some(Tile::RightBigBox) => {
                                    positions.push(next_pos);
                                }
                                Some(Tile::Empty) => {
                                    self.map.set_point(self.robot, Tile::Empty);
                                    self.map.set_point(next_robot_pos, Tile::Robot);
                                    self.map.set_point(next_pos, if matches!(direction, Direction::Left) { Tile::LeftBigBox } else { Tile::RightBigBox });
                                    self.robot = next_robot_pos;
                                    for pos in positions {
                                        match self.map.at_point(pos) {
                                            Some(Tile::LeftBigBox) => {
                                                self.map.set_point(pos, Tile::RightBigBox);
                                            }
                                            Some(Tile::RightBigBox) => {
                                                self.map.set_point(pos, Tile::LeftBigBox);
                                            }
                                            _ => {}
                                        }
                                    }
                                    break;
                                }
                                Some(Tile::Wall) | None => break,
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        // Up/Down is a bit more complicated since we need to resolve both left & right
                        let mut y_to_check: Vec<usize> = vec![];
                        let mut boxes_to_move: Vec<Vec<usize>> = vec![];

                        if matches!(tile, Tile::RightBigBox) {
                            y_to_check.extend([next_robot_pos.1 - 1, next_robot_pos.1]);
                        } else {
                            y_to_check.extend([next_robot_pos.1, next_robot_pos.1 + 1]);
                        }

                        loop {
                            boxes_to_move.push(y_to_check.clone());

                            next_pos = aoc::move_point_by(next_pos, dx, dy);
                            let next_tiles: Vec<(Point, &Tile)> = y_to_check.iter()
                                .map(|&y| ((next_pos.0, y), self.map.at(next_pos.0, y).unwrap())).collect();

                            if next_tiles.iter().all(|(_, tile)| matches!(tile, Tile::Empty)) {
                                let mut changes: std::collections::HashMap<Point, Tile> = std::collections::HashMap::new();

                                boxes_to_move.iter().enumerate().for_each(|(i, y)| {
                                    y.iter().for_each(|&y| {
                                        // Should we go up or down?
                                        let direction: isize = if matches!(direction, Direction::Up) { -1 } else { 1 };
                                        let current_pos = ((next_robot_pos.0 as isize + (i as isize * direction)) as usize, y);
                                        let destination_pos = ((next_robot_pos.0 as isize + ((i + 1) as isize * direction)) as usize, y);
                                        let tile = self.map.at(current_pos.0, current_pos.1).unwrap();

                                        changes.insert(destination_pos, *tile);
                                        changes.entry(current_pos).or_insert(Tile::Empty);
                                    });
                                });

                                for (pos, tile) in changes.iter() {
                                    self.map.set_point(*pos, *tile);
                                }

                                self.map.set_point(self.robot, Tile::Empty);
                                self.map.set_point(next_robot_pos, Tile::Robot);
                                self.robot = next_robot_pos;
                                break;
                            } else if next_tiles.iter().any(|(_, tile)| matches!(tile, Tile::Wall)) {
                                break;
                            } else {
                                y_to_check.clear();
                                // Find out which boxes we need to check next
                                for (i, ((_, x), tile)) in next_tiles.iter().enumerate() {
                                    if matches!(tile, Tile::RightBigBox) && i == 0 {
                                        y_to_check.extend([*x - 1, *x]);
                                    } else if matches!(tile, Tile::LeftBigBox) && i == next_tiles.len() - 1 {
                                        y_to_check.extend([*x, *x + 1]);
                                    } else if matches!(tile, Tile::RightBigBox) || matches!(tile, Tile::LeftBigBox) {
                                        y_to_check.push(*x);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn resolve(&mut self) {
        for i in 0..self.directions.len() {
            self.step(self.directions[i]);
        }
    }

    fn score(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Box) || matches!(tile, Tile::LeftBigBox))
            .map(|(pos, _)| pos.0 * 100 + pos.1)
            .sum()
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let mut puzzle = Puzzle::from(input);
    let mut expanded_puzzle = puzzle.expanded_clone();

    puzzle.resolve();
    println!("Part 1: {}", puzzle.score());

    println!("{}", expanded_puzzle.map);
    expanded_puzzle.resolve();

    println!("{}", expanded_puzzle.map);
    println!("Part 2: {}", expanded_puzzle.score());
}
