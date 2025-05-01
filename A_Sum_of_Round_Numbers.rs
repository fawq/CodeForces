fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn print_summands(mut number: u32) {
    let mut summands: Vec<u32> = Vec::new();
    let mut scalar: u32 = 10;

    while number > 0 {
        let digit_with_zeros = number % scalar;
        number -= digit_with_zeros;

        if digit_with_zeros > 0 {
            summands.push(digit_with_zeros);
        }

        scalar *= 10;
    }

    println!("{}", summands.len());
    println!("{}", summands.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn main() {
    let number_of_questions: u16 = read();

    for _ in 0..number_of_questions {
        let mut number: u32 = read();
        print_summands(number);
    }    
}