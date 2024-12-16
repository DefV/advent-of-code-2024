use aoc::Map;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Corridor,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn add_delta(&self, x: usize, y: usize) -> (usize, usize) {
        let (dx, dy) = self.to_delta();
        ((x as isize + dx) as usize, (y as isize + dy) as usize)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Corridor,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Wall => '#',
            Tile::Corridor => '.',
            Tile::Start => 'S',
            Tile::End => 'E',
        };
        write!(f, "{}", c)
    }
}

struct Maze {
    map: Map<Tile>,
    position: (usize, usize, Direction),
    end: (usize, usize),
}

impl From<&str> for Maze {
    fn from(s: &str) -> Self {
        let map = Map::from(s);

        let position = (map.height - 2, 1, Direction::Right);
        let end = (1, map.height - 2);

        Maze { map, position, end }
    }
}

impl Maze {
    fn cheapest_path(&self) -> u32 {
        let mut weights: HashMap<(usize, usize, Direction), (u32, Vec<(usize, usize, Direction)>, Vec<Vec<(usize, usize)>>)> = HashMap::new();
        let mut queue = vec![(self.position, 0u32)];

        while let Some(((x, y, direction), weight)) = queue.pop() {
            let splits = self.reachable_splits(x, y, direction);
            for (new_x, new_y, new_dir, new_weight, path) in splits {
                if let Some((existing_weight, sources, existing_paths)) = weights.get_mut(&(new_x, new_y, new_dir)) {
                    if weight + new_weight > *existing_weight {
                        continue;
                    } else if weight + new_weight == *existing_weight {
                        sources.push((x, y, direction));
                        existing_paths.push(path);
                        continue;
                    }
                }

                weights.insert((new_x, new_y, new_dir), (weight + new_weight, vec![(x, y, direction)], vec![path]));
                queue.push(((new_x, new_y, new_dir), weight + new_weight));
            }
        }

        let mut end_routes: Vec<(u32, Vec<(usize, usize, Direction)>, Vec<Vec<(usize, usize)>>)> = weights.iter().filter_map(|((x, y, dir), (weight, sources, paths))| {
            if (*x, *y) == self.end {
                Some((*weight, sources.clone(), paths.clone()))
            } else {
                None
            }
        }).collect();
        end_routes.sort_by_key(|(weight, _, _)| *weight);
        let mut visited_points: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut visited_sources: std::collections::HashSet<(usize, usize, Direction)> = std::collections::HashSet::new();
        let (weight, sources, paths) = end_routes.first().unwrap();
        let mut sources_to_follow = sources.clone();
        paths.iter().for_each(|path| {
            path.iter().for_each(|point| {
                visited_points.insert(*point);
            });
        });

        while let Some(source) = sources_to_follow.pop()  {
            if !visited_sources.insert(source) {
                continue;
            }
            if let Some((_, sources, paths)) = weights.get(&source) {
                sources_to_follow.extend(sources.clone());
                paths.iter().for_each(|path| {
                    path.iter().for_each(|point| {
                        visited_points.insert(*point);
                    });
                });
            }
        };

        dbg!(visited_points.len() + 1);
        *weight
    }

    fn reachable_splits(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Vec<(usize, usize, Direction, u32, Vec<(usize, usize)>)> {
        let mut splits: Vec<(usize, usize, Direction, u32, Vec<(usize, usize)>)> = Vec::new();

        for (point, dir) in self.corridors_at(x, y, direction) {
            let mut weight = if direction == dir { 1 } else { 1001 };
            let mut point = point;
            let mut dir = dir;
            let mut visited = vec![(x, y)];

            loop {
                visited.push(point);
                let next = self.corridors_at(point.0, point.1, dir);
                if next.len() == 1 {
                    weight += if next[0].1 == dir { 1 } else { 1001 };
                    point = next[0].0;
                    dir = next[0].1;

                    if self.map.at_point(point) == Some(&Tile::End) {
                        splits.push((point.0, point.1, dir, weight, visited));
                        break;
                    }
                } else if next.len() == 0 {
                    break;
                } else {
                    for (_, ndir) in next {
                        splits.push((point.0, point.1, ndir, if ndir != dir { weight + 1000 } else { weight }, visited.clone()));
                    }
                    break;
                }
            }
        }

        splits
    }

    fn corridors_at(&self, x: usize, y: usize, direction: Direction) -> Vec<((usize, usize), Direction)> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ].into_iter().filter_map(|dir| {
            // Don't go where we came from
            if direction.is_opposite(dir) {
                return None
            }

            let pos = dir.add_delta(x, y);
            let point = self.map.at_point(pos);
            if matches!(point, Some(Tile::Corridor|Tile::End)) {
                Some((pos, dir))
            } else {
                None
            }
        }).collect()
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let maze = Maze::from(input);
    println!("Part 1: {}", maze.cheapest_path());
}
