use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let instructions: String = read_line().trim().to_string();
    
    let mut is_break: bool = false;
    for instruction in instructions.chars() {
        if "HQ9" .contains(instruction) {
            println!("YES");
            is_break = true;
            break;
        }
    }
    if !is_break {
        println!("NO");
    }
}