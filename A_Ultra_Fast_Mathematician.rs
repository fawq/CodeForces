use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let bits_str: String = read_line().trim().to_string();
    let bits2_str: String = read_line().trim().to_string();

    for (bit1, bit2) in bits_str.chars().zip(bits2_str.chars()) {
        if bit1 != bit2 {
            print!("1");
        } else {
            print!("0");
        }
    }
}