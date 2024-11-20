use std::io::BufRead;
use std::cmp::max;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let n: usize = read_line().trim().parse().unwrap();
    let moneys: Vec<u32> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut max_non_decreasing_subsequence: u32 = 1;

    let mut non_decreasing_subsequence: u32 = 1;
    for index in 1..n {
        if moneys[index] >= moneys[index - 1] {
            non_decreasing_subsequence += 1;
            max_non_decreasing_subsequence = max(max_non_decreasing_subsequence, non_decreasing_subsequence);
        } else {
            non_decreasing_subsequence = 1;
        }
    }
    println!("{}", max_non_decreasing_subsequence);
}