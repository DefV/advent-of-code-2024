struct Report {
    levels: Vec<i32>,
}

impl From<&str> for Report {
    fn from(input: &str) -> Self {
        let levels = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Report { levels }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let differences: Vec<i32> = self.levels
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        differences.iter().all(|&x| (x > 0 && x < 4)) ||
            differences.iter().all(|&x| (x < 0 && x > -4))
    }
}

fn main() {
    let document = aoc::input();

    println!("Step 1: {}", step1(&document));
}

fn step1(input: &str) -> String {
    input.lines()
         .filter(|line| Report::from(*line).is_safe())
         .count()
         .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_input() {
        assert_eq!(step1(EXAMPLE), "2");
    }


    #[test]
    fn test_is_safe() {
        assert!(Report::from("1 2 3 4 5").is_safe());
        assert!(Report::from("7 6 4 2 1").is_safe());
        assert!(!Report::from("1 2 7 8 9").is_safe());
        assert!(!Report::from("1 3 2 4 5").is_safe());
    }
}
