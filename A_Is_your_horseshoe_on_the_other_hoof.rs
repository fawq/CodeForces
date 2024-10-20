use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let horsehoes: HashSet<u32> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", 4 - horsehoes.len());
}