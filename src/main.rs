//! Numeronym生成プログラム
//! Rustの練習！

use std::io::{self, Write};

fn main() {
    print!("Type Something\n> ");
    // 全てのバッファを画面に表示する
    io::stdout().flush().unwrap();

    // 標準入力
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("For some reason it failed to read.");
    let input_line = input_line.trim();

    // Numeronym変換関数へ通す
    println!("{}", a24z(input_line));
}

/// 与えられた文字列をNumeronymに変換します。<br>
/// 文字は空白で区切ります。
fn a24z(input: &str) -> String {
    input
        .split_whitespace() // 空白文字で分割
        .map(a24z_each_word) // 単語単位でNumeronymに変換
        .collect::<Vec<String>>() // IteratorをVecに変換
        .join(" ") // 空白文字で再結合
}

/// 空白を含まない、単語単位でNumeronymに変換します。
fn a24z_each_word(word: &str) -> String {
    // 3文字以下はNumeronymにできないのでそのまま
    if word.chars().count() <= 3 {
        word.to_string()
    } else {
        format!(
            "{}{}{}",                     // 以下を全ての結合
            word.chars().next().unwrap(), // 最初の文字はそのまま
            word.chars().count() - 2,     // 間に挟まれる文字数
            word.chars().last().unwrap()  // 最後の文字はそのまま
        )
    }
}
