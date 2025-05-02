fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_tuple<T: std::str::FromStr, S: std::str::FromStr, R: std::str::FromStr>() -> (T, S, R) {
    let line = read::<String>();
    let mut iter = line.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().ok().unwrap();
    let s = iter.next().unwrap().parse().ok().unwrap();
    let r = iter.next().unwrap().parse().ok().unwrap();
    (t, s, r)
}

fn print_if_sum_of_others(a: u8, b: u8, c: u8) {
    if a + b == c || a + c == b || b + c == a {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let number_of_test_cases: u16 = read();

    for _ in 0..number_of_test_cases {
        let (a, b, c): (u8, u8, u8) = read_tuple();
        print_if_sum_of_others(a, b, c);
    }
}