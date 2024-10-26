use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let _ = read_line();
    println!("{}", if read_line().trim().to_lowercase().chars().collect::<HashSet<char>>().len() == 26 { "YES" } else { "NO" });
}