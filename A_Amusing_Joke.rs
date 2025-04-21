use std::collections::HashMap;

fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn count_occurrences<T: Eq + std::hash::Hash, I: IntoIterator<Item = T>>(line: I) -> HashMap<T, usize>
{
    line.into_iter().fold(HashMap::new(), |mut counter, item| {
        *counter.entry(item).or_insert(0) += 1;
        counter
    })
}

fn main() {
    let first_word = read::<String>();
    let second_word = read::<String>();
    let third_word = read::<String>();

    if count_occurrences((first_word + &second_word).chars()) == count_occurrences(third_word.chars()) {
        println!("YES");
    } else {
        println!("NO");
    }
}