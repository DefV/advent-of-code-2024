use itertools::Itertools;

type Point = (usize, usize);
struct Trails {
    grid: Vec<Vec<usize>>,
    trail_starts: Vec<Point>,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl From<&str> for Trails {
    fn from(input: &str) -> Self {
        let mut trail_starts = vec![];
        let grid = input.lines().enumerate().map(|(x, line)| {
            line.chars().enumerate().map(|(y, c)| {
                if c == '0' {
                    trail_starts.push((x, y));
                }
                c.to_digit(10).unwrap() as usize
            }).collect()
        }).collect();

        Self { grid, trail_starts }
    }
}

impl Trails {
    fn moved(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(&usize, Point)> {
        let next_pos = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
        let next = self.grid.get(next_pos.0)?.get(next_pos.1)?;

        Some((next, next_pos))
    }

    fn reachable_ends(&self, trail_start: &Point) -> Vec<Point> {
        let (x, y) = *trail_start;
        let current = self.grid[x][y];

        DIRECTIONS.iter().flat_map(|&(dx, dy)| {
            self.moved(x, y, dx, dy).and_then(|(&next, next_pos)| {
                if next == current + 1 {
                    if next == 9 {
                        Some(vec![next_pos])
                    } else {
                        Some(self.reachable_ends(&next_pos))
                    }
                } else {
                    None
                }
          }).unwrap_or_default()
        }).collect()
    }

    fn trail_score(&self, trail_start: &Point) -> usize {
        self.reachable_ends(trail_start).into_iter().unique().count()
    }

    fn score(&self) -> usize {
        self.trail_starts.iter().map(|start| self.trail_score(start)).sum()
    }

    fn rating(&self) -> usize {
        self.trail_starts.iter().map(|start| self.reachable_ends(start).len() ).sum()
    }
}

fn main() {
    let input = aoc::input();
    let trails = Trails::from(input.as_str());

    println!("Part 1: {}", trails.score());
    println!("Part 2: {}", trails.rating());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732\
";
    #[test]
    fn test_trails_from() {
        let trails = Trails::from(EXAMPLE);

        assert_eq!(9, trails.trail_starts.len());
        let trail_start = trails.trail_starts[0];
        assert_eq!(0, trails.grid[trail_start.0][trail_start.1]);
    }

    #[test]
    fn test_trail_score() {
        let trails = Trails::from(EXAMPLE);

        let trail_start = trails.trail_starts[0];
        assert_eq!(5, trails.trail_score(&trail_start));
    }

    #[test]
    fn test_score() {
        let trails = Trails::from(EXAMPLE);

        assert_eq!(36, trails.score());
    }

    #[test]
    fn test_rating() {
        let trails = Trails::from(EXAMPLE);

        assert_eq!(81, trails.rating());
    }
}
