use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number_of_drinks: u8 = read_line().trim().parse().unwrap();
    let sum_of_percentages: u16 = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u8>().unwrap())
        .fold(0, |acc, x| acc + x as u16);

    // It will not cover the same output but should be in bound
    println!("{}", sum_of_percentages as f64 / number_of_drinks as f64);
}