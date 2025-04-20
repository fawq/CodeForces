use std::io::BufRead;
use std::collections::HashMap;

fn read<T: std::str::FromStr>() -> T {
    let mut stdin = std::io::stdin().lock();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_ascii_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let number_of_lines: u32 = read();
    let mut occurences: HashMap<String, u32> = HashMap::new();

    for _ in 0..number_of_lines {
        let name: String = read();

        if occurences.contains_key(&name) {
            println!("{}{}", &name, occurences.get(&name).unwrap());
            *occurences.get_mut(&name).unwrap() += 1;
        } else {
            println!("OK");
            occurences.insert(name, 1);
        }
    }
}