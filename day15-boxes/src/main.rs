use aoc::{Map, Point};

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
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
    fn step(&mut self, direction: Direction) {
        let (dx, dy) = direction.to_movement();

        let mut next_pos = aoc::move_point_by(self.robot, dx, dy);

        match self.map.at_point(next_pos) {
            Some(Tile::Empty) => {
                self.map.set_point(self.robot, Tile::Empty);
                self.map.set_point(next_pos, Tile::Robot);
                self.robot = next_pos;
            }
            Some(Tile::Box) => {
                let robot_pos = next_pos;
                loop {
                    next_pos = aoc::move_point_by(next_pos, dx, dy);

                    match self.map.at_point(next_pos) {
                        Some(Tile::Empty) => {
                            self.map.set_point(self.robot, Tile::Empty);
                            self.map.set_point(robot_pos, Tile::Robot);
                            self.map.set_point(next_pos, Tile::Box);
                            self.robot = robot_pos;
                            break;
                        }
                        Some(Tile::Wall) | None => break,
                        _ => {}
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
            .filter(|(_, tile)| matches!(tile, Tile::Box))
            .map(|(pos, _)| pos.0 * 100 + pos.1)
            .sum()
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let mut puzzle = Puzzle::from(input);
    puzzle.resolve();
    println!("Part 1: {}", puzzle.score());
}
