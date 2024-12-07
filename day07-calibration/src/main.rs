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

impl Calibration {
    fn try_all_operations(total: u64, numbers: Vec<u64>) -> Vec<u64> {
        if let Some(number) = numbers.first() {
            let mut results = vec![];
            let rest = numbers[1..].to_vec();

            results.extend(Self::try_all_operations(total + number, rest.clone()));
            results.extend(Self::try_all_operations(total * number, rest.clone()));
            results
        } else {
            vec![total]
        }
    }

    fn is_solvable(&self) -> bool {
        Self::try_all_operations(self.numbers[0], self.numbers[1..].to_vec()).contains(&self.result)
    }
}

fn main() {
    let input = aoc::input();

    let total: u64 = input
        .lines()
        .map(Calibration::from)
        .filter(Calibration::is_solvable)
        .map(|calibration| calibration.result)
        .sum();

    println!("Total: {}", total);
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
