use std::io::{stdin, stdout, Write};
use wordcount::count_words;

fn main() {
    print!("Enter text: ");
    let s = read_user_input();
    println!("Number of words: {}", count_words(&s, ))
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
