use std::cmp::min;

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

fn main() {
    let (number_of_children, number_of_puzzless) = read_tuple::<usize, usize>();
    let mut puzzles: Vec<u16> = read_vec();
    puzzles.sort();

    let mut min_distance: u16 = 1000;
    for index in 0..number_of_puzzless - number_of_children + 1 {
        min_distance = min(min_distance, puzzles[index + number_of_children - 1] - puzzles[index]);
    }

    println!("{}", min_distance);
}