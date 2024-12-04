type Grid = Vec<Vec<char>>;

struct Puzzle {
    grid: Grid,
    size: (isize, isize),
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let grid: Grid = input.lines().map(|l| l.chars().collect() ).collect();
        let size = (grid.len() as isize, grid[0].len() as isize);

        Self { grid, size }
    }
}

impl Puzzle {
    fn word_count(&self, word: &str) -> u32 {
        let mut count: u32 = 0;
        let start = word.chars().next().unwrap();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if self.char_at(x, y) == Some(start) {
                    for found_word in self.words_from(x, y, word.len()).iter() {
                        if found_word == word {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn words_from(&self, x: isize, y: isize, len: usize) -> Vec<String> {
        DIRECTIONS.iter().map(|(dx, dy)| {
            let mut letters = vec![];
            for n in 0..len as isize {
                let lx = x + dx * n;
                let ly = y + dy * n;

                if let Some(c) = self.char_at(lx, ly) {
                    letters.push(c);
                } else {
                    break;
                }
            }

            String::from_iter(letters.into_iter())
        }).collect()
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        if x >= 0 && x < self.size.0 &&
           y >= 0 && y < self.size.1 {
            Some(self.grid[x as usize][y as usize])
        } else {
            None
        }
    }
}

fn main() {
    let input = aoc::input();

    println!("Step 1 result: {}", step1(&input));
}

fn step1(input: &str) -> u32 {
    Puzzle::from(input).word_count("XMAS")
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn step1_example() {
        assert_eq!(super::step1(EXAMPLE), 18);
    }
}
