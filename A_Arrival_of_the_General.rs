use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number: u8 = read_line().trim().parse().unwrap();
    let soldiers: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut max_height: u8 = 0;
    let mut max_index: usize = 0;
    let mut min_height: u8 = 101;
    let mut min_index: usize = 0;

    for (index, height) in soldiers.iter().enumerate() {
        if *height > max_height {
            max_height = *height;
            max_index = index;
        }
        if *height <= min_height {
            min_height = *height;
            min_index = index;
        }
    }

    if min_index > max_index {
        println!("{}", max_index + number as usize - min_index - 1);
    } else {
        println!("{}", max_index + number as usize - min_index - 2);
    }
}