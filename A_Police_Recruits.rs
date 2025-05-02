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

fn main() {
    _ = read::<String>();
    let events: Vec<i32> = read_vec();

    let mut event_sum: i32 = 0;
    let mut untreated_crimes: u32 = 0;
    for event in events {
        if event > 0 && event_sum < 0 {
            event_sum = 0;
        }
        event_sum += event;
        
        if event_sum < 0 {
            untreated_crimes += 1;
        }
    }

    println!("{}", untreated_crimes);
}