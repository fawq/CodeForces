use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number: u8 = read_line().trim().parse().unwrap();
    let soldiers_height: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut max_height: u8 = 0;
    let mut max_index: usize = 0;
    let mut min_height: u8 = 101;
    let mut min_index: usize = 0;

    for (index, soldier_height) in soldiers_height.iter().enumerate() {
        if *soldier_height > max_height {
            max_height = *soldier_height;
            max_index = index;
        }
        if *soldier_height <= min_height {
            min_height = *soldier_height;
            min_index = index;
        }
    }

    if min_index > max_index {
        println!("{}", max_index + number as usize - min_index - 1);
    } else {
        println!("{}", max_index + number as usize - min_index - 2);
    }
}