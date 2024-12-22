// Let's start with the naive approach
struct Trader {
    secret: u64
}

impl From<&str> for Trader {
    fn from(input: &str) -> Self {
        let secret = input.parse().unwrap();
        Trader { secret }
    }
}

impl Trader {
    fn regen_secret(&self, times: usize) -> u64 {
        let mut secret = self.secret;

        for _ in 0..times {
            secret = ((secret << 6) ^ secret) % (2 << 23);
            secret = ((secret >> 5) ^ secret) % (2 << 23);
            secret = ((secret << 11) ^ secret) % (2 << 23);
        }

        secret
    }
}

fn main() {
    let input = aoc::input();
    let result: u64 = input.trim().lines().map(Trader::from).map(|trader| {
        trader.regen_secret(2000)
    }).sum();

    println!("Part 1: {}", result);
}
