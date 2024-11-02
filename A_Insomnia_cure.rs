use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let k: u8 = read_line().trim().parse().unwrap();
    let l: u8 = read_line().trim().parse().unwrap();
    let m: u8 = read_line().trim().parse().unwrap();
    let n: u8 = read_line().trim().parse().unwrap();
    let d: u32 = read_line().trim().parse().unwrap();
    let mut damaged_dragons: u32 = 0;

    for i in 1..=d {
        if i % k as u32 == 0 || i % l as u32 == 0 || i % m as u32 == 0 || i % n as u32 == 0 {
            damaged_dragons += 1;
        }
    }
    
    println!("{}", damaged_dragons);
}