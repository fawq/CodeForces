use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let n: u8 = read_line().trim().parse().unwrap();
    let mut hate_loves: Vec<String> = Vec::new();

    for index in 0..n {
        if index % 2 == 0 {
            hate_loves.push("hate".to_string());
        } else {
            hate_loves.push("love".to_string());
        }
    }

    println!("I {} it", hate_loves.join(" that I "));
}