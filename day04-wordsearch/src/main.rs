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

        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some(start))
            .flat_map(|(x, y)| self.words_from(x, y, word.len()))
            .filter(|found_word| found_word == word)
            .count() as u32
    }

    // Just hardcode we're looking for the MAS cross
    fn cross_count(&self) -> u32 {
        self.positions()
            .filter(|&(x, y)| self.char_at(x, y) == Some('A'))
            .filter(|&(x, y)| {
                let lt = self.char_at(x - 1, y - 1);
                let rt = self.char_at(x + 1, y - 1);
                let lb = self.char_at(x - 1, y + 1);
                let rb = self.char_at(x + 1, y + 1);

                matches!(
                    (lt, rb, lb, rt),
                    (Some('M'), Some('S'), Some('M'), Some('S')) |
                    (Some('S'), Some('M'), Some('S'), Some('M')) |
                    (Some('M'), Some('S'), Some('S'), Some('M')) |
                    (Some('S'), Some('M'), Some('M'), Some('S'))
                )
            })
            .count() as u32
    }

    fn words_from(&self, x: isize, y: isize, len: usize) -> impl Iterator<Item = String> + '_ {
        DIRECTIONS.iter().filter_map(move |&(dx, dy)| {
            (0..len as isize)
                .map(|n| self.char_at(x + dx * n, y + dy * n))
                .collect()
        })
    }

    fn char_at(&self, x: isize, y: isize) -> Option<char> {
        self.grid.get(x as usize)?.get(y as usize).copied()
    }

    fn positions(&self) -> impl Iterator<Item = (isize, isize)> {
        let rows = self.grid.len() as isize;
        let cols = self.grid.first().map_or(0, |row| row.len()) as isize;
        (0..rows).flat_map(move |x| (0..cols).map(move |y| (x, y)))
    }
}

fn main() {
    let input = aoc::input();

    println!("Step 1 result: {}", step1(&input));
    println!("Step 2 result: {}", step2(&input));
}

fn step1(input: &str) -> u32 {
    Puzzle::from(input).word_count("XMAS")
}

fn step2(input: &str) -> u32 {
    Puzzle::from(input).cross_count()
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

    #[test]
    fn step2_example() {
        assert_eq!(super::step2(EXAMPLE), 9);
    }
}
