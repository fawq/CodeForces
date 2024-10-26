use std::io::BufRead;
use std::collections::HashSet;

const PASS: &str = "I become the guy.";
const FAIL: &str = "Oh, my keyboard!";

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let levels: u8 = read_line().trim().parse().unwrap();
    let mut levels_passed: HashSet<u8> = HashSet::new();

    let x_levels: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let y_levels: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for index in 1..x_levels.len() {
        levels_passed.insert(x_levels[index]);
    }

    for index in 1..y_levels.len() {
        levels_passed.insert(y_levels[index]);
    }

    println!("{}", if levels_passed.len() == levels as usize { PASS } else { FAIL });
}