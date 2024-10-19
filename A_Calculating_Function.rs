use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number: i64 = read_line().trim().parse().unwrap();
    let sign: i64 = i64::pow(-1, (number % 2) as u32);
    let ans: i64 = (2 * sign * number + sign - 1) / 4;
    println!("{}", ans);
}