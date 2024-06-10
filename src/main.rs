use std::{env, fs};
use std::io::{stdin, stdout, Write};

use wordcount::{calculate_average_characters_of_words, count_unique_words, count_words};

fn main() {
    let args: Vec<String> = env::args().collect();

    let s: String;
    if args.len() > 1 {
        let text_file_path = &args[1];
        s = read_text_txt(text_file_path)
    } else {
        print!("Enter text: ");
        s = read_user_input();
    }

    let stopwords: Vec<_> = read_stopwords_txt();

    println!("Number of words: {}, unique: {}; average word length: {:.2} characters",
             count_words(&s, stopwords.clone()),
             count_unique_words(&s, stopwords.clone()),
             calculate_average_characters_of_words(&s, stopwords.clone()))
}

fn read_text_txt(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}

fn read_stopwords_txt() -> Vec<String> {
    match fs::read_to_string("./stopwords.txt") {
        Ok(val) => val.split("\n").into_iter().map(|s| s.to_string()).collect(),
        Err(_) => vec![]
    }
}

fn read_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    {}
    s
}
