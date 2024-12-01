struct Lists {
    list1: Vec<i32>,
    list2: Vec<i32>,
}

impl From<&String> for Lists {
    fn from(s: &String) -> Self {
        let (list1, list2) = s
            .lines()
            .map(|line| {
                line.split_once("   ")
                    .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                    .unwrap()
            })
            .unzip();

        Lists { list1, list2 }
    }
}

impl Lists {
    fn sorted(&self) -> (Vec<i32>, Vec<i32>) {
        let mut list1 = self.list1.clone();
        let mut list2 = self.list2.clone();
        list1.sort();
        list2.sort();

        (list1, list2)
    }

    fn distance (&self) -> i32 {
        let (list1, list2) = self.sorted();

        list1.iter().enumerate().fold(0, |acc, (i, x)| {
            acc + (list2[i] - x).abs()
        })
    }

    fn similarity_score(&self) -> i32 {
        self.list1.iter().fold(0, |acc, x| {
            acc + x * self.list2.iter().filter(|y| x == *y ).count() as i32
        })
    }
}

fn main() {
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let document =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Part 1 result: {}", part1(&document));
    println!("Part 2 result: {}", part2(&document));
}

fn part1(document: &String) -> String {
    return Lists::from(document).distance().to_string();
}

fn part2(document: &String) -> String {
    return Lists::from(document).similarity_score().to_string();
}

#[cfg(test)]
mod tests {
    // Example data from AoC
const EXAMPLE: &str =  "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!("11", super::part1(&EXAMPLE.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!("31", super::part2(&EXAMPLE.to_string()));
    }
}
