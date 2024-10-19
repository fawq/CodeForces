use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let rooms: u8 = read_line().trim().parse().unwrap();
    let mut free_rooms: u8 = 0;

    for _ in 0..rooms {
        let (p, q) = {
            let line = read_line();
            let mut parts = line.split_ascii_whitespace();

            let p: u8 = parts.next().unwrap().parse().unwrap();
            let q: u8 = parts.next().unwrap().parse().unwrap();

            (p, q)
        };

        if q - p >= 2 {
            free_rooms += 1;
        }
    }

    println!("{}", free_rooms);
}