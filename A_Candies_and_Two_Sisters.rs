use std::io::BufRead;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() -> () {
    let number_of_tests: u16 = read();

    for _ in 0..number_of_tests {
        let number_of_candies: u32 = read();

        if number_of_candies % 2 == 0 {
            println!("{}", number_of_candies / 2 - 1);
        } else {
            println!("{}", number_of_candies / 2);
        }
    }
}