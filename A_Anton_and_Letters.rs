use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let set_of_letters: String = read_line().trim().to_string();
    let set_of_letters = set_of_letters.replace(&['{', '}', ','], "");

    let letters: HashSet<&str> = set_of_letters
        .split_ascii_whitespace()
        .collect();

    println!("{}", letters.len());
}