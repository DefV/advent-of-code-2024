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

fn main() {
    let input = aoc::input();

    // let grid_size = (11, 7);
    let grid_size = (101, 103);
    let positions: Vec<Point> = input.lines().map(Robot::from).map(|r| r.position_after(100, grid_size)).collect();

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
