use std::fmt;
pub type Point = (usize, usize);

pub fn move_point_by(point: Point, dx: isize, dy: isize) -> Point {
    let (x, y) = point;

    ((x as isize + dx) as usize, (y as isize + dy) as usize)
}

#[derive(Debug)]
pub struct Map<T> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Top
    (0, 1),  // Right
    (1, 0),  // Bottom
    (0, -1), // Left
];

const ALL_DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // Top
    (-1, 1),  // Top Right
    (0, 1),   // Right
    (1, 1),   // Bottom Right
    (1, 0),   // Bottom
    (1, -1),  // Bottom Left
    (0, -1),  // Left
    (-1, -1), // Top Left
];

impl<T> From<&str> for Map<T>
where
    T: From<char>,
{
    fn from(input: &str) -> Self {
        let data: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(T::from).collect())
            .collect();

        let height = data.len();
        let width = data[0].len();

        Map { data, width, height }
    }
}

impl<T> fmt::Display for Map<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            for x in row.iter() {
                write!(f, "{}", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> Default for Map<T>
{
    fn default() -> Self {
        Map {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl<T> Map<T> {
    pub fn at_point(&self, point: Point) -> Option<&T> {
        let (x, y) = point;
        self.data.get(x).and_then(|row| row.get(y))
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.at_point((x, y))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[x][y] = value;
    }

    pub fn set_point(&mut self, point: Point, value: T) {
        let (x, y) = point;
        self.set(x, y, value);
    }

    pub fn cardinal_neighbours(&self, point: Point) -> [(Point, Option<&T>); 4] {
        let (x, y) = point;
        CARDINAL_DIRECTIONS.map(|(dx, dy)| {
            let np = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            (np, self.at_point(np))
        })
    }

    pub fn all_neighbours(&self, point: Point) -> [(Point, Option<&T>); 8] {
        let (x, y) = point;
        ALL_DIRECTIONS.map(|(dx, dy)| {
            let np = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            (np, self.at_point(np))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, item)| ((x, y), item)))
    }
}

pub fn input() -> String {
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];
    std::fs::read_to_string(filename).expect("Something went wrong reading the file")
}
