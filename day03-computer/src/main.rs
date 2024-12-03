use regex::Regex;

fn main() {
    let input = aoc::input();

    println!("Step 1 result: {}", step1(&input));
    println!("Step 2 result: {}", step2(&input));
}

fn step1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
      .fold(0, |acc, m| {
        acc + mul(m)
      })
}

fn step2(input: &str) -> i32 {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut acc = 0;

    re.captures_iter(input)
      .for_each(|m| {
        if let Some(command) = m.get(0) {
            match command.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        acc += mul(m);
                    }
                }
            }
        }
      });

    acc
}

fn mul(captures: regex::Captures<'_>) -> i32 {
    captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_step1() {
        assert_eq!(step1(EXAMPLE), 161);
    }

    #[test]
    fn test_step2() {
        assert_eq!(step2(EXAMPLE2), 48);
    }
}
