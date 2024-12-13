use regex::Regex;

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn solve_machine(input: &str) -> Option<u32> {
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let captures = regex.captures(input).unwrap();

    let numbers: Vec<f32> = captures
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().parse::<f32>().unwrap())
        .collect();

    let [x1, y1, x2, y2, r1, r2] = numbers[..] else {
        panic!("Invalid input")
    };

    let times_a = ((r1*y2) - (r2*x2)) / ((x1*y2) - (y1*x2));
    let times_b = ((r1*y1) - (r2*x1)) / ((x2*y1) - (y2*x1));

    if (times_a.round() == times_a) && (times_b.round() == times_b) {
        Some((times_a * 3f32 + times_b * 1f32) as u32)
    } else {
        None
    }
}

fn main() {
    let input = aoc::input();
    let result: u32 = input.split("\n\n").filter_map(|machine| solve_machine(machine)).sum();

    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_machine() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let impossible_input = "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176";
        let possible_input2 = "Button A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450";

        assert_eq!(solve_machine(input), Some(280));
        assert_eq!(solve_machine(impossible_input), None);
        assert_eq!(solve_machine(possible_input2), Some(200));
    }
}
