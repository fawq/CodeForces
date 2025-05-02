fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn check_if_prime(number: u64) -> bool {
    if number == 1 {
        return false;
    }
    
    if number == 2 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn check_if_square(number: u64) -> bool {
    let sqrt = (number as f64).sqrt() as u64;
    sqrt * sqrt == number
}

fn print_if_t_prime(number: u64) {
    if check_if_square(number) && check_if_prime((number as f64).sqrt() as u64) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    _ = read::<String>();
    let numbers: Vec<u64> = read_vec();

    for number in numbers {
        print_if_t_prime(number);
    }
}