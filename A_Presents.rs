use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let mut number_of_friends: u8 = read_line().trim().parse().unwrap();
    let mut presents_origin: Vec<u8> = vec![0; number_of_friends as usize];
    let mut presents_destination: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for (index, present) in presents_destination.iter().enumerate() {
        presents_origin[*present as usize - 1] = index as u8 + 1;
    }

    presents_origin.iter().for_each(|x| print!("{} ", x));
}