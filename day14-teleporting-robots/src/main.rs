use core::fmt;
use std::thread;
use std::time::Duration;

use aoc::Point;

struct Robot {
    position: Point,
    velocity: (i32, i32)
}

impl From<&str> for Robot {
    // Ugh... A regex would be so much better here
    fn from(s: &str) -> Self {
        let (position_data, velocity_data) = s.split_once(" ").unwrap();
        let (_, position) = position_data.split_once("=").unwrap();
        let (_, velocity) = velocity_data.split_once("=").unwrap();
        let (position_x, position_y) = position.split_once(",").unwrap();
        let (velocity_x, velocity_y) = velocity.split_once(",").unwrap();

        Robot {
            position: (position_x.parse().unwrap(), position_y.parse().unwrap()),
            velocity: (velocity_x.parse().unwrap(), velocity_y.parse().unwrap())
        }
    }
}

impl Robot {
    fn position_after(&self, seconds: i32, grid_size: (usize, usize)) -> Point {
        let new_x = (self.position.0 as i32 + self.velocity.0 * seconds).rem_euclid(grid_size.0 as i32);
        let new_y = (self.position.1 as i32 + self.velocity.1 * seconds).rem_euclid(grid_size.1 as i32);

        (new_x as usize, new_y as usize)
    }
}

struct Floor<'a> {
    size: (usize, usize),
    positions: &'a Vec<Point>
}

impl <'a> fmt::Display for Floor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if self.positions.contains(&(x, y)) {
                    write!(f, "x")?;
                } else {
                    write!(f, "-")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

const GRID_HEIGHT: usize = 101;
const GRID_WIDTH: usize = 103;

fn main() {
    let input = aoc::input();

    let grid_size = (GRID_HEIGHT, GRID_WIDTH);
    let robots: Vec<Robot> = input.lines().map(Robot::from).collect();
    let positions: Vec<Point> = robots.iter().map(|r| r.position_after(100, grid_size)).collect();

    let mut ranges: [usize; 4] = [0; 4];
    for &(x, y) in positions.iter() {
        if x < grid_size.0 / 2 && y < grid_size.1 / 2 {
            ranges[0] += 1;
        } else if x > grid_size.0 / 2 && y < grid_size.1 / 2 {
            ranges[1] += 1;
        } else if x < grid_size.0 / 2 && y > grid_size.1 / 2 {
            ranges[2] += 1;
        } else if x > grid_size.0 / 2 && y > grid_size.1 / 2 {
            ranges[3] += 1;
        }
    }

    let safety_factor: usize = ranges.iter().product();

    println!("Part 1: {}", safety_factor);

    // Let's find a christmas three
    let mut steps = 0;
    loop {
        let positions: Vec<Point> = robots.iter().map(|r| r.position_after(steps, grid_size)).collect();

        // Count the maximum number of robots on 1 line
        let &maximum_on_line = positions.iter().fold([0; GRID_HEIGHT], |mut acc, &(x, y)| {
            acc[x] += 1;
            acc
        }).iter().max().unwrap();

        if maximum_on_line >= 34 {
            let floor = Floor { positions: &positions, size: grid_size };
            println!("{}", floor);
            println!("Step: {}", steps);
            print!("\x1b[2J\x1b[H");
            thread::sleep(Duration::from_millis(1000));
        }

        steps += 1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_from() {
        let robot = Robot::from("p=7,6 v=-1,-3");
        assert_eq!(robot.position, (7, 6));
        assert_eq!(robot.velocity, (-1, -3));
    }

    #[test]
    fn test_robot_position_after() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3)
        };

        let example_grid = (11, 7);
        assert_eq!(robot.position_after(1, example_grid), (4, 1));
        assert_eq!(robot.position_after(2, example_grid), (6, 5));
        assert_eq!(robot.position_after(3, example_grid), (8, 2));
        assert_eq!(robot.position_after(4, example_grid), (10, 6));
        assert_eq!(robot.position_after(5, example_grid), (1, 3));
    }
}
