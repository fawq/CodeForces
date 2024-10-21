use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    _ = read_line();
    let mut numbers: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    numbers.sort();

    let numbers_str: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    println!("{}", numbers_str.join(" "));
}