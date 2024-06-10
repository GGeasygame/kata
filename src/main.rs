use std::{env, fs};
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

use wordcount::{calculate_average_characters_of_words, count_unique_words, count_words};


const STOPWORDS_FILE_PATH: &str = "./stopwords.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_text = get_input_text(&args.get(1));

    let stopwords: HashSet<_> = read_stopwords_txt();

    println!("Number of words: {}, unique: {}; average word length: {:.2} characters",
             count_words(&input_text, &stopwords),
             count_unique_words(&input_text, &stopwords),
             calculate_average_characters_of_words(&input_text, &stopwords))
}

fn get_input_text(text_file_path: &Option<&String>) -> String {
    match text_file_path {
        Some(path) => read_text_file(path),
        None => {
            print!("Enter text: ");
            read_user_input()
        }
    }
}

fn read_text_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}

fn read_stopwords_txt() -> HashSet<String> {
    read_text_file(STOPWORDS_FILE_PATH).split("\n").into_iter().map(|s| s.to_string()).collect()
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
    s
}
