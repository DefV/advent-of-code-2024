pub type Point = (usize, usize);

pub struct Map<T> {
    pub data: Vec<Vec<T>>
}

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

impl From<&str> for Map<char> {
    fn from(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect() ).collect();

        Map { data }
    }
}

impl<T> Map<T> {
    pub fn at_point(&self, point: Point) -> Option<&T> {
        let (x, y) = point;
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.at_point((x, y))
    }

    pub fn neighbours(&self, point: Point) -> [(Point, Option<&T>);4] {
        let (x, y) = point;
        DIRECTIONS.map(|(dx, dy)| {
            let np = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            (np, self.at_point(np))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.data.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, item)| ((x, y), item))
        })
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
