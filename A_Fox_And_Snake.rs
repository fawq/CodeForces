use std::iter;

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
    let (n, m) = read_tuple::<usize, usize>();

    for index in 0..n {
        if index % 2 == 0 {
            println!("{}", iter::repeat('#').take(m).collect::<String>());
        } else if index % 4 == 1 {
            println!("{}{}", iter::repeat('.').take(m-1).collect::<String>(), '#');
        } else {
            println!("{}{}", '#', iter::repeat('.').take(m-1).collect::<String>());
        }
    }
}