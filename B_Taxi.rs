use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let groups_count: u32 = read_line().trim().parse().unwrap();
    let groups: Vec<u32> = read_line()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut groups_counter: HashMap<u32, u32> = HashMap::new();
    
    for group in groups{
        groups_counter.entry(group).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut groups_1: u32 = *groups_counter.get(&1).unwrap_or(&0);
    let mut groups_2: u32 = *groups_counter.get(&2).unwrap_or(&0);
    let mut groups_3: u32 = *groups_counter.get(&3).unwrap_or(&0);
    let mut groups_4: u32 = *groups_counter.get(&4).unwrap_or(&0);
    let mut cars: u32 = 0;
    cars += groups_4;

    cars += groups_3;
    if groups_1 > groups_3{
        groups_1 -= groups_3;
    }
    else {
        groups_1 = 0;
    }

    cars += groups_2 / 2;
    if groups_2 % 2 == 1{
        cars += 1;
        if groups_1 > 1{
            groups_1 -= 2;
        }
        else if groups_1 == 1{
            groups_1 = 0;
        }
    }

    cars += groups_1 / 4;
    if groups_1 % 4 != 0 {
        cars += 1;
    }

    println!("{}", cars);
}