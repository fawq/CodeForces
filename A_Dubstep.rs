use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let dubstep: String = read_line().trim().to_string();
    let dubstep = dubstep.replace("WUB", " ").trim().to_string();
    let dubstep = dubstep.replace("  ", " ");

    println!("{}", dubstep);
}