use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let a: u8 = read_line().trim().parse().unwrap();
    let b: u8 = read_line().trim().parse().unwrap();
    let c: u8 = read_line().trim().parse().unwrap();

    let result_1: u16 = a as u16 + b as u16 + c as u16;
    let result_2: u16 = a as u16 + b as u16 * c as u16;
    let result_3: u16 = a as u16 * b as u16 + c as u16;
    let result_4: u16 = a as u16 * b as u16 * c as u16;
    let result_5: u16 = (a as u16 + b as u16) * c as u16;
    let result_6: u16 = a as u16 * (b as u16 + c as u16);

    let results: Vec<u16> = vec![result_1, result_2, result_3, result_4, result_5, result_6];

    println!("{}", results.iter().max().unwrap());
}