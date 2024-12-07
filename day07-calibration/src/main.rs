use std::time::Instant;
use rayon::prelude::*;

struct Calibration {
    result: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Calibration {
    fn from(input: &str) -> Self {
        let (result, numbers) = input
            .split_once(": ")
            .and_then(|(result, numbers)| {
                Some((
                    result.parse().unwrap(),
                    numbers.split(" ").map(|n| n.parse().unwrap()).collect(),
                ))
            })
            .expect("Error parsing calibration input");

        Self { result, numbers }
    }
}

fn mul(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(b)
}

fn add(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

fn combine(a: u64, b: u64) -> Option<u64> {
    let mut digits = a.to_string();
    digits.push_str(&b.to_string());
    digits.parse().ok()
}

impl Calibration {
    fn try_all_operations(total: u64, max: &u64, numbers: &[u64]) -> Vec<u64> {
        if *max < total {
            return vec![];
        }
        if let Some((number, rest)) = numbers.split_first() {
            let mut results = vec![];

            for operation in [mul, add, combine] {
                if let Some(result) = operation(total, *number) {
                    results.extend(Self::try_all_operations(result, max,rest));
                }
            };

            results
        } else {
            vec![total]
        }
    }

    fn is_solvable(&self) -> bool {
        Self::try_all_operations(self.numbers[0], &self.result,&self.numbers[1..]).contains(&self.result)
    }
}

fn main() {
    let start = Instant::now();
    let input = aoc::input();

    let total: u64 = input
        .par_lines()
        .map(Calibration::from)
        .filter(Calibration::is_solvable)
        .map(|calibration| calibration.result)
        .sum();

    println!("Total: {} in {:?}", total, start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_is_solvable() {
        assert_eq!(Calibration::from("190: 10 19").is_solvable(), true);
        assert_eq!(Calibration::from("3267: 81 40 27").is_solvable(), true);
        assert_eq!(Calibration::from("161011: 16 10 13").is_solvable(), false);
    }
}
