use std::collections::HashMap;

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

fn read_tuple<T: std::str::FromStr, S: std::str::FromStr>() -> (T, S) {
    let line = read::<String>();
    let mut iter = line.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().ok().unwrap();
    let s = iter.next().unwrap().parse().ok().unwrap();
    (t, s)
}

fn count_occurrences<T: Eq + std::hash::Hash, I: IntoIterator<Item = T>>(line: I) -> HashMap<T, usize>
{
    line.into_iter().fold(HashMap::new(), |mut counter, item| {
        *counter.entry(item).or_insert(0) += 1;
        counter
    })
}

fn main() {
    let (n, m) = read_tuple::<u8, u128>();
    println!("{} {}", n, m);
    let mut v = read_vec::<u64>();
    v.sort();
    count_occurrences(v);
}
