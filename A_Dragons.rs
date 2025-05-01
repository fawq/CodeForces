fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_tuple<T: std::str::FromStr, S: std::str::FromStr>() -> (T, S) {
    let line = read::<String>();
    let mut iter = line.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().ok().unwrap();
    let s = iter.next().unwrap().parse().ok().unwrap();
    (t, s)
}

fn main() {
    let (mut strength, number_of_duels) = read_tuple::<u32, u16>();
    let mut duels: Vec<(u32, u32)> = Vec::new();

    for _ in 0..number_of_duels {
        let (boss_strength, additional_strength) = read_tuple::<u32, u32>();
        duels.push((boss_strength, additional_strength));
    }

    duels.sort_by_key(|duel| duel.0);

    let mut loop_break: bool = false;
    for duel in duels {
        if strength <= duel.0 {
            println!("NO");
            loop_break = true;
            break;
        }
        strength += duel.1;
    }
    if !loop_break {
        println!("YES");
    }
}