use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let (n, k): (u64, u64) = {
        let line = read_line();
        let mut parts = line.split_ascii_whitespace();
        (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
    };

    let index_of_change: u64 = if n % 2 == 0 { n / 2 } else { (n + 1) /2 };

    if k <= index_of_change {
        println!("{}", 2 * k - 1);
    } else {
        println!("{}", 2 * (k - index_of_change));
    }
}