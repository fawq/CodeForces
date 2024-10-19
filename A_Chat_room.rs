use std::io::BufRead;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut read_line = || {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        line
    };

    let word: String = read_line().trim().to_string();
    let hello_word: String = "hello".to_string();
    let hello_word_chars: Vec<char> = hello_word.chars().collect();
    let mut index: usize = 0;
    let mut is_break: bool = false;

    for character in word.chars() {
        if character == hello_word_chars[index] {
            index += 1;
        }
        if index == 5 {
            println!("YES");
            is_break = true;
            break;
        }
    }
    if !is_break {
        println!("NO");
    }
}