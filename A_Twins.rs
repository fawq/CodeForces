use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    _ = read_line();
    let mut coins: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    coins.sort_by(|a, b| b.cmp(a));
    let sum_of_all_coins: u16 = coins.iter().fold(0, |acc, x| acc + (*x as u16));
    let mut partial_sum: u16 = 0;

    for (index, coin) in coins.iter().enumerate() {
        partial_sum += (*coin as u16);
        if partial_sum > sum_of_all_coins / 2 {
            println!("{}", index + 1);
            break;
        }
    }
}