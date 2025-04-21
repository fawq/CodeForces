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

fn is_caps_lock(text: &str) -> bool {
    text.chars().skip(1).all(|c| c.is_ascii_uppercase())
}

fn main() {
    let CAPS_MAP: HashMap<char, char> = HashMap::from([
        ('A', 'a'),
        ('B', 'b'),
        ('C', 'c'),
        ('D', 'd'),
        ('E', 'e'),
        ('F', 'f'),
        ('G', 'g'),
        ('H', 'h'),
        ('I', 'i'),
        ('J', 'j'),
        ('K', 'k'),
        ('L', 'l'),
        ('M', 'm'),
        ('N', 'n'),
        ('O', 'o'),
        ('P', 'p'),
        ('Q', 'q'),
        ('R', 'r'),
        ('S', 's'),
        ('T', 't'),
        ('U', 'u'),
        ('V', 'v'),
        ('W', 'w'),
        ('X', 'x'),
        ('Y', 'y'),
        ('Z', 'z'),
        ('a', 'A'),
        ('b', 'B'),
        ('c', 'C'),
        ('d', 'D'),
        ('e', 'E'),
        ('f', 'F'),
        ('g', 'G'),
        ('h', 'H'),
        ('i', 'I'),
        ('j', 'J'),
        ('k', 'K'),
        ('l', 'L'),
        ('m', 'M'),
        ('n', 'N'),
        ('o', 'O'),
        ('p', 'P'),
        ('q', 'Q'),
        ('r', 'R'),
        ('s', 'S'),
        ('t', 'T'),
        ('u', 'U'),
        ('v', 'V'),
        ('w', 'W'),
        ('x', 'X'),
        ('y', 'Y'),
        ('z', 'Z'),
    ]);
    let text: String = read();

    if is_caps_lock(&text) {
        println!("{}", text.chars().map(|c| CAPS_MAP[&c]).collect::<String>());
    } else {
        println!("{}", text);
    }
}