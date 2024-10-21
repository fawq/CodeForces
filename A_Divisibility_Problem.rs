use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let number_of_test_cases: u16 = read_line().trim().parse().unwrap();

    for _ in 0..number_of_test_cases {
        let (a, b): (u32, u32) = {
            let line = read_line();
            let mut parts = line.split_ascii_whitespace();
            (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
        };

        println!("{}", if a % b == 0 { format!("0") } else { format!("{}", b - a % b) } );
    }
}