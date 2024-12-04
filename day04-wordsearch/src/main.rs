type Grid = Vec<Vec<char>>;

struct Puzzle {
    grid: Grid,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let grid: Grid = input.lines().map(|l| l.chars().collect() ).collect();

        Self { grid }
    }
}

impl Puzzle {
    fn word_count(&self, word: &str) -> u32 {
        let start = word.chars().next().unwrap();

        (0..self.grid.len())
            .flat_map(|x| (0..self.grid[x].len()).map(move |y| (x, y)))
            .filter(|&(x, y)| self.char_at(x as isize, y as isize) == Some(start))
            .flat_map(|(x, y)| self.words_from(x as isize, y as isize, word.len()))
            .filter(|found_word| found_word == word)
            .count() as u32
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
        self.grid.get(x as usize)?.get(y as usize).copied()
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
