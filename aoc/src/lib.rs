pub fn input() -> String {
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];
    std::fs::read_to_string(filename).expect("Something went wrong reading the file")
}
