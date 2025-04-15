use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let n: u8 = read_line().trim().parse().unwrap();
    let mut host_colors: Vec<u8> = Vec::new();
    let mut guest_colors: Vec<u8> = Vec::new();

    for _ in 0..n {
        let (host_color, guest_color): (u8, u8) = {
            let line = read_line();
            let mut parts = line.split_ascii_whitespace();
            (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
        };

        host_colors.push(host_color);
        guest_colors.push(guest_color);
    }

    let mut host_colors_counter: HashMap<u8, u8> = HashMap::new();
    let mut guest_colors_counter: HashMap<u8, u8> = HashMap::new();

    for color in host_colors {
        *host_colors_counter.entry(color).or_insert(0) += 1;
    }

    for color in guest_colors {
        *guest_colors_counter.entry(color).or_insert(0) += 1;
    }

    let mut sum: u16 = 0;
    for color in host_colors_counter {
        let host_color_count: u8 = color.1;
        let guest_color_count: u8 = *guest_colors_counter.get(&color.0).unwrap_or(&0);
        sum += host_color_count as u16 * guest_color_count as u16;
    }

    println!("{}", sum);
}