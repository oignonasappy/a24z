use std::io::{self, Write};

fn main() {
    print!("Type Something\n> ");
    io::stdout().flush().unwrap();

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("For some reason it failed to read.");
    let input_line = input_line.trim();

    println!("{}", a24z(input_line));
}

fn a24z(input: &str) -> String {
    input
        .split_whitespace()
        .map(a24z_each_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn a24z_each_word(word: &str) -> String {
    if word.chars().count() <= 3 {
        word.to_string()
    } else {
        format!(
            "{}{}{}",
            word.chars().next().unwrap(),
            word.chars().count() - 2,
            word.chars().last().unwrap()
        )
    }
}
