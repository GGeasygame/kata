use std::fs;
use std::io::{Read, stdin, stdout, Write};

use wordcount::count_words;

fn main() {
    let stopwords = read_stopwords_txt();
    print!("Enter text: ");
    let s = read_user_input();
    println!("Number of words: {}", count_words(&s, stopwords.iter().map(|s| s.as_str()).collect()))
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
