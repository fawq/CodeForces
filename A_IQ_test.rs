use std::io::BufRead;

fn read_line() -> String {
    let mut stdin = std::io::stdin().lock();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    line
}

fn get_number(n: u8) -> u8 {
    let mut even_number_indexes: Vec<u8> = Vec::new();
    let mut odd_number_indexes: Vec<u8> = Vec::new();

    let numbers: Vec<u8> = read_line()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for (index, number) in numbers.iter().enumerate() {
        if number % 2 == 0 {
            even_number_indexes.push(index as u8 + 1);
        } else {
            odd_number_indexes.push(index as u8 + 1);
        }
    }

    if even_number_indexes.len() == 1 {
        even_number_indexes[0]
    } else if odd_number_indexes.len() == 1 {
        odd_number_indexes[0]
    } else {
        0
    }
}

fn main() {
    let n: u8 = read_line().trim().parse().unwrap();
    let result = get_number(n);
    println!("{}", result);
}