use std::io::BufRead;

const DENOMINATORS: [u8; 5] = [100, 20, 10, 5, 1];

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let mut money: u32 = read_line().trim().parse().unwrap();
    let mut total: u32 = 0;

    for denominator in DENOMINATORS {
        total += money / denominator as u32;
        money %= denominator as u32;
    }

    println!("{}", total);
}