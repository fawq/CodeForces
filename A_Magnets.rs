use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number_of_magnets: u32 = read_line().trim().parse().unwrap();
    let mut number_of_groups: u32 = 0;
    let mut current_magnet: String = "".to_string();

    for _ in 0..number_of_magnets {
        let magnet = read_line().trim().to_string();
        if magnet != current_magnet {
            current_magnet = magnet;
            number_of_groups += 1;
        }
    }

    println!("{}", number_of_groups);
}