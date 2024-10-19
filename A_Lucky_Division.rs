use std::io::BufRead;

const LUCKY_NUMBERS: [u16; 14] = [4, 7, 44, 47, 74, 77, 444, 447, 474, 477, 744, 747, 774, 777];

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number: u16 = read_line().trim().parse().unwrap();

    let mut is_break = false;
    for lucky_number in LUCKY_NUMBERS {
        if number % lucky_number == 0 {
            println!("YES");
            is_break = true;
            break;
        }
    }
    if !is_break {
        println!("NO");
    }
}